/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::input::indices::{CharIndex, Index};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub(crate) struct IteratorState {
    state: State,
    token_starts_at: Index,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct NumericState {
    exact: Option<bool>,
    radix: Option<u8>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum State {
    Nothing,
    InWhitespace,
    InDirective,
    InIdentifier,
    InVBarIdentifier,
    InVBarIdentifierEscape,
    InVBarIdentifierHexEscape,
    InVBarIdentifierHexEscapeDigits,
    InNumberOrIdentifier,
    InDotNumberOrIdentifier,
    InPeculiarIdentifier,
    InNumeric,
    InString,
    InStringEscape,
    InStringHexEscape,
    InStringHexEscapeDigits,
    InSpecial,
    InCharacter,
    InCharacterName,
    InCharacterX,
    InCharacterXNum,
    InLineComment,
    InBlockComment,
    InOpenByteVector(char),
    InDatumRefNum,
    InDatumRef,
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

impl NumericState {
    #[inline(always)]
    pub(crate) fn decimal() -> Self {
        Self {
            exact: Default::default(),
            radix: Some(10),
        }
    }

    #[inline(always)]
    pub(crate) fn inexact_decimal() -> Self {
        Self {
            exact: Some(false),
            radix: Some(10),
        }
    }

    #[inline(always)]
    pub(crate) fn is_exact(&self) -> Option<bool> {
        self.exact
    }

    #[inline(always)]
    pub(crate) fn set_exact(&mut self, c: char) {
        match c {
            'e' => self.exact = Some(true),
            'i' => self.exact = Some(false),
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    pub(crate) fn radix(&self) -> Option<u8> {
        self.radix
    }

    #[inline(always)]
    pub(crate) fn set_radix(&mut self, c: char) {
        match c {
            'b' => self.radix = Some(2),
            'o' => self.radix = Some(8),
            'd' => self.radix = Some(10),
            'x' => self.radix = Some(16),
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    pub(crate) fn has_prefix(&self) -> bool {
        self.exact.is_some() || self.radix.is_some()
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for IteratorState {
    fn default() -> Self {
        Self {
            state: State::Nothing,
            token_starts_at: Index::from(0),
        }
    }
}

impl IteratorState {
    // #[inline(always)]
    // pub(crate) fn clone_with_new_state(&self, state: State) -> Self {
    //     Self {
    //         state,
    //         ..self.clone()
    //     }
    // }

    #[inline(always)]
    pub(crate) fn state(&self) -> State {
        self.state
    }

    #[inline(always)]
    pub(crate) fn set_state(&mut self, state: State) {
        self.state = state;
    }

    #[inline(always)]
    pub(crate) fn token_starts_at(&self) -> Index {
        self.token_starts_at
    }

    #[inline(always)]
    pub(crate) fn set_token_start(&mut self, index: &CharIndex) {
        self.token_starts_at = index.index();
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
