use ffsr::lexer::Lexer;
use pretty_assertions::assert_eq;
use std::borrow::Cow;

#[test]
fn test_lexer_empty_list() {
    let lexer = Lexer::from("(\n)");
    let mut tokens = lexer.tokens();
    assert!(tokens
        .next()
        .unwrap()
        .expect("tokenizer failed")
        .is_open_parenthesis());
    assert!(tokens
        .next()
        .unwrap()
        .expect("tokenizer failed")
        .is_close_parenthesis());
    assert!(tokens.next().is_none());
}

#[test]
fn test_lexer_empty_list_ws() {
    let lexer = Lexer::from("  ( \r\n )\r\t");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_open_parenthesis());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("("));

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_close_parenthesis());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed(")"));

    assert!(tokens.next().is_none());
}
