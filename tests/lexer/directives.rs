use ffsr::lexer::Lexer;
use pretty_assertions::assert_eq;
use std::borrow::Cow;

#[test]
fn test_lexer_directive() {
    let lexer = Lexer::from("#!fold-case");
    let mut tokens = lexer.tokens();

    let token = tokens.next().unwrap().expect("tokenizer failed");
    assert!(token.is_directive());
    assert_eq!(lexer.token_str(&token), Cow::Borrowed("#!fold-case"));
}
