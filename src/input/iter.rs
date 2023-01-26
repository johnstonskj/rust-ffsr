/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::{
    input::indices::{CharIndex, Index},
    Sourced,
};
use std::str::CharIndices as ActualCharIndices;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct CharIndices<'a> {
    source: &'a str,
    iter: ActualCharIndices<'a>,
    current_index: Index,
    pushback_stack: Vec<CharIndex>,
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

impl Iterator for CharIndices<'_> {
    type Item = CharIndex;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pushback_stack.is_empty() {
            if let Some((i, c)) = self.iter.next() {
                self.current_index.set_byte(i);
                let char_index = self.current_index.to_char_index(c);
                self.current_index.increment_character();
                Some(char_index)
            } else {
                None
            }
        } else {
            self.pushback_stack.pop()
        }
    }
}

impl Sourced for CharIndices<'_> {
    #[inline(always)]
    fn source_str(&self) -> &str {
        self.source
    }
}

impl<'a> CharIndices<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            iter: source.char_indices(),
            current_index: Default::default(),
            pushback_stack: Default::default(),
        }
    }

    pub fn from(&self, starts_from: Index) -> Self {
        Self {
            source: self.source,
            iter: self.source[starts_from.byte()..].char_indices(),
            current_index: starts_from,
            pushback_stack: Default::default(),
        }
    }

    #[inline(always)]
    pub fn push_back(&mut self, v: CharIndex) {
        self.pushback_stack.push(v)
    }

    #[inline(always)]
    pub fn peek(&mut self) -> Option<&CharIndex> {
        if let Some(v) = self.next() {
            self.push_back(v);
            self.pushback_stack.last()
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn current_index(&self) -> Index {
        self.current_index
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
