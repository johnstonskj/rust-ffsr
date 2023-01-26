macro_rules! assert_complete {
    ($iter:expr) => {
        assert!($iter.next().is_none());
    };
}

macro_rules! inner_success_fn {
    ($test_name:ident, $input:expr => $( ($kind:ident, $expected:expr) ),* ) => {
        paste! {
            #[test]
            fn $test_name() {
                let _guard = crate::init_tracing();

                let lexer = ::ffsr::lexer::Lexer::from($input);
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

                assert_complete!(tokens);
            }
        }
    };
}

macro_rules! success_case {
    ($test_name:ident, $input:expr => $kind:ident) => {
        success_case!($test_name, $input => $kind, $input);
    };
    ($test_name:ident, $input:expr => $kind:ident, $expected:expr) => {
        success_case!($test_name, $input => ($kind, $expected));
    };
    ($test_name:ident, $input:expr => $( ($kind:ident, $expected:expr) ),* ) => {
        inner_success_fn!($test_name, $input => $( ($kind, $expected) ),*);
        paste! {
            inner_success_fn!(
                [<$test_name _with_whitespace>],
                format!(" {} ", $input)
                    => $( ($kind, $expected)  ),*
            );
        }
    };
    (! $test_name:ident, $input:expr => $kind:ident) => {
        success_case!(! $test_name, $input => $kind, $input);
    };
    (! $test_name:ident, $input:expr => $kind:ident, $expected:expr) => {
        success_case!(! $test_name, $input => ($kind, $expected));
    };
    (! $test_name:ident, $input:expr => $( ($kind:ident, $expected:expr) ),* ) => {
        inner_success_fn!($test_name, $input => $( ($kind, $expected) ),*);
    };
}

macro_rules! failure_case {
    ($test_name:ident, $input:expr) => {
        failure_case!($test_name, $input, "");
    };
    ($test_name:ident, $input:expr, $expected:expr) => {
        #[test]
        fn $test_name() {
            let _guard = crate::init_tracing();

            use ffsr::Sourced;
            let lexer = ::ffsr::lexer::Lexer::from($input);
            let mut iter = lexer.tokens();

            match iter.next().unwrap() {
                Err(e) => {
                    e.print(lexer.source_str());
                    if !$expected.is_empty() {
                        ::pretty_assertions::assert_eq!(e.to_string(), $expected.to_string());
                    }
                }
                Ok(v) => {
                    println!("Unexpected success: {:#?}", v);
                    panic!();
                }
            }
        }
    };
}

#[test]
fn empty_input() {
    let _guard = crate::init_tracing();

    let lexer = ::ffsr::lexer::Lexer::from("");
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
