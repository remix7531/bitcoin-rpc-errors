use proc_macro2::{TokenStream, Ident};
use regex::Regex;
use syn::{Attribute, Data::Enum, DataEnum, Fields, Type::Path, Lit, Meta, NestedMeta, Variant};
use quote::{quote};

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

// Checks if the enum contains a variant "Generic(String)"
fn contains_generic_variant(data_enum: &DataEnum) -> bool {
    data_enum.variants.iter().any(|variant| is_generic_string_variant(variant))
}

fn is_generic_string_variant(variant: &syn::Variant) -> bool {
    if variant.ident != "Generic" {
        return false;
    }
    
    if let Fields::Unnamed(fields) = &variant.fields {
        if fields.unnamed.len() == 1 {
            if let Path(type_path) = &fields.unnamed[0].ty {
                if let Some(segment) = type_path.path.segments.last() {
                    return segment.ident == "String";
                }
            }
        }
    }

    false
}

fn gen_variant_check(enum_name: &Ident, variant_name: &Ident, num_captures: usize, patterns: &[String]) 
    -> Result<TokenStream, ()>
{
    let mut output = quote! {
        if let Some(v) = vec![#( #patterns ),*]
            .iter()
            .find_map(|&pattern| regex::Regex::new(pattern).ok()?.captures(s))
            .map(|captures| captures.iter().skip(1).map(|capture| capture.unwrap().as_str().to_string()).collect::<Vec<String>>())
    }.to_string();
    output += "{";
        
    output += &match num_captures {
        0 => quote! {
            if let [] = v.as_slice() {
                return Ok(#enum_name::#variant_name);
            }
        }.to_string(),
        1 => quote! {
            if let [a] = v.as_slice() {
                return Ok(#enum_name::#variant_name(a.to_string()));
            }
        }.to_string(),
        2 => quote! {
            if let [a, b] = v.as_slice() {
                return Ok(#enum_name::#variant_name(a.to_string(), b.to_string()));
            }
        }.to_string(),
        3 => quote! {
            if let [a, b, c] = v.as_slice() {
                return Ok(#enum_name::#variant_name(a.to_string(), b.to_string(), c.to_string()));
            }
        }.to_string(),
        4 => quote! {
            if let [a, b, c, d] = v.as_slice() {
                return Ok(#enum_name::#variant_name(a.to_string(), b.to_string(), c.to_string(), d.to_string()));
            }
        }.to_string(),
        _ => return Err(()),
    };
    output += "}";

    Ok(output.parse::<TokenStream>().unwrap())
}


#[proc_macro_derive(EnumError, attributes(patterns))]
pub fn derive_from_str_from_patterns(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    // Check if the input is an enum
    match input.data {
        Enum(e) => {
            let mut output: String = format!("
                impl std::str::FromStr for {} {{
                    type Err = ();
                    
                    fn from_str(s: &str) -> Result<Self, Self::Err> {{
            ", &input.ident);

            for v in e.variants.iter().filter(|variant| !is_generic_string_variant(variant)) {
                let patterns: Vec<String> = get_string_tokens_with_name_from_variant(v, "patterns")
                    .expect("No Attribute \"patterns\" found!");
                let regexs: Vec<regex::Regex> = patterns
                    .iter()
                    .map(|pattern| {
                        regex::Regex::new(pattern)
                            .unwrap_or_else(|_| panic!("Invalid regex pattern: {}", pattern))
                    })
                    .collect();

                let num_unnamed_fields = get_number_unnamed_fields_from_variant(v)
                    .expect("Enum Variant has to have none or unnamed fields");

                if let Some(x) = regexs_same_num_captures(&regexs) {
                    if num_unnamed_fields != x {
                        panic!("The number of captures in patterns does not match with the number of fields in Variant {}! {} != {}", &v.ident, num_unnamed_fields, x);
                    }
                } else {
                    panic!("The number of captures in patterns have to be the same for variant {}!", &v.ident);
                }
                
                output += &gen_variant_check(&input.ident, &v.ident, num_unnamed_fields, &patterns).unwrap().to_string();
            }
            
            if contains_generic_variant(&e) {
                output += &format!("
                    Ok({}::Generic(s.to_string()))
                ", &input.ident);
            } else {
                output += "
                    Err(())
                ";
            }
            output += "}}";

            output.parse::<TokenStream>().unwrap().into()
        }
        _ => {
            panic!("`EnumError` can only be derived on enums");
        }
    }
}
