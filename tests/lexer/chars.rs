use ffsr::lexer::Lexer;
use pretty_assertions::assert_eq;
use std::borrow::Cow;

#[test]
fn single_a() {
    let lexer = Lexer::from("#\\a");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_character());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("#\\a"));

    assert!(tokens.next().is_none());
}

#[test]
fn named() {
    let lexer = Lexer::from("#\\space");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_character());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("#\\space"));

    assert!(tokens.next().is_none());
}

#[test]
fn escaped_unicode() {
    let lexer = Lexer::from("#\\x00fb;");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_character());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("#\\x00fb;"));

    assert!(tokens.next().is_none());
}
