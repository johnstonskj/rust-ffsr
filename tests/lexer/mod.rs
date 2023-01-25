use ffsr::lexer::Lexer;

macro_rules! success_case {
    ($test_name:ident, $input:expr => $kind:ident) => {
        success_case!($test_name, $input => $kind, $input);
    };
    ($test_name:ident, $input:expr => $kind:ident, $expected:expr) => {
        success_case!($test_name, $input => ($kind, $expected));
    };
    ($test_name:ident, $input:expr => $( ($kind:ident, $expected:expr) ),* ) => {
        paste! {
            #[test]
            fn [<$test_name _with_whitespace>]() {
                let lexer = $crate::lexer::Lexer::from(format!(" {} ", $input));
                let mut tokens = lexer.tokens();
                $(

                    let token = tokens.next().unwrap().expect("tokenizer failed");
                    println!("success result: {:#?}", token);
                    assert!(token.[<is_ $kind>]());
                    ::pretty_assertions::assert_eq!(
                        lexer.token_str(&token),
                        ::std::borrow::Cow::Borrowed($expected)
                    );
                )*

                assert!(tokens.next().is_none());
            }
            #[test]
            fn $test_name() {
                let lexer = $crate::lexer::Lexer::from($input);
                let mut tokens = lexer.tokens();
                $(

                    let token = tokens.next().unwrap().expect("tokenizer failed");
                    println!("success result: {:#?}", token);
                    assert!(token.[<is_ $kind>]());
                    ::pretty_assertions::assert_eq!(
                        lexer.token_str(&token),
                        std::borrow::Cow::Borrowed($expected)
                    );
                )*

                assert!(tokens.next().is_none());
            }
        }
    };
}

macro_rules! failure_case {
    ($test_name:ident, $input:expr) => {
        #[test]
        fn $test_name() {
            use ffsr::Sourced;
            let lexer = $crate::lexer::Lexer::from($input);
            let mut tokens = lexer.tokens();

            let error = tokens.next().unwrap();
            println!("failure result: {:#?}", error);
            assert!(error.is_err());

            let error = error.err().unwrap();
            error.print(lexer.source_str());
        }
    };
    ($test_name:ident, $input:expr, $error:expr) => {
        #[test]
        fn $test_name() {
            use ffsr::Sourced;
            let lexer = $crate::lexer::Lexer::from($input);
            let mut tokens = lexer.tokens();

            let error = tokens.next().unwrap();
            println!("failure result: {:#?}", error);
            assert!(error.is_err());

            let error = error.err().unwrap();
            ::pretty_assertions::assert_eq!(error.to_string(), $error.to_string());
            error.print(lexer.source_str());
        }
    };
}

#[test]
fn test_lexer_empty_string() {
    let lexer = Lexer::from("");
    assert!(lexer.tokens().next().is_none());
}

pub mod booleans;
pub mod chars;
pub mod comments;
pub mod datum;
pub mod directives;
pub mod identifiers;
pub mod lists;
pub mod numbers;
pub mod specials;
pub mod strings;
