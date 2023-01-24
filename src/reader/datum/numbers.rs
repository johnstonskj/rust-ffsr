/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::error::Error;
use crate::lexer::token::Span;
use crate::reader::datum::{Datum, DatumValue, SimpleDatumValue};
use std::fmt::Debug;
use std::{fmt::Display, str::FromStr};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum SNumber {
    Byte(SByte),
    Integer(SInteger),
    Long(SLong),
    Float(SFloat),
    Rational(SRational),
    LongRational(SLongRational),
    ExactComplex(SExactComplex),
    Complex(SComplex),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SByte(u8);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SInteger(i64);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SLong(i128);

// precision? f32
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct SFloat(f64);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SRational(SInteger, SInteger);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SLongRational(SLong, SLong);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SExactComplex(SInteger, SInteger);

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct SComplex(SFloat, SFloat);

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

macro_rules! number_impl {
    ($number:ty, $inner_type:ty, $number_variant:ident) => {
        impl From<$inner_type> for $number {
            fn from(v: $inner_type) -> Self {
                Self(v)
            }
        }

        impl From<$number> for $inner_type {
            fn from(v: $number) -> Self {
                v.0
            }
        }

        impl From<$number> for SNumber {
            fn from(v: $number) -> Self {
                Self::$number_variant(v)
            }
        }

        impl From<$number> for Datum {
            fn from(v: $number) -> Self {
                Self::Number(SNumber::$number_variant(v))
            }
        }
    };
}

macro_rules! tuple_number_impl {
    ($number:ty, $inner_type:ty, $number_variant:ident) => {
        impl From<($inner_type, $inner_type)> for $number {
            fn from(tuple: ($inner_type, $inner_type)) -> Self {
                Self(tuple.0, tuple.1)
            }
        }

        impl From<$number> for SNumber {
            fn from(v: $number) -> Self {
                Self::$number_variant(v)
            }
        }

        impl From<$number> for Datum {
            fn from(v: $number) -> Self {
                Self::Number(SNumber::$number_variant(v))
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

number_impl!(SByte, u8, Byte);

// ------------------------------------------------------------------------------------------------

number_impl!(SInteger, i64, Integer);

// ------------------------------------------------------------------------------------------------

number_impl!(SLong, i128, Long);

// ------------------------------------------------------------------------------------------------

number_impl!(SFloat, f64, Float);

// ------------------------------------------------------------------------------------------------

tuple_number_impl!(SRational, SInteger, Rational);

// ------------------------------------------------------------------------------------------------

tuple_number_impl!(SLongRational, SLong, LongRational);

// ------------------------------------------------------------------------------------------------

impl Display for SNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Debug for SNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Byte(arg0) => f.debug_tuple("Byte").field(arg0).finish(),
            Self::Integer(arg0) => f.debug_tuple("Integer").field(arg0).finish(),
            Self::Long(arg0) => f.debug_tuple("Long").field(arg0).finish(),
            Self::Float(arg0) => f.debug_tuple("Float").field(arg0).finish(),
            Self::Rational(arg0) => f.debug_tuple("Rational").field(arg0).finish(),
            Self::LongRational(arg0) => f.debug_tuple("LongRational").field(arg0).finish(),
            Self::ExactComplex(arg0) => f.debug_tuple("ExactComplex").field(arg0).finish(),
            Self::Complex(arg0) => f.debug_tuple("Complex").field(arg0).finish(),
        }
    }
}

impl From<SNumber> for Datum {
    fn from(v: SNumber) -> Self {
        Self::Number(v)
    }
}

impl FromStr for SNumber {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_in_span(s, Span::new_char_span_from(s))
    }
}

impl DatumValue for SNumber {}

impl SimpleDatumValue for SNumber {
    fn from_str_in_span(s: &str, span: Span) -> Result<Self, Error> {
        todo!()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
