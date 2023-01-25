use ffsr::lexer::Lexer;
use ffsr::reader::datum::{Datum, Fixnum, SNumber};
use ffsr::reader::Reader;
use pretty_assertions::assert_eq;

#[test]
fn single_one() {
    let reader = Reader::from(Lexer::from("1 "));
    let mut iter = reader.iter();

    let c = iter.next().unwrap(); // not None
    let c = c.unwrap(); // not Err

    assert_eq!(c, Datum::Number(SNumber::Fixnum(Fixnum::from(1))));
}

#[test]
fn single_one_eoi() {
    let reader = Reader::from(Lexer::from("1"));
    let mut iter = reader.iter();

    let c = iter.next().unwrap(); // not None
    let c = c.unwrap(); // not Err

    assert_eq!(c, Datum::Number(SNumber::Fixnum(Fixnum::from(1))));
}

#[test]
fn integer() {
    let reader = Reader::from(Lexer::from("101 "));
    let mut iter = reader.iter();

    let c = iter.next().unwrap(); // not None
    let c = c.unwrap(); // not Err

    assert_eq!(c, Datum::Number(SNumber::Fixnum(Fixnum::from(101))));
}

#[test]
fn integer_eoi() {
    let reader = Reader::from(Lexer::from("101"));
    let mut iter = reader.iter();

    let c = iter.next().unwrap(); // not None
    let c = c.unwrap(); // not Err

    assert_eq!(c, Datum::Number(SNumber::Fixnum(Fixnum::from(101))));
}
