use proc_macro2::TokenStream;
use regex::Regex;
use syn::{Attribute, Data::Enum, Fields, Lit, Meta, NestedMeta, Variant};

fn get_string_tokens_from_attribute(attribute: &Attribute) -> Vec<String> {
    let mut string_tokens = Vec::new();

    if let Ok(Meta::List(list)) = attribute.parse_meta() {
        for nested_meta in list.nested {
            if let NestedMeta::Lit(Lit::Str(string_token)) = nested_meta {
                string_tokens.push(string_token.value());
            }
        }
    }

    string_tokens
}

fn get_attribute_with_name_from_variant<'a>(
    variant: &'a Variant,
    name: &str,
) -> Option<&'a Attribute> {
    for attr in &variant.attrs {
        if attr.path.is_ident(name) {
            return Some(attr);
        }
    }

    None
}

fn get_string_tokens_with_name_from_variant(variant: &Variant, name: &str) -> Option<Vec<String>> {
    match get_attribute_with_name_from_variant(variant, name) {
        Some(attr) => {
            let patterns = get_string_tokens_from_attribute(attr);
            if patterns.is_empty() {
                None
            } else {
                Some(patterns)
            }
        }
        None => None,
    }
}

fn get_name_from_variant(variant: &Variant) -> String {
    variant.ident.to_string()
}

fn get_number_unnamed_fields_from_variant(variant: &Variant) -> Option<usize> {
    match &variant.fields {
        Fields::Named(_) => None,
        _ => Some(variant.fields.len()),
    }
}

fn regexs_same_num_captures(regexs: &[Regex]) -> Option<usize> {
    let lens: Vec<usize> = regexs
        .iter()
        .map(|regex| regex.captures_len() - 1)
        .collect();

    if lens.windows(2).all(|w| w[0] == w[1]) {
        Some(lens[0])
    } else {
        None
    }
}

#[proc_macro_derive(ErrorEnum, attributes(patterns))]
pub fn derive_from_str_from_patterns(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    // Check if the input is an enum
    match input.data {
        Enum(e) => {
            let enum_name = &input.ident;
            let mut output: TokenStream = TokenStream::new();

            output.extend(quote::quote! {
                fn one_regex_captures(patterns: &Vec<&str>, input: &str) -> Option<Vec<String>> {
                    let set = regex::RegexSet::new(patterns).unwrap();
                    let matches: Vec<_> = set.matches(input).into_iter().collect();

                    if matches.is_empty() {
                        return None;
                    }

                    // Always use the first match. Do not care if multiple match
                    let regex = regex::Regex::new(&patterns[matches[0]]).unwrap();

                    let captures: Vec<String> = regex
                    .captures(input)
                    .unwrap()
                    .iter()
                    .skip(1)
                    .map(|x| x.unwrap().as_str().to_string())
                    .collect();

                    Some(captures)
                }
            });

            output.extend(quote::quote! {
                macro_rules! gen_variant_check {
                    ($enum_name:ident::$variant_name:ident, 0, $input:expr, $($patterns:expr),*) => {
                        if let Some(v) = one_regex_captures(&vec![$($patterns),*], $input) {
                            if let [] = v.as_slice() {
                                return Ok($enum_name::$variant_name);
                            }
                        }
                    };
                    ($enum_name:ident::$variant_name:ident, 1, $input:expr, $($patterns:expr),*) => {
                        if let Some(v) = one_regex_captures(&vec![$($patterns),*], $input) {
                            if let [a] = v.as_slice() {
                                return Ok($enum_name::$variant_name(a.to_string()));
                            }
                        }
                    };
                    ($enum_name:ident::$variant_name:ident, 2, $input:expr, $($patterns:expr),*) => {
                        if let Some(v) = one_regex_captures(&vec![$($patterns),*], $input) {
                            if let [a, b] = v.as_slice() {
                                return Ok($enum_name::$variant_name(a.to_string(), b.to_string()));
                            }
                        }
                    };
                    ($enum_name:ident::$variant_name:ident, 3, $input:expr, $($patterns:expr),*) => {
                        if let Some(v) = one_regex_captures(&vec![$($patterns),*], $input) {
                            if let [a, b, c] = v.as_slice() {
                                return Ok($enum_name::$variant_name(a.to_string(), b.to_string(), c.to_string()));
                            }
                        }
                    };
                    ($enum_name:ident::$variant_name:ident, 4, $input:expr, $($patterns:expr),*) => {
                        if let Some(v) = one_regex_captures(&vec![$($patterns),*], $input) {
                            if let [a, b, c, d] = v.as_slice() {
                                return Ok($enum_name::$variant_name(a.to_string(), b.to_string(), c.to_string(), d.to_string()));
                            }
                        }
                    };
                }
            });

            let mut output_str: String = String::new();

            output_str += &format!(
                "
                impl std::str::FromStr for {enum_name} {{
                    type Err = ();

                    fn from_str(s: &str) -> Result<Self, Self::Err> {{
            "
            );

            for v in &e.variants {
                let variant_name = get_name_from_variant(v);

                let patterns: Vec<String> = get_string_tokens_with_name_from_variant(v, "patterns")
                    .unwrap_or_else(|| {
                        panic!("No Attribute \"patterns\" found!");
                    });
                let patterns_joined: String = patterns
                    .iter()
                    .map(|w| format!("\"{}\"", w))
                    .collect::<Vec<String>>()
                    .join(", ");

                let regexs: Vec<regex::Regex> = patterns
                    .iter()
                    .map(|pattern| {
                        regex::Regex::new(pattern)
                            .unwrap_or_else(|_| panic!("Invalid regex pattern: {}", pattern))
                    })
                    .collect();

                let num_unnamed_fields = get_number_unnamed_fields_from_variant(v)
                    .unwrap_or_else(|| panic!("Enum Variant has to have none or unnamed fields"));

                match regexs_same_num_captures(&regexs) {
                    Some(x) => {
                        if num_unnamed_fields != x {
                            panic!("The number of captures in patterns does not match with the number of fields in Variant {}! {} != {}", variant_name, num_unnamed_fields, x);
                        }
                    }
                    None => {
                        panic!("The number of captures in patterns have to be the same for variant {}!", variant_name);
                    }
                }

                output_str += &format!(
                    "
                    gen_variant_check!(
                        {enum_name}::{variant_name}, 
                        {num_unnamed_fields}, 
                        s, 
                        {patterns_joined}
                    );
                "
                );
            }

            output_str += "
                Err(())
                }}
            ";

            output.extend(output_str.parse::<TokenStream>().unwrap());

            output.into()
        }
        _ => {
            panic!("`HelloWorldDisplay` can only be derived on enums");
        }
    }
}
