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
    Quote(Box<Datum>),
    QuasiQuote(Box<Datum>),
    Unquote(Box<Datum>),
    UnquoteSplicing(Box<Datum>),
    Identifier(SIdentifier),
    Boolean(SBoolean),
    Char(SChar),
    Number(SNumber),
    String(SString),
    List(SList),
    Vector(SVector),
    ByteVector(SByteVector),
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
                Self::Quote(v) => format!("'{}", v),
                Self::QuasiQuote(v) => format!("`{}", v),
                Self::Unquote(v) => format!(",{}", v),
                Self::UnquoteSplicing(v) => format!(",@{}", v),
                Self::Identifier(v) => v.to_string(),
                Self::Boolean(v) => v.to_string(),
                Self::Char(v) => v.to_string(),
                Self::Number(v) => v.to_string(),
                Self::String(v) => v.to_string(),
                Self::List(v) => v.to_string(),
                Self::Vector(v) => v.to_string(),
                Self::ByteVector(v) => v.to_string(),
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
                Self::Quote(v) => format!("(quote {:?})", v),
                Self::QuasiQuote(v) => format!("(quasiquote {:?})", v),
                Self::Unquote(v) => format!("(unquote {:?})", v),
                Self::UnquoteSplicing(v) => format!("(unquote-splicing {:?})", v),
                Self::Identifier(v) => format!("{:?}", v),
                Self::Boolean(v) => format!("{:?}", v),
                Self::Char(v) => format!("{:?}", v),
                Self::Number(v) => format!("{:?}", v),
                Self::String(v) => format!("{:?}", v),
                Self::List(v) => format!("{:?}", v),
                Self::Vector(v) => format!("{:?}", v),
                Self::ByteVector(v) => format!("{:?}", v),
                Self::Comment(v) => format!("{:?}", v),
            }
        )
    }
}

impl Datum {
    pub fn quote(self) -> Self {
        Self::Quote(Box::new(self))
    }
    pub fn quasiquote(self) -> Self {
        Self::QuasiQuote(Box::new(self))
    }
    pub fn unquote(self) -> Self {
        Self::Unquote(Box::new(self))
    }
    pub fn unquote_splicing(self) -> Self {
        Self::UnquoteSplicing(Box::new(self))
    }

    pub fn is_quote(&self) -> bool {
        matches!(self, Self::Quote(_))
    }

    pub fn as_quote_inner(&self) -> Option<&Datum> {
        match self {
            Self::Quote(v) => Some(v),
            _ => None,
        }
    }

    pub fn type_string(&self) -> &'static str {
        match self {
            Self::Quote(_) => "quote",
            Self::QuasiQuote(_) => "quasiquote",
            Self::Unquote(_) => "unquote",
            Self::UnquoteSplicing(_) => "unquote-splicing",
            Self::Identifier(_) => "identifier",
            Self::Boolean(_) => "boolean",
            Self::Char(_) => "char",
            Self::Number(v) => v.type_string(),
            Self::String(_) => "string",
            Self::List(_) => "list",
            Self::Vector(_) => "vector",
            Self::ByteVector(_) => "byte-vector",
            Self::Comment(v) => v.type_string(),
        }
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
pub use numbers::{Complexnum, Fixnum, Flonum, Ratnum, SNumber};

mod strings;
pub use strings::SString;

mod vectors;
pub use vectors::{SByteVector, SVector};
