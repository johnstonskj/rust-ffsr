use ffsr::lexer::Lexer;
use ffsr::reader::datum::Datum;
use ffsr::reader::Reader;
use pretty_assertions::assert_eq;

#[test]
fn empty() {
    let reader = Reader::from(Lexer::from("\"\" "));
    let mut iter = reader.iter();

    if let Ok(Datum::String(id)) = iter.next().unwrap() {
        assert_eq!(id.as_str(), "");
    } else {
        panic!()
    }
}

#[test]
fn empty_eoi() {
    let reader = Reader::from(Lexer::from("\"\""));
    let mut iter = reader.iter();

    if let Ok(Datum::String(id)) = iter.next().unwrap() {
        assert_eq!(id.as_str(), "");
    } else {
        panic!()
    }
}

#[test]
fn simple() {
    let reader = Reader::from(Lexer::from("\"hello\" "));
    let mut iter = reader.iter();

    if let Ok(Datum::String(id)) = iter.next().unwrap() {
        assert_eq!(id.as_str(), "hello");
    } else {
        panic!()
    }
}

#[test]
fn simple_eoi() {
    let reader = Reader::from(Lexer::from("\"hello\""));
    let mut iter = reader.iter();

    if let Ok(Datum::String(id)) = iter.next().unwrap() {
        assert_eq!(id.as_str(), "hello");
    } else {
        panic!()
    }
}

#[test]
fn simple_with_escape() {
    let reader = Reader::from(Lexer::from("\"hel\\\"lo\" "));
    let mut iter = reader.iter();

    if let Ok(Datum::String(id)) = iter.next().unwrap() {
        assert_eq!(id.as_str(), "hel\"lo");
    } else {
        panic!()
    }
}

#[test]
fn simple_with_escape_eoi() {
    let reader = Reader::from(Lexer::from("\"hel\\\"lo\""));
    let mut iter = reader.iter();

    if let Ok(Datum::String(id)) = iter.next().unwrap() {
        assert_eq!(id.as_str(), "hel\"lo");
    } else {
        panic!()
    }
}

#[test]
fn simple_with_hex_escape() {
    let reader = Reader::from(Lexer::from("\"hel\\x00fd;lo\" "));
    let mut iter = reader.iter();

    if let Ok(Datum::String(id)) = iter.next().unwrap() {
        assert_eq!(id.as_str(), "helýlo");
    } else {
        panic!()
    }
}

#[test]
fn simple_with_hex_escape_eoi() {
    let reader = Reader::from(Lexer::from("\"hel\\x00fd;lo\""));
    let mut iter = reader.iter();

    if let Ok(Datum::String(id)) = iter.next().unwrap() {
        assert_eq!(id.as_str(), "helýlo");
    } else {
        panic!()
    }
}

#[test]
fn incomplete_string_eoi() {
    let reader = Reader::from(Lexer::from("\" #t #f"));
    let mut iter = reader.iter();

    assert!(iter.next().unwrap().is_err());
}

#[test]
fn with_bad_hex_escape() {
    let reader = Reader::from(Lexer::from("\"hel\\x00fdlo\""));
    let mut iter = reader.iter();

    assert!(iter.next().unwrap().is_err());
}

#[test]
fn with_bad_mnemonic_escape() {
    let reader = Reader::from(Lexer::from("\"hel\\zlo\""));
    let mut iter = reader.iter();

    assert!(iter.next().unwrap().is_err());
}
