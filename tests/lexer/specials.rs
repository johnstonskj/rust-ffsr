use ffsr::lexer::Lexer;

#[test]
fn test_lexer_incomplete_special() {
    let lexer = Lexer::from("# ()");
    let mut tokens = lexer.tokens();

    let error = tokens.next().unwrap();
    assert!(error.is_err());
    let error = error.err().unwrap();
    assert_eq!(
        error.to_string(),
        format!("Token 'special' not closed, span: 0..1")
    );
    error.print(lexer.source_str());
}
