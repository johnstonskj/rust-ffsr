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
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Reader<'a> {
    source: Lexer<'a>,
}

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
    fn source_id(&self) -> &crate::SourceId {
        self.source.source_id()
    }

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
// Modules
// ------------------------------------------------------------------------------------------------

mod internals;

pub mod datum;

pub mod iter;
