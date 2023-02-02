/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use super::{Datum, DatumValue, SimpleDatumValue};
use crate::error::{invalid_char_input, invalid_unicode_value, unknown_char_name, Error};
use crate::lexer::token::Span;
use std::fmt::Write;
use std::iter::FusedIterator;
use std::{
    fmt::{Debug, Display},
    str::FromStr,
};
use tracing::{error, trace};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SChar(char);

/// See [`std::char::EscapeUnicode`]
#[derive(Clone, Debug)]
pub struct EscapeUnicode {
    c: char,
    state: EscapeUnicodeState,
    hex_digit_idx: usize,
}

/// See [`std::char::EscapeDefault`]
#[derive(Clone, Debug)]
pub struct EscapeDefault {
    state: EscapeDefaultState,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
enum EscapeUnicodeState {
    Done,
    Semicolon,
    Value,
    Type,
    Backslash,
}

#[derive(Clone, Debug)]
enum EscapeDefaultState {
    Done,
    Char(char),
    Backslash(char),
    Unicode(EscapeUnicode),
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for SChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for SChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.escape_default())
    }
}

impl From<char> for SChar {
    fn from(v: char) -> Self {
        Self(v)
    }
}

impl From<SChar> for char {
    fn from(v: SChar) -> Self {
        v.0
    }
}

impl From<SChar> for Datum {
    fn from(v: SChar) -> Self {
        Self::Char(v)
    }
}

impl FromStr for SChar {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_in_span(s, Span::new_char_span_from(s))
    }
}

impl DatumValue for SChar {}

impl SimpleDatumValue for SChar {
    fn from_str_in_span(s: &str, span: Span) -> Result<Self, Error> {
        let _span = ::tracing::trace_span!("from_str_in_span", s, ?span);
        let _scope = _span.enter();

        trace!("SChar? {:?}", s);
        if s.len() > 2 && s.starts_with("#\\") {
            let s = &s[2..];
            if s == "alarm" {
                Ok(SChar::from('\u{07}'))
            } else if s == "backspace" {
                Ok(SChar::from('\u{08}'))
            } else if s == "delete" {
                Ok(SChar::from('\u{7f}'))
            } else if s == "escape" {
                Ok(SChar::from('\u{1b}'))
            } else if s == "newline" {
                Ok(SChar::from('\u{0a}'))
            } else if s == "null" {
                Ok(SChar::from('\u{00}'))
            } else if s == "return" {
                Ok(SChar::from('\u{0d}'))
            } else if s == "space" {
                Ok(SChar::from(' '))
            } else if s == "tab" {
                Ok(SChar::from('\u{09}'))
            } else if s.starts_with('x') && s.ends_with(';') {
                let s = &s[1..s.len() - 1];
                let cp = u32::from_str_radix(s, 16)
                    .map_err(|_| invalid_unicode_value::<Self>(span).unwrap_err())?;
                Ok(SChar::from(char::from_u32(cp).ok_or_else(|| {
                    invalid_unicode_value::<Self>(span).unwrap_err()
                })?))
            } else {
                let mut chars: Vec<char> = s.chars().collect();
                if chars.len() == 1 {
                    Ok(Self::from(chars.remove(0)))
                } else {
                    error!("Unknown char name in {s:?}");
                    unknown_char_name(span, s)
                }
            }
        } else {
            error!("Char does not start with prefix '#\\'");
            invalid_char_input(span)
        }
    }
}

impl SChar {
    /// See [`char::escape_default`]
    pub fn escape_default(&self) -> EscapeDefault {
        let init_state = match self.0 {
            '\u{07}' => EscapeDefaultState::Backslash('a'),
            '\u{08}' => EscapeDefaultState::Backslash('b'),
            '\t' => EscapeDefaultState::Backslash('t'),
            '\r' => EscapeDefaultState::Backslash('r'),
            '\n' => EscapeDefaultState::Backslash('n'),
            '\"' | '\\' | '|' => EscapeDefaultState::Backslash(self.0),
            _ if self.is_non_printing() => EscapeDefaultState::Unicode(self.escape_unicode()),
            _ => EscapeDefaultState::Char(self.0),
        };
        EscapeDefault { state: init_state }
    }

    pub fn escape_default_string(&self) -> String {
        self.escape_default().collect()
    }

    /// See [`char::escape_unicode`]
    pub fn escape_unicode(&self) -> EscapeUnicode {
        let c = self.0 as u32;

        // or-ing 1 ensures that for c==0 the code computes that one
        // digit should be printed and (which is the same) avoids the
        // (31 - 32) underflow
        let msb = 31 - (c | 1).leading_zeros();

        // the index of the most significant hex digit
        let ms_hex_digit = msb / 4;
        EscapeUnicode {
            c: self.0,
            state: EscapeUnicodeState::Backslash,
            hex_digit_idx: ms_hex_digit as usize,
        }
    }

