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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum State {
    Nothing,
    InWhitespace,
    InDirective,
    InIdentifier,
    InNumeric,
    InString,
    InStringEscape,
    InStringHexEscape,
    InSpecial,
    InCharacter,
    InLineComment,
    InBlockComment,
    InOpenByteVector,
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

impl Default for IteratorState {
    fn default() -> Self {
        Self {
            state: State::Nothing,
            token_starts_at: Index::from(0),
        }
    }
}

impl IteratorState {
    pub(crate) fn clone_with_new_state(&self, state: State) -> Self {
        Self {
            state,
            ..self.clone()
        }
    }

    pub(crate) fn state(&self) -> State {
        self.state
    }

    pub(crate) fn set_state(&mut self, state: State) {
        self.state = state;
    }

    pub(crate) fn token_starts_at(&self) -> Index {
        self.token_starts_at
    }

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
