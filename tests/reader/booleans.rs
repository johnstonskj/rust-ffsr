use ffsr::lexer::Lexer;
use ffsr::reader::datum::{Datum, SBoolean};
use ffsr::reader::Reader;
use pretty_assertions::assert_eq;

#[test]
fn test_reader_boolean_true() {
    let reader = Reader::from(Lexer::from("#t"));
    let mut iter = reader.iter();

    let c = iter.next().unwrap(); // not None
    let c = c.unwrap(); // not Err

    assert_eq!(c, Datum::Boolean(SBoolean::from(true)));
}

#[test]
fn test_reader_boolean_false() {
    let reader = Reader::from(Lexer::from("#f"));
    let mut iter = reader.iter();

    let c = iter.next().unwrap(); // not None
    let c = c.unwrap(); // not Err

    assert_eq!(c, Datum::Boolean(false.into()));
}
