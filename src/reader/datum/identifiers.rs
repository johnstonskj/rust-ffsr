/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::error::{invalid_escape_string, invalid_identifier_input, invalid_unicode_value, Error};
use crate::lexer::iter::{
    is_dot_subsequent, is_identifier_initial, is_identifier_subsequent, is_sign_subsequent,
};
use crate::lexer::token::Span;
use crate::reader::datum::{Datum, DatumValue, SString, SimpleDatumValue};
use std::fmt::Debug;
use std::{fmt::Display, str::FromStr};
use unicode_categories::UnicodeCategories;

use super::SChar;

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
enum IdentifierParseState {
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
        println!(
            "datum [{}] ident state {:?} => {:?}",
            line!(),
            $current,
            IdentifierParseState::$state,
        );
        $current = IdentifierParseState::$state;
    };
}

macro_rules! save {
    ($character:expr => $buffer:expr) => {
        println!("datum [{}] ident push {:?}", line!(), $character);
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
        let s = if s.starts_with('|') && s.ends_with('|') {
            &s[1..s.len() - 1]
        } else {
            s
        };

        if s.is_empty() {
            eprintln!("Identifier is an empty string");
            return Err(invalid_identifier_input(span));
        }

        let mut requires_escape = false;
        let mut buffer = String::with_capacity(s.len());
        let mut current_state = IdentifierParseState::Start;
        let mut mark: usize = 0;

        for (i, c) in s.char_indices() {
            println!("datum ident ({i}, {c:?}) requires escape: {requires_escape}");

            match (current_state, c) {
                (IdentifierParseState::Start, '+' | '-') => {
                    save_and_change_state!(buffer, c, current_state => InPeculiar);
                }
                (IdentifierParseState::Start, '.') => {
                    save_and_change_state!(buffer, c, current_state => InDotPeculiar);
                }
                (
                    IdentifierParseState::Start
                    | IdentifierParseState::InPeculiar
                    | IdentifierParseState::InDotPeculiar,
                    c,
                ) if c.is_ascii_digit() => {
                    save_and_change_state!(buffer, c, current_state => InNumber);
                }
                (IdentifierParseState::InNumber, c) if c.is_ascii_digit() => {
                    save!(c => buffer);
                }
                (IdentifierParseState::InNumber, c) => {
                    save_and_change_state!(buffer, c, current_state => InRest);
                }
                (IdentifierParseState::Start, c) if is_identifier_initial(c) => {
                    save_and_change_state!(buffer, c, current_state => InRest);
                }
                (IdentifierParseState::InPeculiar, '.') => {
                    save_and_change_state!(buffer, c, current_state => InDotPeculiar);
                }
                (IdentifierParseState::InPeculiar, c) if is_sign_subsequent(c) => {
                    save_and_change_state!(buffer, c, current_state => InRest);
                }
                (IdentifierParseState::InDotPeculiar, c) if is_dot_subsequent(c) => {
                    save_and_change_state!(buffer, c, current_state => InRest);
                }
                (IdentifierParseState::InRest, c) if is_identifier_subsequent(c) => {
                    save!(c => buffer);
                }
                (IdentifierParseState::Start | IdentifierParseState::InRest, c)
                    if c.is_separator() || c.is_control() =>
                {
                    requires_escape = true;
                    save!(c => buffer);
                }
                (IdentifierParseState::Start | IdentifierParseState::InRest, c)
                    if ['(', ')', '[', ']', '{', '}', '"', ',', '\'', '`', ';', '#']
                        .contains(&c) =>
                {
                    requires_escape = true;
                    save!(c => buffer);
                }
                (IdentifierParseState::Start | IdentifierParseState::InRest, '\\') => {
                    requires_escape = true;
                    change_state!(current_state => InEscape);
                }
                (IdentifierParseState::InEscape, 'a') => {
                    save_and_change_state!(buffer, '\u{07}', current_state => InRest);
                }
                (IdentifierParseState::InEscape, 'b') => {
                    save_and_change_state!(buffer, '\u{08}', current_state => InRest);
                }
                (IdentifierParseState::InEscape, 't') => {
                    save_and_change_state!(buffer, '\t', current_state => InRest);
                }
                (IdentifierParseState::InEscape, 'n') => {
                    save_and_change_state!(buffer, '\n', current_state => InRest);
                }
                (IdentifierParseState::InEscape, 'r') => {
                    save_and_change_state!(buffer, '\r', current_state => InRest);
                }
                (IdentifierParseState::InEscape, '"') => {
                    save_and_change_state!(buffer, c, current_state => InRest);
                }
                (IdentifierParseState::InEscape, '\\') => {
                    save_and_change_state!(buffer, c, current_state => InRest);
                }
                (IdentifierParseState::InEscape, '|') => {
                    save_and_change_state!(buffer, c, current_state => InRest);
                }
                (IdentifierParseState::InEscape, 'x') => {
                    mark = i;
                    change_state!(current_state => InHexEscape);
                }
                (IdentifierParseState::InEscape, ' ' | '\t' | '\r' | '\n') => {
                    save_and_change_state!(buffer, c, current_state => InRest);
                }
                (IdentifierParseState::InHexEscape, c) if c.is_ascii_hexdigit() => {}
                (IdentifierParseState::InHexEscape, ';') => {
                    let hex_str = &s[mark + 1..i];
                    let hex_val = u32::from_str_radix(hex_str, 16)
                        .map_err(|_| invalid_unicode_value(span))?;
                    // TODO: use SChar::is_valid ?
                    let c = char::from_u32(hex_val).ok_or_else(|| invalid_unicode_value(span))?;

                    save_and_change_state!(buffer, c, current_state => InRest);
                }
                (s, c) => {
                    eprintln!("Not expecting {c:?} in state {s:?}");
                    return Err(invalid_identifier_input(span));
                }
            }
        }
        if current_state == IdentifierParseState::InNumber {
            eprintln!("identifier cannot be a number");
            Err(invalid_identifier_input(span))
        } else if current_state == IdentifierParseState::InEscape
            || current_state == IdentifierParseState::InHexEscape
        {
            eprintln!("incomplete escape sequence, in {current_state:?}");
            Err(invalid_escape_string(span))
        } else if requires_escape {
            Ok(SIdentifier(format!("|{buffer}|")))
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

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    macro_rules! success_test {
        ($test_name:ident, $input_and_result:expr) => {
            success_test!($test_name, $input_and_result, $input_and_result);
        };
        ($test_name:ident, $input:expr, $result:expr) => {
            #[test]
            fn $test_name() {
                use pretty_assertions::assert_eq;
                use std::str::FromStr;
                let test_str = $input;
                let s = $crate::reader::datum::SIdentifier::from_str(test_str);
                assert!(s.is_ok());
                assert_eq!(s.unwrap().as_ref(), $result);
            }
        };
    }
    macro_rules! failure_test {
        ($test_name:ident, $input:expr) => {
            #[test]
            fn $test_name() {
                use std::str::FromStr;
                let test_str = $input;
                let s = $crate::reader::datum::SIdentifier::from_str(test_str);
                assert!(s.is_err());
            }
        };
    }

    success_test!(from_str_a, "a");

    success_test!(from_str_plus, "+");

    success_test!(from_str_divide, "÷");

    success_test!(from_str_vbar_a, "|a|", "a");

    success_test!(from_str_with_spaces, " a ", "| a |");

    success_test!(from_str_space, " ", "| |");

    success_test!(from_str_with_reserved_chars, "a[0].ba#", "|a[0].ba#|");

    success_test!(
        from_str_with_ascii_escape,
        "hello \\\"scheme\\\" from rust",
        "|hello \"scheme\" from rust|"
    );

    success_test!(
        from_str_with_hex_escape,
        "\\x03B1; is named GREEK SMALL LETTER ALPHA.",
        "|α is named GREEK SMALL LETTER ALPHA.|"
    );

    failure_test!(from_str_empty, "");

    failure_test!(from_str_vbar_empty, "||");

    failure_test!(from_str_incomplete_ascii_escape, "str\\");

    failure_test!(from_str_incomplete_hex_escape, "str\\x20");

    failure_test!(from_str_number, "12");

    failure_test!(from_str_plus_number, "+12");
}
