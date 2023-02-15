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

macro_rules! impl_datum_value {
    ($datum_variant:ident, $datum_type:ty, infallible $rust_type:ty) => {
        impl_datum_value!($datum_variant, $datum_type, $rust_type);

        impl From<$rust_type> for $datum_type {
            fn from(v: $rust_type) -> Self {
                Self(v)
            }
        }

        impl From<$rust_type> for $crate::reader::datum::Datum {
            fn from(v: $rust_type) -> Self {
                Self::$datum_variant(v.into())
            }
        }
    };
    ($datum_variant:ident, $datum_type:ty, $rust_type:ty) => {
        impl_datum_value!($datum_variant, $datum_type);

        impl From<$datum_type> for $rust_type {
            fn from(v: $datum_type) -> Self {
                v.0
            }
        }
    };
    ($datum_variant:ident, $datum_type:ty) => {
        impl From<$datum_type> for $crate::reader::datum::Datum {
            fn from(v: $datum_type) -> Self {
                Self::$datum_variant(v)
            }
        }

        impl $crate::reader::datum::DatumValue for $datum_type {}
    };
}

macro_rules! impl_simple_datum_from_str {
    ($datum_variant:ident, $datum_type:ty) => {
        impl ::std::str::FromStr for $datum_type {
            type Err = $crate::error::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::from_str_in_span(s, Span::new_char_span_from(s))
            }
        }
    };
}

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
    Directive(SDirective),
}

pub trait DatumValue: Display + Debug + Into<Datum> {}

pub trait SimpleDatumValue: DatumValue + FromStr {
    fn from_str_in_span(s: &str, span: Span) -> Result<Self, Error>;
}

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
                Self::Directive(v) => v.to_string(),
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
                Self::Directive(v) => format!("{:?}", v),
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

    is_as_variant!(
        (quote, Quote, Datum),
        (quasi_quote, QuasiQuote, Datum),
        (unquote, Unquote, Datum),
        (unquote_splicing, Quote, Datum),
        (identifier, Identifier, SIdentifier),
        (boolean, Boolean, SBoolean),
        (character, Char, SChar),
        (number, Number, SNumber),
        (string, String, SString),
        (list, List, SList),
        (vector, Vector, SVector),
        (byte_vector, ByteVector, SByteVector),
        (comment, Comment, SComment)
    );

    pub fn is_empty_list(&self) -> bool {
        self.as_list()
            .map(|list| list.is_empty())
            .unwrap_or_default()
    }

    into_variant!(
        (quote, Quote, Box<Datum>),
        (quasi_quote, QuasiQuote, Box<Datum>),
        (unquote, Unquote, Box<Datum>),
        (unquote_splicing, Quote, Box<Datum>),
        (identifier, Identifier, SIdentifier),
        (boolean, Boolean, SBoolean),
        (character, Char, SChar),
        (number, Number, SNumber),
        (string, String, SString),
        (list, List, SList),
        (vector, Vector, SVector),
        (byte_vector, ByteVector, SByteVector),
        (comment, Comment, SComment)
    );

    pub fn type_string(&self) -> &'static str {
        match_into_str!(
            self,
            (Self::Quote(_) => "quote"),
            (Self::QuasiQuote(_) => "quasiquote"),
            (Self::Unquote(_) => "unquote"),
            (Self::UnquoteSplicing(_) => "unquote-splicing"),
            (Self::Identifier(_) => "identifier"),
            (Self::Boolean(_) => "boolean"),
            (Self::Char(_) => "char"),
            (Self::Number(v) => v.type_string()),
            (Self::String(_) => "string"),
            (Self::List(_) => "pair-or-list"),
            (Self::Vector(_) => "vector"),
            (Self::ByteVector(_) => "byte-vector"),
            (Self::Comment(v) => v.type_string()),
            (Self::Directive(_) => "directive")
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod booleans;
pub use booleans::SBoolean;

mod chars;
pub use chars::{EscapeDefault, EscapeUnicode, SChar};

mod comments;
pub use comments::SComment;

mod directives;
pub use directives::SDirective;

mod identifiers;
pub use identifiers::SIdentifier;

mod lists;
pub use lists::{SList, SPair, EMPTY_LIST};

pub mod numbers;
pub use numbers::{Complexnum, Fixnum, Flonum, Ratnum, SNumber};

mod strings;
pub use strings::SString;

mod vectors;
pub use vectors::{SByteVector, SVector};
