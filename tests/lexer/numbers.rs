use ffsr::lexer::Lexer;
use pretty_assertions::assert_eq;
use std::borrow::Cow;

#[test]
fn single_one() {
    let lexer = Lexer::from(" 1 ");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_numeric());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("1"));

    assert!(tokens.next().is_none());
}

#[test]
fn single_one_eoi() {
    let lexer = Lexer::from("1");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_numeric());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("1"));

    assert!(tokens.next().is_none());
}

#[test]
fn integer() {
    let lexer = Lexer::from(" 101 ");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_numeric());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("101"));

    assert!(tokens.next().is_none());
}

#[test]
fn integer_eoi() {
    let lexer = Lexer::from("101");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_numeric());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("101"));

    assert!(tokens.next().is_none());
}

#[test]
fn exact_integer() {
    let lexer = Lexer::from(" #e101 ");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_numeric_exactness_prefix());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("#e"));

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_numeric());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("101"));

    assert!(tokens.next().is_none());
}

#[test]
fn exact_integer_eoi() {
    let lexer = Lexer::from("#e101");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_numeric_exactness_prefix());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("#e"));

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_numeric());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("101"));

    assert!(tokens.next().is_none());
}

#[test]
fn binary_integer() {
    let lexer = Lexer::from(" #b101 ");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_numeric_radix_prefix());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("#b"));

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_numeric());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("101"));

    assert!(tokens.next().is_none());
}

#[test]
fn binary_integer_eoi() {
    let lexer = Lexer::from("#b101");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_numeric_radix_prefix());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("#b"));

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_numeric());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("101"));

    assert!(tokens.next().is_none());
}
