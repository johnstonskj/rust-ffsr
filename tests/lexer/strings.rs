use ffsr::lexer::Lexer;
use ffsr::Sourced;
use pretty_assertions::assert_eq;
use std::borrow::Cow;

#[test]
fn test_lexer_empty_string() {
    let lexer = Lexer::from("\"\"");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_string());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("\"\""));

    assert!(tokens.next().is_none());
}

#[test]
fn test_lexer_simple_string() {
    let lexer = Lexer::from("\"hello\"");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_string());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("\"hello\""));

    assert!(tokens.next().is_none());
}

#[test]
fn test_lexer_simple_string_with_escape() {
    let lexer = Lexer::from("\"hel\\\"lo\"");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_string());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("\"hel\\\"lo\""));

    assert!(tokens.next().is_none());
}

#[test]
fn test_lexer_simple_string_with_hex_escape() {
    let lexer = Lexer::from("\"hel\\x00fd;lo\"");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_string());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("\"hel\\x00fd;lo\""));

    assert!(tokens.next().is_none());
}

#[test]
fn test_lexer_incomplete_string_eoi() {
    let lexer = Lexer::from("\" #t #f");
    let mut tokens = lexer.tokens();

    let error = tokens.next().unwrap();
    assert!(error.is_err());
    let error = error.err().unwrap();
    assert_eq!(
        error.to_string(),
        format!("Token 'string' not closed, span: 0..6")
    );
    error.print(lexer.source_str());
}

#[test]
fn test_lexer_simple_string_with_bad_hex_escape() {
    let lexer = Lexer::from("\"hel\\x00fdlo\"");
    let mut tokens = lexer.tokens();

    let error = tokens.next().unwrap();
    assert!(error.is_err());
    let error = error.err().unwrap();
    assert_eq!(
        error.to_string(),
        format!("Invalid, or badly formed, string escape; span: 0..10")
    );
    error.print(lexer.source_str());
}

#[test]
fn test_lexer_simple_string_with_bad_mnemonic_escape() {
    let lexer = Lexer::from("\"hel\\zlo\"");
    let mut tokens = lexer.tokens();

    let error = tokens.next().unwrap();
    assert!(error.is_err());
    let error = error.err().unwrap();
    assert_eq!(
        error.to_string(),
        format!("Invalid, or badly formed, string escape; span: 0..5")
    );
    error.print(lexer.source_str());
}
