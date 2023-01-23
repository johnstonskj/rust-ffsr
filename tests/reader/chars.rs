use ffsr::lexer::Lexer;
use ffsr::reader::datum::{Datum, SChar};
use ffsr::reader::Reader;
use pretty_assertions::assert_eq;

#[test]
fn test_reader_char_a() {
    let reader = Reader::from(Lexer::from("#\\a"));
    let mut iter = reader.iter();

    let c = iter.next().unwrap(); // not None
    let c = c.unwrap(); // not Err

    assert_eq!(c, Datum::Char(SChar::from('a')));
}

#[test]
fn test_reader_char_x() {
    let reader = Reader::from(Lexer::from("#\\x"));
    let mut iter = reader.iter();

    let c = iter.next().unwrap(); // not None
    let c = c.unwrap(); // not Err

    assert_eq!(c, Datum::Char(SChar::from('x')));
}

#[test]
fn test_reader_char_named() {
    let reader = Reader::from(Lexer::from("#\\space"));
    let mut iter = reader.iter();

    let c = iter.next().unwrap(); // not None
    let c = c.unwrap(); // not Err

    assert_eq!(c, Datum::Char(SChar::from(' ')));
}

#[test]
fn test_reader_char_unicode_escape() {
    let reader = Reader::from(Lexer::from("#\\x00fb;"));
    let mut iter = reader.iter();

    let c = iter.next().unwrap(); // not None
    let c = c.unwrap(); // not Err

    assert_eq!(c, Datum::Char(SChar::from('û')));
}

#[test]
fn test_reader_char_unicode() {
    let reader = Reader::from(Lexer::from("#\\û"));
    let mut iter = reader.iter();

    let c = iter.next().unwrap(); // not None
    let c = c.unwrap(); // not Err

    assert_eq!(c, Datum::Char(SChar::from('û')));
}
