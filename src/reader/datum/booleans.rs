/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::reader::datum::{Datum, DatumValue, SimpleDatumValue};
use crate::{
    error::{invalid_boolean_input, Error},
    lexer::token::Span,
};
use std::{
    fmt::{Debug, Display},
    str::FromStr,
};
use tracing::error;

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
        let _span = ::tracing::trace_span!("from_str_in_span");
        let _scope = _span.enter();

        if s == "#t" {
            Ok(SBoolean::from(true))
        } else if s == "#f" {
            Ok(SBoolean::from(false))
        } else {
            error!("Invalid value for boolean {s:?}");
            Err(invalid_boolean_input(span))
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------
