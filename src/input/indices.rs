/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use std::ops::{Add, Sub};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Index {
    byte: usize,
    character: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CharIndex {
    index: Index,
    character: char,
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

impl Default for Index {
    fn default() -> Self {
        Self::from(0)
    }
}

impl From<usize> for Index {
    fn from(v: usize) -> Self {
        Self {
            byte: v,
            character: v,
        }
    }
}

impl From<(usize, usize)> for Index {
    fn from(v: (usize, usize)) -> Self {
        Self {
            byte: v.0,
            character: v.1,
        }
    }
}

impl Add for Index {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            byte: self.byte + rhs.byte,
            character: self.character + rhs.character,
        }
    }
}

impl Add<usize> for Index {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self {
            byte: self.byte + rhs,
            character: self.character + rhs,
        }
    }
}

impl Index {
    pub fn new(byte: usize, character: usize) -> Self {
        Self { byte, character }
    }

    #[inline(always)]
    pub fn byte(&self) -> usize {
        self.byte
    }

    #[inline(always)]
    pub fn character(&self) -> usize {
        self.character
    }

    pub fn to_char_index(&self, character: char) -> CharIndex {
        CharIndex::new_from(*self, character)
    }

    #[inline(always)]
    pub(crate) fn set_byte(&mut self, byte: usize) {
        self.byte = byte
    }

    #[inline(always)]
    pub(crate) fn increment_character(&mut self) {
        self.character += 1;
    }
}

// ------------------------------------------------------------------------------------------------

impl PartialEq<char> for CharIndex {
    fn eq(&self, other: &char) -> bool {
        &self.character == other
    }
}

impl From<CharIndex> for Index {
    fn from(ci: CharIndex) -> Self {
        ci.index
    }
}

impl From<CharIndex> for char {
    fn from(ci: CharIndex) -> Self {
        ci.character
    }
}

impl Add<(usize, usize)> for CharIndex {
    type Output = Self;

    fn add(self, rhs: (usize, usize)) -> Self::Output {
        Self {
            index: Index::new(self.byte_index() + rhs.0, self.char_index() + rhs.1),
            character: self.character,
        }
    }
}

impl Sub<(usize, usize)> for CharIndex {
    type Output = Self;

    fn sub(self, rhs: (usize, usize)) -> Self::Output {
        Self {
            index: Index::new(self.byte_index() - rhs.0, self.char_index() - rhs.1),
            character: self.character,
        }
    }
}

impl CharIndex {
    pub fn new(byte_index: usize, char_index: usize, character: char) -> Self {
        Self {
            index: Index::new(byte_index, char_index),
            character,
        }
    }

    pub fn new_from(index: Index, character: char) -> Self {
        Self { index, character }
    }

    #[inline(always)]
    pub fn index(&self) -> Index {
        self.index
    }

    #[inline(always)]
    pub fn byte_index(&self) -> usize {
        self.index.byte()
    }

    #[inline(always)]
    pub fn char_index(&self) -> usize {
        self.index.character()
    }

    #[inline(always)]
    pub fn character(&self) -> char {
        self.character
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
