use ffsr::lexer::Lexer;
use pretty_assertions::assert_eq;
use std::borrow::Cow;

#[test]
fn test_lexer_datum_comment() {
    let lexer = Lexer::from("#;");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_datum_comment());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("#;"));
}

#[test]
fn test_lexer_datum_assign() {
    let lexer = Lexer::from("#21=");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_datum_assignment());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("#21="));
}

#[test]
fn test_lexer_datum_ref() {
    let lexer = Lexer::from("#21#");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_datum_reference());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("#21#"));
}
