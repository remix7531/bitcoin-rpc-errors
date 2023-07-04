use proc_macro_magic::ErrorEnum;

#[allow(dead_code)]
#[derive(Debug, PartialEq, ErrorEnum)]
enum Test {
    #[patterns("Hello (.*) (.*)", "(.*) World (.*)!")]
    Variant1(String, String),
    #[patterns("two", "regexs")]
    Variant2,
    #[patterns("single (.*) regex")]
    Variant3(String),
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_pos() {
        let enum1: crate::test::Test = "Hello test1 test2".parse().unwrap();
        assert_eq!(
            enum1,
            crate::test::Test::Variant1(String::from("test1"), String::from("test2"))
        );

        let enum2: crate::test::Test = "The World is big!".parse().unwrap();
        assert_eq!(
            enum2,
            crate::test::Test::Variant1(String::from("The"), String::from("is big"))
        );

        let enum3: crate::test::Test = "two".parse().unwrap();
        assert_eq!(enum3, crate::test::Test::Variant2);

        let enum4: crate::test::Test = "regexs".parse().unwrap();
        assert_eq!(enum4, crate::test::Test::Variant2);

        let enum5: crate::test::Test = "single lame regex".parse().unwrap();
        assert_eq!(enum5, crate::test::Test::Variant3(String::from("lame")));
    }

    #[test]
    fn test_neg() {
        let res1: Result<crate::test::Test, _> = "hello test1 test2".parse();
        assert_eq!(Err(()), res1);

        let res2: Result<crate::test::Test, _> = "The world is big?".parse();
        assert_eq!(Err(()), res2);

        let res3: Result<crate::test::Test, _> = "bla blu blub".parse();
        assert_eq!(Err(()), res3);
    }

    #[test]
    fn from_str() {
        let error_str: String = r#"RPC_VERIFY_ERROR occured: {"code": -25, "message": "Input not found or already spent"}"#.to_string();
        let error: crate::RPCErrorCode = error_str.parse().unwrap();

        assert_eq!(
            error,
            crate::RPCErrorCode::RPC_VERIFY_ERROR(crate::VerifyError::MissingOrSpend)
        );
    }
}
