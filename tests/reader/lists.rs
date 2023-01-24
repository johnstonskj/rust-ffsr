use ffsr::lexer::Lexer;
use ffsr::reader::datum::{Datum, SChar, SIdentifier, SList, SString};
use ffsr::reader::Reader;
use pretty_assertions::assert_eq;
use std::str::FromStr;

#[test]
fn empty() {
    let reader = Reader::from(Lexer::from("() "));
    let mut iter = reader.iter();

    let c = iter.next().unwrap(); // not None
    let c = c.unwrap(); // not Err

    assert_eq!(c, Datum::List(SList::default()));
}

#[test]
fn empty_eoi() {
    let reader = Reader::from(Lexer::from("()"));
    let mut iter = reader.iter();

    let c = iter.next().unwrap(); // not None
    let c = c.unwrap(); // not Err

    assert_eq!(c, Datum::List(SList::default()));
}

#[test]
fn identifiers() {
    let reader = Reader::from(Lexer::from("(a b c) "));
    let mut iter = reader.iter();

    let c = iter.next().unwrap(); // not None
    let c = c.unwrap(); // not Err

    assert_eq!(
        c,
        Datum::List(SList::from(vec![
            Datum::Identifier(SIdentifier::from_str("a").unwrap()),
            Datum::Identifier(SIdentifier::from_str("b").unwrap()),
            Datum::Identifier(SIdentifier::from_str("c").unwrap()),
        ]))
    );
}

#[test]
fn identifiers_eoi() {
    let reader = Reader::from(Lexer::from("(a b c)"));
    let mut iter = reader.iter();

    let c = iter.next().unwrap(); // not None
    let c = c.unwrap(); // not Err

    assert_eq!(
        c,
        Datum::List(SList::from(vec![
            Datum::Identifier(SIdentifier::from_str("a").unwrap()),
            Datum::Identifier(SIdentifier::from_str("b").unwrap()),
            Datum::Identifier(SIdentifier::from_str("c").unwrap()),
        ]))
    );
}

#[test]
fn heterogeneous() {
    let reader = Reader::from(Lexer::from("(a #\\b \"c\") "));
    let mut iter = reader.iter();

    let c = iter.next().unwrap(); // not None
    let c = c.unwrap(); // not Err

    assert_eq!(
        c,
        Datum::List(SList::from(vec![
            Datum::Identifier(SIdentifier::from_str("a").unwrap()),
            Datum::Char(SChar::from('b')),
            Datum::String(SString::from_str("c").unwrap()),
        ]))
    );
}

#[test]
fn heterogeneous_eoi() {
    let reader = Reader::from(Lexer::from("(a #\\b \"c\")"));
    let mut iter = reader.iter();

    let c = iter.next().unwrap(); // not None
    let c = c.unwrap(); // not Err

    assert_eq!(
        c,
        Datum::List(SList::from(vec![
            Datum::Identifier(SIdentifier::from_str("a").unwrap()),
            Datum::Char(SChar::from('b')),
            Datum::String(SString::from_str("c").unwrap()),
        ]))
    );
}

#[test]
fn heterogeneous_nested() {
    let reader = Reader::from(Lexer::from("(a () \"c\") "));
    let mut iter = reader.iter();

    let c = iter.next().unwrap(); // not None
    let c = c.unwrap(); // not Err

    assert_eq!(
        c,
        Datum::List(SList::from(vec![
            Datum::Identifier(SIdentifier::from_str("a").unwrap()),
            Datum::List(SList::default()),
            Datum::String(SString::from_str("c").unwrap()),
        ]))
    );
}

#[test]
fn heterogeneous_nested_eoi() {
    let reader = Reader::from(Lexer::from("(a () \"c\")"));
    let mut iter = reader.iter();

    let c = iter.next().unwrap(); // not None
    let c = c.unwrap(); // not Err

    assert_eq!(
        c,
        Datum::List(SList::from(vec![
            Datum::Identifier(SIdentifier::from_str("a").unwrap()),
            Datum::List(SList::default()),
            Datum::String(SString::from_str("c").unwrap()),
        ]))
    );
}
