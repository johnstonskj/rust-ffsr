/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::{error::Error, lexer::token::Span};
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

#[derive(Clone, PartialEq)]
pub enum Datum {
    Identifier(SIdentifier),
    Boolean(SBoolean),
    Char(SChar),
    Number(SNumber),
    String(SString),
    List(SList),
    Vector(SVector),
    Comment(SComment),
}

pub trait DatumValue: Display + Debug + Into<Datum> {}

pub trait SimpleDatumValue: DatumValue + FromStr {
    fn from_str_in_span(s: &str, span: Span) -> Result<Self, Error>;
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

impl Display for Datum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Identifier(v) => v.to_string(),
                Self::Boolean(v) => v.to_string(),
                Self::Char(v) => v.to_string(),
                Self::Number(v) => v.to_string(),
                Self::String(v) => v.to_string(),
                Self::List(v) => v.to_string(),
                Self::Vector(v) => v.to_string(),
                Self::Comment(v) => v.to_string(),
            }
        )
    }
}

impl Debug for Datum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Identifier(v) => format!("{:?}", v),
                Self::Boolean(v) => format!("{:?}", v),
                Self::Char(v) => format!("{:?}", v),
                Self::Number(v) => format!("{:?}", v),
                Self::String(v) => format!("{:?}", v),
                Self::List(v) => format!("{:?}", v),
                Self::Vector(v) => format!("{:?}", v),
                Self::Comment(v) => format!("{:?}", v),
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod booleans;
pub use booleans::SBoolean;

mod chars;
pub use chars::{EscapeDefault, EscapeUnicode, SChar};

mod comments;
pub use comments::SComment;

mod identifiers;
pub use identifiers::SIdentifier;

mod lists;
pub use lists::SList;

pub mod numbers;
pub use numbers::{Complexnum, ExactComplexnum, Fixnum, Flonum, Ratnum, SNumber};

mod strings;
pub use strings::SString;

mod vectors;
pub use vectors::SVector;
