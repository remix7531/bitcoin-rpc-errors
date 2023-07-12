use crate::{Error, VerifyError};
use proc_macro_magic::EnumError;

#[derive(Debug, PartialEq, EnumError)]
enum Enum1 {
    #[patterns("^Hello (.*) (.*)$", "^(.*) World (.*)!$")]
    Variant1(String, String),
    #[patterns("^two", "regexs$")]
    Variant2,
    #[patterns("^single (.*) regex$")]
    Variant3(String),
}

#[derive(Debug, PartialEq, EnumError)]
enum Enum2 {
    #[patterns("^moin$")]
    Variant1,

    #[patterns("^moin moin (.*)!$")]
    Variant2(String),

    // No patterns
    Generic(String),
}

#[test]
fn test_pos() {
    let enum1: Enum1 = "Hello test1 test2".parse().unwrap();
    assert_eq!(
        enum1,
        Enum1::Variant1(String::from("test1"), String::from("test2"))
    );

    let enum2: Enum1 = "The World is big!".parse().unwrap();
    assert_eq!(
        enum2,
        Enum1::Variant1(String::from("The"), String::from("is big"))
    );

    let enum3: Enum1 = "two".parse().unwrap();
    assert_eq!(enum3, Enum1::Variant2);

    let enum4: Enum1 = "regexs".parse().unwrap();
    assert_eq!(enum4, Enum1::Variant2);

    let enum5: Enum1 = "single lame regex".parse().unwrap();
    assert_eq!(enum5, Enum1::Variant3(String::from("lame")));
}

#[test]
fn test_neg() {
    let res1: Result<Enum1, _> = "hello test1 test2".parse();
    assert_eq!(Err(()), res1);

    let res2: Result<Enum1, _> = "The world is big?".parse();
    assert_eq!(Err(()), res2);

    let res3: Result<Enum1, _> = "bla blu blub".parse();
    assert_eq!(Err(()), res3);
}

#[test]
fn test_generic_pos() {
    let enum1: Enum2 = "moin".parse().unwrap();
    assert_eq!(enum1, Enum2::Variant1);

    let enum2: Enum2 = "moin moin remix!".parse().unwrap();
    assert_eq!(enum2, Enum2::Variant2(String::from("remix")));
}

#[test]
fn test_generic_neg() {
    let enum1: Enum2 = "not a found pattern".parse().unwrap();
    assert_eq!(enum1, Enum2::Generic(String::from("not a found pattern")));

    let enum2: Enum2 = "".parse().unwrap();
    assert_eq!(enum2, Enum2::Generic(String::from("")));
}

// Tests with real erorrs
#[test]
fn from_str() {
    let error_str1 = String::from(
        r#"RPC_VERIFY_ERROR occured: {"code": -25, "message": "Input not found or already spent"}"#,
    );
    let error1: Error = error_str1.parse().unwrap();
    assert_eq!(error1, Error::RPC_VERIFY_ERROR(VerifyError::MissingOrSpend));
    
    let error_str2 = String::from(
        r#"sendrawtransaction RPC error: {"code":-27,"message":"Transaction already in block chain"}"#
    );
    let error2: Error = error_str2.parse().unwrap();
    assert_eq!(error2, Error::RPC_VERIFY_ALREADY_IN_CHAIN);
}
