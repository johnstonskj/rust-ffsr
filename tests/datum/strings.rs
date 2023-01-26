use ffsr::reader::datum::SString;
use pretty_assertions::assert_eq;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Single-valued success cases
// ------------------------------------------------------------------------------------------------

#[test]
fn from_str_with_hex_escape() {
    let test_str = "\\x03B1; is named GREEK SMALL LETTER ALPHA.";
    let s = SString::from_str(test_str);
    assert!(s.is_ok());
    assert_eq!(s.unwrap().as_ref(), "α is named GREEK SMALL LETTER ALPHA.");
}

#[test]
fn from_str_with_newline() {
    let test_str = r#"Here’s text \
       containing just one line"#;
    let s = SString::from_str(test_str);
    assert!(s.is_ok());
    assert_eq!(s.unwrap().as_ref(), "Here’s text containing just one line");
}

#[test]
fn from_str_with_quote() {
    let test_str = "The word \"recursion\" has many meanings.";
    let s = SString::from_str(test_str);
    assert!(s.is_ok());
    assert_eq!(
        s.unwrap().as_ref(),
        "The word \"recursion\" has many meanings."
    );
}

#[test]
fn from_str_with_ascii_escape() {
    let test_str = "\u{00} a \u{07} b \u{08} r \r n \n t \t \" \\ |";
    let s = SString::from_str(test_str);
    assert!(s.is_ok());
    let results: String = s.unwrap().escape_default().collect();
    assert_eq!(
        results.as_str(),
        "\\x0; a \\a b \\b r \\r n \\n t \\t \\\" \\|"
    );
}

#[test]
fn from_str_with_unicode_escape() {
    let test_str = "γϛ";
    let s = SString::from_str(test_str);
    assert!(s.is_ok());
    let results: String = s.unwrap().escape_default().collect();
    assert_eq!(results.as_str(), "\\x3b3;\\x3db;");
}

// ------------------------------------------------------------------------------------------------
// Failure cases
// ------------------------------------------------------------------------------------------------
