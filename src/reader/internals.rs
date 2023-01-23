/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::{
    error::{invalid_datum_label, Error},
    lexer::token::Span,
    reader::datum::Datum,
};
use std::collections::HashMap;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub(crate) struct IteratorState {
    state: State,
    ref_table: HashMap<u16, Datum>,
    //content: Vec<Datum>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum State {
    Nothing,
    DatumComment,
    DatumAssign(u16),
    // List,
    // Vector,
    // ByteVector,
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
            ref_table: Default::default(),
            //content: Default::default(),
        }
    }
}

impl From<State> for IteratorState {
    fn from(v: State) -> Self {
        Self {
            state: v,
            ref_table: Default::default(),
            //content: Default::default(),
        }
    }
}

impl IteratorState {
    #[inline(always)]
    pub(crate) fn state(&self) -> State {
        self.state
    }

    #[inline(always)]
    pub(crate) fn contains_label(&self, label: u16) -> bool {
        self.ref_table.contains_key(&label)
    }

    #[inline(always)]
    pub(crate) fn insert_labeled(&mut self, label: u16, datum: Datum) {
        self.ref_table.insert(label, datum);
    }

    #[inline(always)]
    pub(crate) fn get_labeled(&mut self, label: u16, span: Span) -> Result<Datum, Error> {
        self.ref_table
            .get(&label)
            .cloned()
            .ok_or_else(|| invalid_datum_label(span))
    }

    // #[inline(always)]
    // pub(crate) fn push_content(&mut self, datum: Datum) {
    //     self.content.push(datum)
    // }
    //
    // #[inline(always)]
    // pub(crate) fn content(&self) -> impl Iterator<Item = &Datum> {
    //     self.content.iter()
    // }
    //
    // #[inline(always)]
    // pub(crate) fn into_content(self) -> Vec<Datum> {
    //     self.content
    // }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
