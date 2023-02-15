/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::error::{invalid_char_input, invalid_unicode_value, unknown_char_name, Error};
use crate::lexer::token::Span;
use crate::reader::datum::SimpleDatumValue;
use crate::syntax::{
    CHAR_ESCAPE_ALARM, CHAR_ESCAPE_BACKSPACE, CHAR_ESCAPE_NEWLINE, CHAR_ESCAPE_RETURN,
    CHAR_ESCAPE_TAB, CHAR_HEX_ESCAPE_END, CHAR_HEX_ESCAPE_START, CHAR_NAME_ALARM,
    CHAR_NAME_BACKSPACE, CHAR_NAME_DELETE, CHAR_NAME_ESCAPE, CHAR_NAME_NEWLINE, CHAR_NAME_NULL,
    CHAR_NAME_RETURN, CHAR_NAME_SPACE, CHAR_NAME_TAB, CHAR_PREFIX_STR, CHAR_VALUE_ALARM,
    CHAR_VALUE_BACKSLASH, CHAR_VALUE_BACKSPACE, CHAR_VALUE_DELETE, CHAR_VALUE_ESCAPE,
    CHAR_VALUE_NEWLINE, CHAR_VALUE_NULL, CHAR_VALUE_QUOTE, CHAR_VALUE_RETURN, CHAR_VALUE_SPACE,
    CHAR_VALUE_TAB, CHAR_VALUE_VBAR,
};
use std::fmt::Write;
use std::fmt::{Debug, Display};
use std::iter::FusedIterator;
use tracing::error;

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
        write!(f, "{}{}", CHAR_PREFIX_STR, self.0)
    }
}

impl Debug for SChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", CHAR_PREFIX_STR, self.escape_default())
    }
}

impl_datum_value!(Char, SChar, infallible char);

impl_simple_datum_from_str!(Char, SChar);

impl SimpleDatumValue for SChar {
    fn from_str_in_span(s: &str, span: Span) -> Result<Self, Error> {
        let _span = ::tracing::trace_span!("from_str_in_span", s, ?span);
        let _scope = _span.enter();

        if s.len() > 2 && s.starts_with(CHAR_PREFIX_STR) {
            let s = &s[2..];
            if s == CHAR_NAME_ALARM {
                Ok(SChar::from(CHAR_VALUE_ALARM))
            } else if s == CHAR_NAME_BACKSPACE {
                Ok(SChar::from(CHAR_VALUE_BACKSPACE))
            } else if s == CHAR_NAME_DELETE {
                Ok(SChar::from(CHAR_VALUE_DELETE))
            } else if s == CHAR_NAME_ESCAPE {
                Ok(SChar::from(CHAR_VALUE_ESCAPE))
            } else if s == CHAR_NAME_NEWLINE {
                Ok(SChar::from(CHAR_VALUE_NEWLINE))
            } else if s == CHAR_NAME_NULL {
                Ok(SChar::from(CHAR_VALUE_NULL))
            } else if s == CHAR_NAME_RETURN {
                Ok(SChar::from(CHAR_VALUE_RETURN))
            } else if s == CHAR_NAME_SPACE {
                Ok(SChar::from(CHAR_VALUE_SPACE))
            } else if s == CHAR_NAME_TAB {
                Ok(SChar::from(CHAR_VALUE_TAB))
            } else if s.starts_with(CHAR_HEX_ESCAPE_START) && s.ends_with(CHAR_HEX_ESCAPE_END) {
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
            error!("Char does not start with prefix {CHAR_PREFIX_STR:?}");
            invalid_char_input(span)
        }
    }
}

impl SChar {
    /// See [`char::escape_default`]
    pub fn escape_default(&self) -> EscapeDefault {
        let init_state = match self.0 {
            CHAR_VALUE_ALARM => EscapeDefaultState::Backslash(CHAR_ESCAPE_ALARM),
            CHAR_VALUE_BACKSPACE => EscapeDefaultState::Backslash(CHAR_ESCAPE_BACKSPACE),
            CHAR_VALUE_TAB => EscapeDefaultState::Backslash(CHAR_ESCAPE_TAB),
            CHAR_VALUE_RETURN => EscapeDefaultState::Backslash(CHAR_ESCAPE_RETURN),
            CHAR_VALUE_NEWLINE => EscapeDefaultState::Backslash(CHAR_ESCAPE_NEWLINE),
            CHAR_VALUE_QUOTE | CHAR_VALUE_BACKSLASH | CHAR_VALUE_VBAR => {
                EscapeDefaultState::Backslash(self.0)
            }
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
                Some(CHAR_VALUE_BACKSLASH)
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
                Some(CHAR_VALUE_BACKSLASH)
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
                Some(CHAR_VALUE_BACKSLASH)
            }
            EscapeUnicodeState::Type => {
                self.state = EscapeUnicodeState::Value;
                Some(CHAR_HEX_ESCAPE_START)
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
                Some(CHAR_HEX_ESCAPE_END)
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
            | EscapeUnicodeState::Backslash => Some(CHAR_HEX_ESCAPE_END),
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
