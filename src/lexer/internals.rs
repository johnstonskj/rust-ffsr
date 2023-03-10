use crate::input::indices::{CharIndex, Index};

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
    InDirectiveText,
    InIdentifier,
    InVBarIdentifier,
    InVBarIdentifierEscape,
    InNumberOrIdentifier,
    InDotNumberOrIdentifier,
    InPeculiarIdentifier,
    InNumber,
    InNumberPrefix,
    InNumberPostPrefix,
    InMaybeInf(u8),
    InMaybeNan(u8),
    InNumberFraction,
    InNumberExponentMark,
    InNumberExponent,
    InRational,
    InComplex,
    InString,
    InStringEscape,
    InSpecial,
    InCharacter,
    InCharacterName,
    InCharacterX,
    InCharacterXNum,
    InLineComment,
    InBlockComment,
    InBlockCommentBar,
    InOpenByteVector(char),
    InDatumRef,
}

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
