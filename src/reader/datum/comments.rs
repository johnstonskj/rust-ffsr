/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use super::{Datum, SimpleDatumValue};
use crate::{
    error::Error,
    lexer::token::Span,
    syntax::{COMMENT_BLOCK_END, COMMENT_BLOCK_START, COMMENT_DATUM_START, COMMENT_LINE_START},
};
use std::fmt::{Debug, Display};
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
                SComment::Datum(v) => format!("{COMMENT_DATUM_START} {v}"),
                SComment::Block(v) => format!("{COMMENT_BLOCK_START} {v} {COMMENT_BLOCK_END}"),
                SComment::Line(v) => format!("{COMMENT_LINE_START} {v}"),
            }
        )
    }
}

impl From<Datum> for SComment {
    fn from(v: Datum) -> Self {
        SComment::Datum(Box::new(v))
    }
}

impl_datum_value!(Comment, SComment);

impl_simple_datum_from_str!(Comment, SComment);

impl SimpleDatumValue for SComment {
    fn from_str_in_span(s: &str, span: crate::lexer::token::Span) -> Result<Self, Error> {
        let _span = ::tracing::trace_span!("from_str_in_span", s, ?span);
        let _scope = _span.enter();

        if s.starts_with(COMMENT_LINE_START) && !s.contains('\n') {
            trace!("line comment");
            Ok(Self::Line(s[1..].trim().to_string()))
        } else if s.starts_with(COMMENT_BLOCK_START) && s.ends_with(COMMENT_BLOCK_END) {
            trace!("Block comment");
            Ok(Self::Block(s[2..s.len() - 2].trim().to_string()))
        } else if s.starts_with(COMMENT_DATUM_START) {
            trace!("Datum comment");
            unimplemented!()
        } else {
            error!("No clue");
            unimplemented!()
        }
    }
}

impl SComment {
    pub fn type_string(&self) -> &'static str {
        match self {
            Self::Datum(_) => "datum-comment",
            Self::Block(_) => "block-comment",
            Self::Line(_) => "line-comment",
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
