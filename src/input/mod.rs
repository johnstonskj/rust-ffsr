/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::input::iter::CharIndices;
use std::borrow::{Borrow, Cow};
use std::slice::SliceIndex;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Input<'a> {
    input: Cow<'a, str>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<'a> From<&'a str> for Input<'a> {
    fn from(s: &'a str) -> Self {
        Self {
            input: Cow::Borrowed(s),
        }
    }
}

impl From<String> for Input<'_> {
    fn from(s: String) -> Self {
        Self {
            input: Cow::Owned(s),
        }
    }
}

impl<'a> Input<'a> {
    pub fn char_indices(&'a self) -> CharIndices<'a> {
        CharIndices::new(&self.input)
    }

    pub fn source(&'a self) -> &'a str {
        self.input.borrow()
    }

    pub fn get<I>(&self, index: I) -> Option<&I::Output>
    where
        I: SliceIndex<str>,
    {
        self.input.get(index)
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
