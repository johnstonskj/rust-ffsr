use ffsr::lexer::Lexer;
use pretty_assertions::assert_eq;
use std::borrow::Cow;

#[test]
fn tbooleans() {
    let lexer = Lexer::from("#t #f#t");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_boolean());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("#t"));

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_boolean());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("#f"));

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_boolean());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("#t"));

    assert!(tokens.next().is_none());
}
