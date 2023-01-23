use ffsr::lexer::Lexer;
use pretty_assertions::assert_eq;
use std::borrow::Cow;

#[test]
fn test_lexer_simplest_identifier() {
    let lexer = Lexer::from("a");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_identifier());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("a"));

    assert!(tokens.next().is_none());
}

#[test]
fn test_lexer_plus_as_identifier() {
    let lexer = Lexer::from("+");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_identifier());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("+"));

    assert!(tokens.next().is_none());
}

#[test]
fn test_lexer_three_dots_identifier() {
    let lexer = Lexer::from("...");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_identifier());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("..."));

    assert!(tokens.next().is_none());
}

#[test]
fn test_lexer_soup_identifier() {
    let lexer = Lexer::from("+soup+");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_identifier());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("+soup+"));

    assert!(tokens.next().is_none());
}

#[test]
fn test_lexer_to_string_identifier() {
    let lexer = Lexer::from("->string");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_identifier());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("->string"));

    assert!(tokens.next().is_none());
}

#[test]
fn test_lexer_is_lte_identifier() {
    let lexer = Lexer::from("<=?");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_identifier());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("<=?"));

    assert!(tokens.next().is_none());
}

#[test]
fn test_lexer_random_identifier() {
    let lexer = Lexer::from("a34kTMNs");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_identifier());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("a34kTMNs"));

    assert!(tokens.next().is_none());
}

#[test]
fn test_lexer_long_identifier() {
    let lexer = Lexer::from("the-word-recursion-has-many-meanings");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_identifier());
    assert_eq!(
        lexer.token_str(&token),
        Cow::Borrowed("the-word-recursion-has-many-meanings")
    );

    assert!(tokens.next().is_none());
}

#[test]
fn test_lexer_greek_identifier() {
    let lexer = Lexer::from("λ");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_identifier());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("λ"));

    assert!(tokens.next().is_none());
}
