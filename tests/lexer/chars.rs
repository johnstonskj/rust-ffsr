use ffsr::lexer::Lexer;
use pretty_assertions::assert_eq;
use std::borrow::Cow;

#[test]
fn test_lexer_single_char_a() {
    let lexer = Lexer::from("#\\a");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_character());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("#\\a"));

    assert!(tokens.next().is_none());
}

#[test]
fn test_lexer_single_char_named() {
    let lexer = Lexer::from("#\\space");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_character());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("#\\space"));

    assert!(tokens.next().is_none());
}

#[test]
fn test_lexer_single_char_unicode() {
    let lexer = Lexer::from("#\\x00fb;");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_character());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("#\\x00fb;"));

    assert!(tokens.next().is_none());
}
