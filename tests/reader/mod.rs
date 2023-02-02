macro_rules! assert_datum_eq {
    ($datum:expr, $datum_type:ident, $expected:expr) => {
        ::pretty_assertions::assert_eq!(
            $datum,
            ::ffsr::reader::datum::Datum::$datum_type($expected)
        );
    };
}

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
                let reader = ::ffsr::reader::Reader::from(lexer);
                let mut iter = reader.iter();
                $(
                    let datum = iter.next().expect("no next datum");
                    let datum = datum.expect("datum parsing fail");
                    assert_datum_eq!(datum, $kind, $expected);
                )*

                assert_complete!(iter);
            }
        }
    };
}

macro_rules! success_case {
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

            let lexer = ::ffsr::lexer::Lexer::from($input);
            let reader = ::ffsr::reader::Reader::from(lexer);
            let mut iter = reader.iter();

            match iter.next().unwrap() {
                Err(e) => {
                    e.print($input);
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

    let reader = ::ffsr::reader::Reader::from(::ffsr::lexer::Lexer::from(""));
    let mut iter = reader.iter();

    assert_complete!(iter);
}

pub mod booleans;
pub mod chars;
pub mod comments;
pub mod identifiers;
pub mod lists;
pub mod numbers;
pub mod quotes;
pub mod references;
pub mod strings;
pub mod vectors;
