/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::lexer::Lexer;
use crate::reader::iter::DatumIter;
use crate::Sourced;
use std::fmt::Display;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReadContext {
    TopLevel,
    InList,
    InVector,
    InByteVector,
}

#[derive(Debug)]
pub struct Reader<'a> {
    source: Lexer<'a>,
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

impl Display for ReadContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::TopLevel => "top level",
                Self::InList => "list",
                Self::InVector => "vector",
                Self::InByteVector => "byte vector",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl<'a> From<Lexer<'a>> for Reader<'a> {
    fn from(source: Lexer<'a>) -> Self {
        Self { source }
    }
}

impl Sourced for Reader<'_> {
    #[inline(always)]
    fn source_str(&self) -> &str {
        self.source.source_str()
    }
}

impl<'a> Reader<'a> {
    #[inline(always)]
    pub fn iter(&'a self) -> DatumIter<'a> {
        DatumIter::from(self.source.tokens())
    }
    #[inline(always)]
    pub fn iter_with_comments(&'a self) -> DatumIter<'a> {
        DatumIter::from(self.source.tokens()).with_comments()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod internals;

pub mod datum;

pub mod iter;
