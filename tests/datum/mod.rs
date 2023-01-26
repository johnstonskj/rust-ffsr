macro_rules! success_case {
    ($test_name:ident, $input:expr => $kind:ty) => {
        success_case!($test_name, $input => $kind, $input);
    };
    ($test_name:ident, $input:expr => $kind:ty, $expected:expr) => {
        #[test]
        fn $test_name() {
            let _guard = crate::init_tracing();

            use std::str::FromStr;
            let result = <$kind>::from_str($input);

            match result {
                Ok(value) => {
                    ::pretty_assertions::assert_eq!(value.to_string(), $expected.to_string());
                }
                Err(err) => {
                    err.print($input);
                    panic!();
                }
            }
        }
    };
    (! $test_name:ident, $input:expr => $kind:ty, $expected:expr) => {
        #[test]
        fn $test_name() {
            let _guard = crate::init_tracing();

            use std::str::FromStr;
            let result = <$kind>::from_str($input);

            match result {
                Ok(value) => {
                    ::pretty_assertions::assert_eq!(value, $expected);
                }
                Err(err) => {
                    err.print($input);
                    panic!();
                }
            }
        }
    };
}

macro_rules! failure_case {
    ($test_name:ident, $input:expr, $kind:ty) => {
        #[test]
        fn $test_name() {
            let _guard = crate::init_tracing();

            use std::str::FromStr;
            let result = <$kind>::from_str($input);

            match result {
                Err(err) => {
                    err.print($input);
                }
                Ok(value) => {
                    println!("Unexpected success: {:#?}", value);
                    panic!();
                }
            }
        }
    };
}

pub mod booleans;
pub mod chars;
pub mod identifiers;
pub mod numbers;
pub mod strings;
