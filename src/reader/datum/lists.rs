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
pub struct SList(Vec<Datum>);

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for SList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({})",
            self.0
                .iter()
                .map(|d| d.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl Debug for SList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({})",
            self.0
                .iter()
                .map(|d| format!("{:?}", d))
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl From<SList> for Datum {
    fn from(v: SList) -> Self {
        Datum::List(v)
    }
}

impl From<Datum> for SList {
    fn from(v: Datum) -> Self {
        Self(vec![v])
    }
}

impl From<Vec<Datum>> for SList {
    fn from(v: Vec<Datum>) -> Self {
        Self(v)
    }
}

impl FromIterator<Datum> for SList {
    fn from_iter<T: IntoIterator<Item = Datum>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl DatumValue for SList {}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------
