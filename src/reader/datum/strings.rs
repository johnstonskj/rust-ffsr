/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::{
    error::{invalid_string_input, invalid_unicode_value, Error},
    lexer::token::Span,
};
use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use super::{Datum, DatumValue, SChar, SimpleDatumValue};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SString(String);

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum StringParseState {
    Normal,
    InEscape,
    InHexEscape,
    InWhitespaceEol,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for SString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for SString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.escape_default_string())
    }
}

impl From<SString> for String {
    fn from(s: SString) -> Self {
        s.0
    }
}

impl From<SString> for Datum {
    fn from(v: SString) -> Self {
        Self::String(v)
    }
}

impl AsRef<str> for SString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl FromStr for SString {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: calculate the span end in from_str_in_span?
        // or another way to save scanning the entire string twice
        Self::from_str_in_span(s, Span::new_char_span_from(s))
    }
}

impl DatumValue for SString {}

impl SimpleDatumValue for SString {
    fn from_str_in_span(s: &str, span: Span) -> Result<Self, Error> {
        let mut buffer = String::with_capacity(s.len());
        let mut state = StringParseState::Normal;
        let mut mark: usize = 0;
        for (i, c) in s.char_indices() {
            match (state, c) {
                (StringParseState::Normal, '\\') => {
                    println!("entering escape");
                    state = StringParseState::InEscape;
                }
                (StringParseState::InEscape, 'a') => {
                    buffer.push('\u{07}');
                    state = StringParseState::Normal;
                }
                (StringParseState::InEscape, 'b') => {
                    buffer.push('\u{08}');
                    state = StringParseState::Normal;
                }
                (StringParseState::InEscape, 't') => {
                    buffer.push('\t');
                    state = StringParseState::Normal;
                }
                (StringParseState::InEscape, 'n') => {
                    buffer.push('\n');
                    state = StringParseState::Normal;
                }
                (StringParseState::InEscape, 'r') => {
                    buffer.push('\r');
                    state = StringParseState::Normal;
                }
                (StringParseState::InEscape, '"') => {
                    buffer.push('"');
                    state = StringParseState::Normal;
                }
                (StringParseState::InEscape, '\\') => {
                    buffer.push('\\');
                    state = StringParseState::Normal;
                }
                (StringParseState::InEscape, '|') => {
                    buffer.push('|');
                    state = StringParseState::Normal;
                }
                (StringParseState::InEscape, 'x') => {
                    mark = i;
                    state = StringParseState::InHexEscape;
                }
                (StringParseState::InEscape, ' ' | '\t' | '\r' | '\n') => {
                    state = StringParseState::InWhitespaceEol;
                }
                (StringParseState::InHexEscape, c) if c.is_ascii_hexdigit() => {}
                (StringParseState::InHexEscape, ';') => {
                    let hex_str = &s[mark + 1..i];
                    let hex_val = u32::from_str_radix(hex_str, 16)
                        .map_err(|_| invalid_unicode_value(span))?;
                    // TODO: use SChar::is_valid ?
                    buffer
                        .push(char::from_u32(hex_val).ok_or_else(|| invalid_unicode_value(span))?);
                    state = StringParseState::Normal;
                }
                (StringParseState::InWhitespaceEol, ' ' | '\t' | '\r' | '\n') => {}
                (StringParseState::InWhitespaceEol, c) => {
                    buffer.push(c);
                    state = StringParseState::Normal;
                }
                (StringParseState::Normal, c) => {
                    buffer.push(c);
                }
                _ => {
                    return Err(invalid_string_input(span));
                }
            }
        }
        Ok(SString(buffer))
    }
}

impl SString {
    // TODO: chars() -> SChar..
    // TODO: char_indices() -> SChar..
    pub fn escape_default(&self) -> impl Iterator<Item = char> + '_ {
        self.0.chars().flat_map(|c| SChar::from(c).escape_default())
    }

    pub fn escape_default_string(&self) -> String {
        self.escape_default().collect()
    }

    pub fn escape_unicode(&self) -> impl Iterator<Item = char> + '_ {
        self.0.chars().flat_map(|c| SChar::from(c).escape_unicode())
    }

    pub fn escape_unicode_string(&self) -> String {
        self.escape_unicode().collect()
    }

    pub fn is_valid<S>(s: S) -> bool
    where
        S: AsRef<str>,
    {
        Self::from_str(s.as_ref()).is_ok()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::reader::datum::SString;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    #[test]
    fn test_scheme_string_from_str_hex() {
        let test_str = "\\x03B1; is named GREEK SMALL LETTER ALPHA.";
        let s = SString::from_str(test_str);
        assert!(s.is_ok());
        assert_eq!(s.unwrap().as_ref(), "α is named GREEK SMALL LETTER ALPHA.");
    }

    #[test]
    fn test_scheme_string_from_str_newline() {
        let test_str = r#"Here’s text \
       containing just one line"#;
        let s = SString::from_str(test_str);
        assert!(s.is_ok());
        assert_eq!(s.unwrap().as_ref(), "Here’s text containing just one line");
    }

    #[test]
    fn test_scheme_string_from_str_quote() {
        let test_str = "The word \"recursion\" has many meanings.";
        let s = SString::from_str(test_str);
        assert!(s.is_ok());
        assert_eq!(
            s.unwrap().as_ref(),
            "The word \"recursion\" has many meanings."
        );
    }
}
