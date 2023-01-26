/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use super::{Datum, DatumValue, SimpleDatumValue};
use crate::{error::Error, lexer::token::Span};
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

#[derive(Clone, Debug, PartialEq)]
pub enum SComment {
    Datum(Box<Datum>),
    Block(String),
    Line(String),
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for SComment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SComment::Datum(v) => format!("#; {}", v),
                SComment::Block(v) => format!("#| {} |#", v),
                SComment::Line(v) => format!("; {}", v),
            }
        )
    }
}

impl From<Datum> for SComment {
    fn from(v: Datum) -> Self {
        SComment::Datum(Box::new(v))
    }
}

impl From<SComment> for Datum {
    fn from(v: SComment) -> Self {
        Self::Comment(v)
    }
}

impl FromStr for SComment {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_in_span(s, Span::new_char_span_from(s))
    }
}

impl DatumValue for SComment {}

impl SimpleDatumValue for SComment {
    fn from_str_in_span(s: &str, _span: crate::lexer::token::Span) -> Result<Self, Error> {
        let _span = ::tracing::trace_span!("from_str_in_span");
        let _scope = _span.enter();

        if s.starts_with(';') && !s.contains('\n') {
            trace!("line comment");
            Ok(Self::Line(s[1..].trim().to_string()))
        } else if s.starts_with("#|") && s.ends_with("|#") {
            trace!("Block comment");
            Ok(Self::Block(s[2..s.len() - 2].trim().to_string()))
        } else if s.starts_with("#;") {
            trace!("Datum comment");
            unimplemented!()
        } else {
            error!("No clue");
            unimplemented!()
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
