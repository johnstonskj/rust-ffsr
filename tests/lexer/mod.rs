use ffsr::lexer::Lexer;

#[test]
fn test_lexer_empty_string() {
    let lexer = Lexer::from("");
    assert!(lexer.tokens().next().is_none());
}

pub mod booleans;
pub mod chars;
pub mod comments;
pub mod datum;
pub mod directives;
pub mod identifiers;
pub mod lists;
pub mod numbers;
pub mod specials;
pub mod strings;
