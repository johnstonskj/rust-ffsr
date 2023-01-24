/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use super::{Datum, DatumValue};
use std::fmt::{Debug, Display};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Default, PartialEq)]
pub struct SVector(Vec<Datum>);

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
            "#({})",
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
            "#({})",
            self.0
                .iter()
                .map(|d| format!("{:?}", d))
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl From<SVector> for Datum {
    fn from(v: SVector) -> Self {
        Datum::Vector(v)
    }
}

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

impl DatumValue for SVector {}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
