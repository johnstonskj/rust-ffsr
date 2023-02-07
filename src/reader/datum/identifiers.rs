/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::error::{incomplete_string, invalid_identifier_input, invalid_unicode_value, Error};
use crate::lexer::iter::{
    is_dot_subsequent, is_identifier_initial, is_identifier_subsequent, is_sign_subsequent,
};
use crate::lexer::token::Span;
use crate::reader::datum::SChar;
use crate::reader::datum::{SString, SimpleDatumValue};
use crate::syntax::IDENTIFIER_WRAPPER;
use std::fmt::Debug;
use std::{fmt::Display, str::FromStr};
use tracing::{error, trace};
use unicode_categories::UnicodeCategories;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SIdentifier(String);

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum ParseState {
    Start,
    InNumber,
    InEscape,
    InHexEscape,
    InPeculiar,
    InDotPeculiar,
    InRest,
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

impl Display for SIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for SIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl_datum_value!(Identifier, SIdentifier, String);

impl_simple_datum_from_str!(Identifier, SIdentifier);

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

impl SimpleDatumValue for SIdentifier {
    fn from_str_in_span(s: &str, span: Span) -> Result<Self, Error> {
        let _span = ::tracing::trace_span!("from_str_in_span", s, ?span);
        let _scope = _span.enter();

        let s = if s.starts_with(IDENTIFIER_WRAPPER) && s.ends_with(IDENTIFIER_WRAPPER) {
            &s[1..s.len() - 1]
        } else {
            s
        };

        if s.is_empty() {
            trace!("Identifier is an empty string");
            return Ok(SIdentifier("||".to_string()));
        }

        let mut requires_escape = false;
        let mut buffer = String::with_capacity(s.len());
        let mut current_state = ParseState::Start;
        let mut mark: usize = 0;

        for (i, c) in s.char_indices() {
            trace!("match ({i}, {c:?}) requires escape: {requires_escape}");

            match (current_state, c) {
                (ParseState::Start, '+' | '-') => {
                    save_and_change_state!(buffer, c, current_state => InPeculiar);
                }
                (ParseState::Start, '.') => {
                    save_and_change_state!(buffer, c, current_state => InDotPeculiar);
                }
                (ParseState::Start | ParseState::InPeculiar | ParseState::InDotPeculiar, c)
                    if c.is_ascii_digit() =>
                {
                    save_and_change_state!(buffer, c, current_state => InNumber);
                }
                (ParseState::InNumber, c) if c.is_ascii_digit() => {
                    save!(c => buffer);
                }
                (ParseState::InNumber, c) => {
                    save_and_change_state!(buffer, c, current_state => InRest);
                }
                (ParseState::Start, c) if is_identifier_initial(c) => {
                    save_and_change_state!(buffer, c, current_state => InRest);
                }
                (ParseState::InPeculiar, '.') => {
                    save_and_change_state!(buffer, c, current_state => InDotPeculiar);
                }
                (ParseState::InPeculiar, c) if is_sign_subsequent(c) => {
                    save_and_change_state!(buffer, c, current_state => InRest);
                }
                (ParseState::InDotPeculiar, c) if is_dot_subsequent(c) => {
                    save_and_change_state!(buffer, c, current_state => InRest);
                }
                (ParseState::InRest, c) if is_identifier_subsequent(c) => {
                    save!(c => buffer);
                }
                (ParseState::Start | ParseState::InRest, c)
                    if c.is_separator() || c.is_control() =>
                {
                    requires_escape = true;
                    save!(c => buffer);
                }
                (ParseState::Start | ParseState::InRest, c)
                    if ['(', ')', '[', ']', '{', '}', '"', ',', '\'', '`', ';', '#']
                        .contains(&c) =>
                {
                    requires_escape = true;
                    save!(c => buffer);
                }
                (ParseState::Start | ParseState::InRest, '\\') => {
                    requires_escape = true;
                    change_state!(current_state => InEscape);
                }
                (ParseState::InEscape, 'a') => {
                    save_and_change_state!(buffer, '\u{07}', current_state => InRest);
                }
                (ParseState::InEscape, 'b') => {
                    save_and_change_state!(buffer, '\u{08}', current_state => InRest);
                }
                (ParseState::InEscape, 't') => {
                    save_and_change_state!(buffer, '\t', current_state => InRest);
                }
                (ParseState::InEscape, 'n') => {
                    save_and_change_state!(buffer, '\n', current_state => InRest);
                }
                (ParseState::InEscape, 'r') => {
                    save_and_change_state!(buffer, '\r', current_state => InRest);
                }
                (ParseState::InEscape, '"') => {
                    save_and_change_state!(buffer, c, current_state => InRest);
                }
                (ParseState::InEscape, '\\') => {
                    save_and_change_state!(buffer, c, current_state => InRest);
                }
                (ParseState::InEscape, '|') => {
                    save_and_change_state!(buffer, c, current_state => InRest);
                }
                (ParseState::InEscape, 'x') => {
                    mark = i;
                    change_state!(current_state => InHexEscape);
                }
                (ParseState::InEscape, ' ' | '\t' | '\r' | '\n') => {
                    save_and_change_state!(buffer, c, current_state => InRest);
                }
                (ParseState::InHexEscape, c) if c.is_ascii_hexdigit() => {}
                (ParseState::InHexEscape, ';') => {
                    let hex_str = &s[mark + 1..i];
                    let hex_val = u32::from_str_radix(hex_str, 16)
                        .map_err(|_| invalid_unicode_value::<Self>(span).unwrap_err())?;
                    // TODO: use SChar::is_valid ?
                    let c = char::from_u32(hex_val)
                        .ok_or_else(|| invalid_unicode_value::<Self>(span).unwrap_err())?;

                    save_and_change_state!(buffer, c, current_state => InRest);
                }
                (s, c) => {
                    error!("Not expecting {c:?} in state {s:?}");
                    return invalid_identifier_input(span);
                }
            }
        }
        if current_state == ParseState::InNumber {
            error!("identifier cannot be a number");
            invalid_identifier_input(span)
        } else if current_state == ParseState::InEscape || current_state == ParseState::InHexEscape
        {
            error!("incomplete escape sequence, in {current_state:?}");
            incomplete_string(span)
        } else if requires_escape {
            Ok(SIdentifier(format!(
                "{IDENTIFIER_WRAPPER}{buffer}{IDENTIFIER_WRAPPER}"
            )))
        } else {
            Ok(SIdentifier(buffer))
        }
    }
}

impl SIdentifier {
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