    pub fn escape_unicode_string(&self) -> String {
        self.escape_unicode().collect()
    }

    pub fn is_non_printing(&self) -> bool {
        matches!(self.0,
            '\u{0000}'..='\u{0008}'
            | '\u{000B}'..='\u{001F}'
            | '\u{007F}'..='\u{009F}'
            | '\u{2000}'..='\u{200F}'
            | '\u{2028}'..='\u{202F}'
            | '\u{205F}'..='\u{206F}'
            | '\u{3000}'..='\u{FEFF}'
        )
    }

    pub fn is_valid(cp: u32) -> bool {
        match cp {
            // Surrogate codepoint values
            0xD800..=0xDBFF | 0xDC00..=0xDFFF => false,
            // Supplementary Private Use codepoint values
            0x100000..=0x10FFFF => false,
            _ => true,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for EscapeDefault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.clone() {
            f.write_char(c)?;
        }
        Ok(())
    }
}

impl Iterator for EscapeDefault {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        match self.state {
            EscapeDefaultState::Backslash(c) => {
                self.state = EscapeDefaultState::Char(c);
                Some('\\')
            }
            EscapeDefaultState::Char(c) => {
                self.state = EscapeDefaultState::Done;
                Some(c)
            }
            EscapeDefaultState::Done => None,
            EscapeDefaultState::Unicode(ref mut iter) => iter.next(),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.len();
        (n, Some(n))
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }

    fn nth(&mut self, n: usize) -> Option<char> {
        match self.state {
            EscapeDefaultState::Backslash(c) if n == 0 => {
                self.state = EscapeDefaultState::Char(c);
                Some('\\')
            }
            EscapeDefaultState::Backslash(c) if n == 1 => {
                self.state = EscapeDefaultState::Done;
                Some(c)
            }
            EscapeDefaultState::Backslash(_) => {
                self.state = EscapeDefaultState::Done;
                None
            }
            EscapeDefaultState::Char(c) => {
                self.state = EscapeDefaultState::Done;

                if n == 0 {
                    Some(c)
                } else {
                    None
                }
            }
            EscapeDefaultState::Done => None,
            EscapeDefaultState::Unicode(ref mut i) => i.nth(n),
        }
    }

    fn last(self) -> Option<char> {
        match self.state {
            EscapeDefaultState::Unicode(iter) => iter.last(),
            EscapeDefaultState::Done => None,
            EscapeDefaultState::Backslash(c) | EscapeDefaultState::Char(c) => Some(c),
        }
    }
}

impl ExactSizeIterator for EscapeDefault {
    fn len(&self) -> usize {
        match self.state {
            EscapeDefaultState::Done => 0,
            EscapeDefaultState::Char(_) => 1,
            EscapeDefaultState::Backslash(_) => 2,
            EscapeDefaultState::Unicode(ref iter) => iter.len(),
        }
    }
}

impl FusedIterator for EscapeDefault {}

// ------------------------------------------------------------------------------------------------

impl Display for EscapeUnicode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.clone() {
            f.write_char(c)?;
        }
        Ok(())
    }
}

impl Iterator for EscapeUnicode {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        match self.state {
            EscapeUnicodeState::Backslash => {
                self.state = EscapeUnicodeState::Type;
                Some('\\')
            }
            EscapeUnicodeState::Type => {
                self.state = EscapeUnicodeState::Value;
                Some('x')
            }
            EscapeUnicodeState::Value => {
                let hex_digit = ((self.c as u32) >> (self.hex_digit_idx * 4)) & 0xf;
                let c = char::from_digit(hex_digit, 16).unwrap();
                if self.hex_digit_idx == 0 {
                    self.state = EscapeUnicodeState::Semicolon;
                } else {
                    self.hex_digit_idx -= 1;
                }
                Some(c)
            }
            EscapeUnicodeState::Semicolon => {
                self.state = EscapeUnicodeState::Done;
                Some(';')
            }
            EscapeUnicodeState::Done => None,
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.len();
        (n, Some(n))
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }

    fn last(self) -> Option<char> {
        match self.state {
            EscapeUnicodeState::Done => None,

            EscapeUnicodeState::Semicolon
            | EscapeUnicodeState::Value
            | EscapeUnicodeState::Type
            | EscapeUnicodeState::Backslash => Some(';'),
        }
    }
}

impl ExactSizeIterator for EscapeUnicode {
    #[inline]
    fn len(&self) -> usize {
        // The match is a single memory access with no branching
        self.hex_digit_idx
            + match self.state {
                EscapeUnicodeState::Done => 0,
                EscapeUnicodeState::Semicolon => 1,
                EscapeUnicodeState::Value => 2,
                EscapeUnicodeState::Type => 3,
                EscapeUnicodeState::Backslash => 4,
            }
    }
}

impl FusedIterator for EscapeUnicode {}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------
