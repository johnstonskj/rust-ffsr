use ffsr::lexer::Lexer;
use ffsr::reader::datum::{Datum, SChar};
use ffsr::reader::Reader;
use pretty_assertions::assert_eq;

#[test]
fn single_a() {
    let reader = Reader::from(Lexer::from("#\\a "));
    let mut iter = reader.iter();

    let c = iter.next().unwrap(); // not None
    let c = c.unwrap(); // not Err

    assert_eq!(c, Datum::Char(SChar::from('a')));
}

#[test]
fn single_a_eoi() {
    let reader = Reader::from(Lexer::from("#\\a"));
    let mut iter = reader.iter();

    let c = iter.next().unwrap(); // not None
    let c = c.unwrap(); // not Err

    assert_eq!(c, Datum::Char(SChar::from('a')));
}

#[test]
fn single_emoji() {
    let reader = Reader::from(Lexer::from("#\\😀"));
    let mut iter = reader.iter();

    let c = iter.next().unwrap(); // not None
    let c = c.unwrap(); // not Err

    assert_eq!(c, Datum::Char(SChar::from('😀')));
}

#[test]
fn single_emoji_eoi() {
    let reader = Reader::from(Lexer::from("#\\😀"));
    let mut iter = reader.iter();

    let c = iter.next().unwrap(); // not None
    let c = c.unwrap(); // not Err

    assert_eq!(c, Datum::Char(SChar::from('😀')));
}

#[test]
fn single_x() {
    let reader = Reader::from(Lexer::from("#\\x "));
    let mut iter = reader.iter();

    let c = iter.next().unwrap(); // not None
    let c = c.unwrap(); // not Err

    assert_eq!(c, Datum::Char(SChar::from('x')));
}

#[test]
fn single_x_eoi() {
    let reader = Reader::from(Lexer::from("#\\x"));
    let mut iter = reader.iter();

    let c = iter.next().unwrap(); // not None
    let c = c.unwrap(); // not Err

    assert_eq!(c, Datum::Char(SChar::from('x')));
}

#[test]
fn named() {
    let reader = Reader::from(Lexer::from("#\\space "));
    let mut iter = reader.iter();

    let c = iter.next().unwrap(); // not None
    let c = c.unwrap(); // not Err

    assert_eq!(c, Datum::Char(SChar::from(' ')));
}

#[test]
fn named_eoi() {
    let reader = Reader::from(Lexer::from("#\\space"));
    let mut iter = reader.iter();

    let c = iter.next().unwrap(); // not None
    let c = c.unwrap(); // not Err

    assert_eq!(c, Datum::Char(SChar::from(' ')));
}

#[test]
fn unicode_escape() {
    let reader = Reader::from(Lexer::from("#\\x00fb; "));
    let mut iter = reader.iter();

    let c = iter.next().unwrap(); // not None
    let c = c.unwrap(); // not Err

    assert_eq!(c, Datum::Char(SChar::from('û')));
}

#[test]
fn unicode_escape_eoi() {
    let reader = Reader::from(Lexer::from("#\\x00fb;"));
    let mut iter = reader.iter();

    let c = iter.next().unwrap(); // not None
    let c = c.unwrap(); // not Err

    assert_eq!(c, Datum::Char(SChar::from('û')));
}

#[test]
fn unicode() {
    let reader = Reader::from(Lexer::from("#\\û "));
    let mut iter = reader.iter();

    let c = iter.next().unwrap(); // not None
    let c = c.unwrap(); // not Err

    assert_eq!(c, Datum::Char(SChar::from('û')));
}

#[test]
fn unicode_eoi() {
    let reader = Reader::from(Lexer::from("#\\û"));
    let mut iter = reader.iter();

    let c = iter.next().unwrap(); // not None
    let c = c.unwrap(); // not Err

    assert_eq!(c, Datum::Char(SChar::from('û')));
}
