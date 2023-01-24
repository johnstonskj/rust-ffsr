use ffsr::{
    lexer::Lexer,
    reader::{datum::Datum, Reader},
};
use pretty_assertions::assert_eq;

#[test]
fn single_a() {
    let reader = Reader::from(Lexer::from("a"));
    let mut iter = reader.iter();

    if let Ok(Datum::Identifier(id)) = iter.next().unwrap() {
        assert_eq!(id.as_str(), "a");
    } else {
        panic!()
    }
}

#[test]
fn plus() {
    let reader = Reader::from(Lexer::from("+"));
    let mut iter = reader.iter();

    if let Ok(Datum::Identifier(id)) = iter.next().unwrap() {
        assert_eq!(id.as_str(), "+");
    } else {
        panic!()
    }
}

#[test]
fn three_dots() {
    let reader = Reader::from(Lexer::from("..."));
    let mut iter = reader.iter();

    if let Ok(Datum::Identifier(id)) = iter.next().unwrap() {
        assert_eq!(id.as_str(), "...");
    } else {
        panic!()
    }
}

#[test]
fn plus_soup_plus() {
    let reader = Reader::from(Lexer::from("+soup+"));
    let mut iter = reader.iter();

    if let Ok(Datum::Identifier(id)) = iter.next().unwrap() {
        assert_eq!(id.as_str(), "+soup+");
    } else {
        panic!()
    }
}

#[test]
fn right_arrow_string() {
    let reader = Reader::from(Lexer::from("->string"));
    let mut iter = reader.iter();

    if let Ok(Datum::Identifier(id)) = iter.next().unwrap() {
        assert_eq!(id.as_str(), "->string");
    } else {
        panic!()
    }
}

#[test]
fn less_than_or_equal_question() {
    let reader = Reader::from(Lexer::from("<=?"));
    let mut iter = reader.iter();

    if let Ok(Datum::Identifier(id)) = iter.next().unwrap() {
        assert_eq!(id.as_str(), "<=?");
    } else {
        panic!()
    }
}

#[test]
fn random_string() {
    let reader = Reader::from(Lexer::from("a34kTMNs"));
    let mut iter = reader.iter();

    if let Ok(Datum::Identifier(id)) = iter.next().unwrap() {
        assert_eq!(id.as_str(), "a34kTMNs");
    } else {
        panic!()
    }
}

#[test]
fn long_example() {
    let reader = Reader::from(Lexer::from("the-word-recursion-has-many-meanings"));
    let mut iter = reader.iter();

    if let Ok(Datum::Identifier(id)) = iter.next().unwrap() {
        assert_eq!(id.as_str(), "the-word-recursion-has-many-meanings");
    } else {
        panic!()
    }
}

#[test]
fn a_greek_letter() {
    let reader = Reader::from(Lexer::from("λ"));
    let mut iter = reader.iter();

    if let Ok(Datum::Identifier(id)) = iter.next().unwrap() {
        assert_eq!(id.as_str(), "λ");
    } else {
        panic!()
    }
}

#[test]
fn vbar_a() {
    let reader = Reader::from(Lexer::from("|a|"));
    let mut iter = reader.iter();

    if let Ok(Datum::Identifier(id)) = iter.next().unwrap() {
        assert_eq!(id.as_str(), "|a|");
    } else {
        panic!()
    }
}

#[test]
fn vbar_spaces() {
    let reader = Reader::from(Lexer::from("|a b\tc|"));
    let mut iter = reader.iter();

    if let Ok(Datum::Identifier(id)) = iter.next().unwrap() {
        assert_eq!(id.as_str(), "|a b\tc|");
    } else {
        panic!()
    }
}

#[test]
fn vbar_empty_error() {
    let reader = Reader::from(Lexer::from("||"));
    let mut iter = reader.iter();

    assert!(matches!(iter.next(), Some(Err(_))));
}

#[test]
fn single_emoji() {
    let reader = Reader::from(Lexer::from("☺️️"));
    let mut iter = reader.iter();

    if let Ok(Datum::Identifier(id)) = iter.next().unwrap() {
        assert_eq!(id.as_str(), "☺️️");
    } else {
        panic!()
    }
}

#[test]
fn single_emoji_eoi() {
    let reader = Reader::from(Lexer::from("☺️️"));
    let mut iter = reader.iter();

    if let Ok(Datum::Identifier(id)) = iter.next().unwrap() {
        assert_eq!(id.as_str(), "☺️");
    } else {
        panic!()
    }
}

#[test]
fn fancy() {
    let reader = Reader::from(Lexer::from("〜foo〜"));
    let mut iter = reader.iter();

    if let Ok(Datum::Identifier(id)) = iter.next().unwrap() {
        assert_eq!(id.as_str(), "〜foo〜");
    } else {
        panic!()
    }
}

#[test]
fn at_here() {
    let reader = Reader::from(Lexer::from("@here"));
    let mut iter = reader.iter();

    if let Ok(Datum::Identifier(id)) = iter.next().unwrap() {
        assert_eq!(id.as_str(), "@here");
    } else {
        panic!()
    }
}

#[test]
fn one_world() {
    let reader = Reader::from(Lexer::from("1world"));
    let mut iter = reader.iter();

    if let Ok(Datum::Identifier(id)) = iter.next().unwrap() {
        assert_eq!(id.as_str(), "1world");
    } else {
        panic!()
    }
}

#[test]
fn line_break() {
    let reader = Reader::from(Lexer::from("|hel\nlo|"));
    let mut iter = reader.iter();

    if let Ok(Datum::Identifier(id)) = iter.next().unwrap() {
        assert_eq!(id.as_str(), "|hel\nlo|");
    } else {
        panic!()
    }
}
