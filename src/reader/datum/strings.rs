/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use super::{Datum, DatumValue, SChar, SimpleDatumValue};
use crate::{
    error::{invalid_string_input, invalid_unicode_value, Error},
    lexer::token::Span,
};
use std::{
    fmt::{Debug, Display},
    str::FromStr,
};
use tracing::trace;

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
enum ParseState {
    Normal,
    InEscape,
    InHexEscape,
    InWhitespaceEol,
}

macro_rules! change_state {
    ($current:expr => $state:ident) => {
        trace!("change state {:?} => {:?}", $current, ParseState::$state,);
        $current = ParseState::$state;
    };
}

macro_rules! save {
    ($character:expr => $buffer:expr) => {
        trace!("save {:?}", $character);
        $buffer.push($character);
    };
}
macro_rules! save_and_change_state {
    ($buffer:expr, $character:expr, $current:expr => $state:ident) => {
        save!($character => $buffer);
        change_state!($current => $state);
    };
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
        let _span = ::tracing::trace_span!("from_str_in_span", s, ?span);
        let _scope = _span.enter();

        let s = if s.starts_with('"') && s.ends_with('"') {
            &s[1..s.len() - 1]
        } else {
            s
        };

        if s.is_empty() {
            return Ok(Default::default());
        }

        let mut buffer = String::with_capacity(s.len());
        let mut current_state = ParseState::Normal;
        let mut mark: usize = 0;

        for (i, c) in s.char_indices() {
            match (current_state, c) {
                (ParseState::Normal, '\\') => {
                    change_state!(current_state => InEscape);
                }
                (ParseState::InEscape, 'a') => {
                    save_and_change_state!(buffer, '\u{07}', current_state => Normal);
                }
                (ParseState::InEscape, 'b') => {
                    save_and_change_state!(buffer, '\u{08}', current_state => Normal);
                }
                (ParseState::InEscape, 't') => {
                    save_and_change_state!(buffer, '\t', current_state => Normal);
                }
                (ParseState::InEscape, 'n') => {
                    save_and_change_state!(buffer, '\n', current_state => Normal);
                }
                (ParseState::InEscape, 'r') => {
                    save_and_change_state!(buffer, '\r', current_state => Normal);
                }
                (ParseState::InEscape, '"') => {
                    save_and_change_state!(buffer, c, current_state => Normal);
                }
                (ParseState::InEscape, '\\') => {
                    save_and_change_state!(buffer, c, current_state => Normal);
                }
                (ParseState::InEscape, '|') => {
                    save_and_change_state!(buffer, c, current_state => Normal);
                }
                (ParseState::InEscape, 'x') => {
                    mark = i;
                    change_state!(current_state => InHexEscape);
                }
                (ParseState::InEscape, ' ' | '\t' | '\r' | '\n') => {
                    change_state!(current_state => InWhitespaceEol);
                }
                (ParseState::InHexEscape, c) if c.is_ascii_hexdigit() => {}
                (ParseState::InHexEscape, ';') => {
                    let hex_str = &s[mark + 1..i];
                    let hex_val = u32::from_str_radix(hex_str, 16)
                        .map_err(|_| invalid_unicode_value::<Self>(span).unwrap_err())?;
                    // TODO: use SChar::is_valid ?
                    let c = char::from_u32(hex_val)
                        .ok_or_else(|| invalid_unicode_value::<Self>(span).unwrap_err())?;
                    save_and_change_state!(buffer, c, current_state => Normal);
                }
                (ParseState::InWhitespaceEol, ' ' | '\t' | '\r' | '\n') => {}
                (ParseState::InWhitespaceEol, c) => {
                    save_and_change_state!(buffer, c, current_state => Normal);
                }
                (ParseState::Normal, c) => {
                    save!(c => buffer);
                }
                _ => {
                    return invalid_string_input(span);
                }
            }
        }
        Ok(SString(buffer))
    }
}

impl SString {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }

    pub fn chars(&self) -> impl Iterator<Item = SChar> + '_ {
        self.0.chars().map(SChar::from)
    }

    pub fn char_indices(&self) -> impl Iterator<Item = (usize, SChar)> + '_ {
        self.0.char_indices().map(|(i, c)| (i, SChar::from(c)))
    }

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
