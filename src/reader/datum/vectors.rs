/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::error::{invalid_byte_input, Error};
use crate::lexer::token::Span;
use crate::reader::datum::numbers::{Fixnum, Integer};
use crate::reader::datum::Datum;
use crate::syntax::{BYTE_VECTOR_END, BYTE_VECTOR_START, VECTOR_END, VECTOR_START};
use std::fmt::{Debug, Display};
use std::ops::Deref;
use tracing::error;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Default, PartialEq)]
pub struct SVector(Vec<Datum>);

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct SByteVector(Vec<Fixnum>);

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for SVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{VECTOR_START}{}{VECTOR_END}",
            self.0
                .iter()
                .map(|d| d.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl Debug for SVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{VECTOR_START}{}{VECTOR_END}",
            self.0
                .iter()
                .map(|d| format!("{d:?}"))
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl_datum_value!(Vector, SVector);

impl From<Datum> for SVector {
    fn from(v: Datum) -> Self {
        Self(vec![v])
    }
}

impl From<Vec<Datum>> for SVector {
    fn from(v: Vec<Datum>) -> Self {
        Self(v)
    }
}

impl FromIterator<Datum> for SVector {
    fn from_iter<T: IntoIterator<Item = Datum>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl SVector {
    pub fn append(&mut self, datum: Datum) {
        self.0.push(datum)
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for SByteVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{BYTE_VECTOR_START}{}{BYTE_VECTOR_END}",
            self.0
                .iter()
                .map(|d| d.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl Debug for SByteVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{BYTE_VECTOR_START}{}{BYTE_VECTOR_END}",
            self.0
                .iter()
                .map(|d| format!("{:?}", d))
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl_datum_value!(ByteVector, SByteVector);

impl From<Fixnum> for SByteVector {
    fn from(v: Fixnum) -> Self {
        Self(vec![v])
    }
}

impl From<Vec<Fixnum>> for SByteVector {
    fn from(v: Vec<Fixnum>) -> Self {
        Self(v)
    }
}

impl FromIterator<Fixnum> for SByteVector {
    fn from_iter<T: IntoIterator<Item = Fixnum>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl SByteVector {
    pub(crate) fn try_append_datum(&mut self, datum: Datum, span: Span) -> Result<(), Error> {
        let number = if let Some(number) = datum.as_number() {
            number
        } else {
            error!(
                "Invalid datum type {}, expecting fixnum",
                datum.type_string()
            );
            return invalid_byte_input(span);
        };

        if let Some(fixnum) = number.as_fixnum() {
            self.try_append(fixnum.clone())
        } else {
            error!(
                "Invalid numeric type {}, expecting fixnum",
                number.type_string()
            );
            return invalid_byte_input(span);
        }
    }

    pub fn try_append(&mut self, fixnum: Fixnum) -> Result<(), Error> {
        if fixnum.deref() >= &Integer::from(0) && fixnum.deref() <= &Integer::from(255) {
            self.0.push(fixnum);
        } else {
            panic!("Not a valid fixnum value, #e0..#e255");
        }
        Ok(())
    }

    pub fn append(&mut self, byte: u8) {
        self.0.push(byte.into());
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------
