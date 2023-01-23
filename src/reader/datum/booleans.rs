/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::{
    error::{invalid_boolean_input, Error},
    lexer::token::Span,
};
use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use super::{Datum, DatumValue, SimpleDatumValue};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SBoolean(bool);

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for SBoolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", if self.0 { 't' } else { 'f' })
    }
}

impl Debug for SBoolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", if self.0 { 't' } else { 'f' })
    }
}

impl From<bool> for SBoolean {
    fn from(v: bool) -> Self {
        Self(v)
    }
}

impl From<SBoolean> for bool {
    fn from(v: SBoolean) -> Self {
        v.0
    }
}

impl From<SBoolean> for Datum {
    fn from(v: SBoolean) -> Self {
        Self::Boolean(v)
    }
}

impl FromStr for SBoolean {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_in_span(s, Span::new_char_span_from(s))
    }
}

impl DatumValue for SBoolean {}

impl SimpleDatumValue for SBoolean {
    fn from_str_in_span(s: &str, span: Span) -> Result<Self, Error> {
        if s == "#t" {
            Ok(SBoolean::from(true))
        } else if s == "#f" {
            Ok(SBoolean::from(false))
        } else {
            Err(invalid_boolean_input(span))
        }
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
    use crate::reader::datum::SBoolean;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    #[test]
    fn test_scheme_boolean_true() {
        let test_str = "#t";
        let s = SBoolean::from_str(test_str);
        assert!(s.is_ok());
        assert_eq!(s.unwrap(), SBoolean::from(true));
    }

    #[test]
    fn test_scheme_boolean_false() {
        let test_str = "#f";
        let s = SBoolean::from_str(test_str);
        assert!(s.is_ok());
        assert_eq!(s.unwrap(), SBoolean::from(false));
    }
}
