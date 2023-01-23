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

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Datum {
    Boolean(SBoolean),
    Char(SChar),
    String(SString),
    Comment(SComment),
    List(SList),
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
                Datum::Boolean(v) => v.to_string(),
                Datum::Char(v) => v.to_string(),
                Datum::String(v) => v.to_string(),
                Datum::Comment(v) => v.to_string(),
                Datum::List(v) => v.to_string(),
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
                Self::Boolean(v) => format!("{:?}", v),
                Self::Char(v) => format!("{:?}", v),
                Self::String(v) => format!("{:?}", v),
                Self::Comment(v) => format!("{:?}", v),
                Self::List(v) => format!("{:?}", v),
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

mod lists;
pub use lists::SList;

mod strings;
pub use strings::SString;
