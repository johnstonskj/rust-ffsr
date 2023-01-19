/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::lexer::Lexer;
use crate::reader::iter::DatumIter;
use crate::Sourced;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

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
