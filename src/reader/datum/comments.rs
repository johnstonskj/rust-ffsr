/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use super::{Datum, DatumValue};
use crate::error::Error;
use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

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
        if s.starts_with(';') && !s.contains('\n') {
            Ok(Self::Line(s[1..].trim().to_string()))
        } else if s.starts_with("#|") && s.ends_with("|#") {
            Ok(Self::Block(s[2..s.len() - 2].trim().to_string()))
        } else if s.starts_with("#;") {
            unimplemented!()
        } else {
            unimplemented! {}
        }
    }
}

impl DatumValue for SComment {}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
