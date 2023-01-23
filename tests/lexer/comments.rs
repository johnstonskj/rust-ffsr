use ffsr::lexer::Lexer;
use pretty_assertions::assert_eq;
use std::borrow::Cow;

#[test]
fn test_lexer_line_comment() {
    let lexer = Lexer::from("#t ; ignore this: #f\n");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_boolean());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("#t"));

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_line_comment());
    assert_eq!(
        lexer.token_str(&token),
        Cow::Borrowed("; ignore this: #f\n")
    );

    assert!(tokens.next().is_none());
}

#[test]
fn test_lexer_line_comment_at_eoi() {
    let lexer = Lexer::from("#t ; ignore this: #f");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_boolean());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("#t"));

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_line_comment());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("; ignore this: #f"));

    assert!(tokens.next().is_none());
}
