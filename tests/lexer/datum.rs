use ffsr::lexer::Lexer;
use pretty_assertions::assert_eq;
use std::borrow::Cow;

#[test]
fn comment() {
    let lexer = Lexer::from("#;");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_datum_comment());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("#;"));
}

#[test]
fn assignment() {
    let lexer = Lexer::from("#21=");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_datum_assignment());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("#21="));
}

#[test]
fn reference() {
    let lexer = Lexer::from("#21#");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_datum_reference());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("#21#"));
}
