/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::input::iter::CharIndices;
use crate::Sourced;
use std::borrow::{Borrow, Cow};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Input<'a> {
    source: Cow<'a, str>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<'a> From<&'a str> for Input<'a> {
    fn from(s: &'a str) -> Self {
        Self {
            source: Cow::Borrowed(s),
        }
    }
}

impl From<String> for Input<'_> {
    fn from(s: String) -> Self {
        println!("Input from {:?}/{}", s, s.len());
        Self {
            source: Cow::Owned(s),
        }
    }
}

impl Sourced for Input<'_> {
    #[inline(always)]
    fn source_str(&self) -> &str {
        self.source.borrow()
    }
}

impl<'a> Input<'a> {
    pub fn char_indices(&'a self) -> CharIndices<'a> {
        CharIndices::new(&self.source)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod indices;

pub mod iter;
