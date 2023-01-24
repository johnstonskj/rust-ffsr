/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::error::{invalid_identifier_input, invalid_unicode_value, Error};
use crate::lexer::iter::{
    is_dot_subsequent, is_identifier_initial, is_identifier_subsequent, is_sign_subsequent,
};
use crate::lexer::token::Span;
use crate::reader::datum::{Datum, DatumValue};
use std::{fmt::Display, str::FromStr};

use super::{SString, SimpleDatumValue};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SIdentifier(String);

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum IdentifierParseState {
    Normal,
    InEscape,
    InHexEscape,
    InWhitespaceEol,
    InPeculiar,
    InDotPeculiar,
    InRest,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for SIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<SIdentifier> for Datum {
    fn from(v: SIdentifier) -> Self {
        Self::Identifier(v)
    }
}

impl From<SIdentifier> for String {
    fn from(v: SIdentifier) -> Self {
        v.0
    }
}

impl TryFrom<SIdentifier> for SString {
    type Error = Error;

    fn try_from(value: SIdentifier) -> Result<Self, Self::Error> {
        SString::from_str(value.as_ref())
    }
}

impl TryFrom<SString> for SIdentifier {
    type Error = Error;

    fn try_from(value: SString) -> Result<Self, Self::Error> {
        Self::from_str(value.as_ref())
    }
}

impl AsRef<str> for SIdentifier {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl FromStr for SIdentifier {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_in_span(s, Span::new_char_span_from(s))
    }
}

impl DatumValue for SIdentifier {}

impl SimpleDatumValue for SIdentifier {
    fn from_str_in_span(s: &str, span: Span) -> Result<Self, Error> {
        if s.is_empty() {
            eprintln!("Identifier is an empty string");
            Err(invalid_identifier_input(span))
        } else if s.starts_with('|') && s.ends_with('|') {
            println!("Identifier is an escaped string-like form");
            let s = &s[1..s.len() - 1];
            if s.is_empty() {
                eprintln!("Identifier is an empty string");
                return Err(invalid_identifier_input(span));
            }
            let mut buffer = String::with_capacity(s.len());
            let mut state = IdentifierParseState::Normal;
            let mut mark: usize = 0;
            for (i, c) in s.char_indices() {
                match (state, c) {
                    (IdentifierParseState::Normal, '\\') => {
                        println!("entering escape");
                        state = IdentifierParseState::InEscape;
                    }
                    (IdentifierParseState::InEscape, 'a') => {
                        buffer.push('\u{07}');
                        state = IdentifierParseState::Normal;
                    }
                    (IdentifierParseState::InEscape, 'b') => {
                        buffer.push('\u{08}');
                        state = IdentifierParseState::Normal;
                    }
                    (IdentifierParseState::InEscape, 't') => {
                        buffer.push('\t');
                        state = IdentifierParseState::Normal;
                    }
                    (IdentifierParseState::InEscape, 'n') => {
                        buffer.push('\n');
                        state = IdentifierParseState::Normal;
                    }
                    (IdentifierParseState::InEscape, 'r') => {
                        buffer.push('\r');
                        state = IdentifierParseState::Normal;
                    }
                    (IdentifierParseState::InEscape, '"') => {
                        buffer.push('"');
                        state = IdentifierParseState::Normal;
                    }
                    (IdentifierParseState::InEscape, '\\') => {
                        buffer.push('\\');
                        state = IdentifierParseState::Normal;
                    }
                    (IdentifierParseState::InEscape, '|') => {
                        buffer.push('|');
                        state = IdentifierParseState::Normal;
                    }
                    (IdentifierParseState::InEscape, 'x') => {
                        mark = i;
                        state = IdentifierParseState::InHexEscape;
                    }
                    (IdentifierParseState::InEscape, ' ' | '\t' | '\r' | '\n') => {
                        state = IdentifierParseState::InWhitespaceEol;
                    }
                    (IdentifierParseState::InHexEscape, c) if c.is_ascii_hexdigit() => {}
                    (IdentifierParseState::InHexEscape, ';') => {
                        let hex_str = &s[mark + 1..i];
                        let hex_val = u32::from_str_radix(hex_str, 16)
                            .map_err(|_| invalid_unicode_value(span))?;
                        // TODO: use SChar::is_valid ?
                        buffer.push(
                            char::from_u32(hex_val).ok_or_else(|| invalid_unicode_value(span))?,
                        );
                        state = IdentifierParseState::Normal;
                    }
                    (IdentifierParseState::InWhitespaceEol, ' ' | '\t' | '\r' | '\n') => {}
                    (IdentifierParseState::InWhitespaceEol, c) => {
                        buffer.push(c);
                        state = IdentifierParseState::Normal;
                    }
                    (IdentifierParseState::Normal, c) => {
                        buffer.push(c);
                    }
                    _ => {
                        return Err(invalid_identifier_input(span));
                    }
                }
            }
            Ok(SIdentifier(format!("|{}|", buffer)))
        } else {
            println!("Identifier is non-escaped form: {:?}", s);
            let mut state = IdentifierParseState::Normal;

            // TODO: handle numeric prefixes!

            for c in s.chars() {
                match (state, c) {
                    (IdentifierParseState::Normal, '+' | '-') => {
                        state = IdentifierParseState::InPeculiar;
                    }
                    (IdentifierParseState::Normal, '.') => {
                        state = IdentifierParseState::InDotPeculiar;
                    }
                    (IdentifierParseState::Normal, c) if is_identifier_initial(c) => {
                        state = IdentifierParseState::InRest;
                    }
                    (IdentifierParseState::InPeculiar, '.') => {
                        state = IdentifierParseState::InDotPeculiar;
                    }
                    (IdentifierParseState::InPeculiar, c) if is_sign_subsequent(c) => {
                        state = IdentifierParseState::InRest;
                    }
                    (IdentifierParseState::InDotPeculiar, c) if is_dot_subsequent(c) => {
                        state = IdentifierParseState::InRest;
                    }
                    (IdentifierParseState::InRest, c) if is_identifier_subsequent(c) => {}
                    (s, c) => {
                        eprintln!("Not expecting {:?} in state {:?}", c, s);
                        return Err(invalid_identifier_input(span));
                    }
                }
            }
            Ok(Self(s.into()))
        }
    }
}

impl SIdentifier {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
