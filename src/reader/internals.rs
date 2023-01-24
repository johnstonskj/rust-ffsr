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

use super::ReadContext;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub(crate) struct IteratorState {
    state: State,
    context: ReadContext,
    ref_table: HashMap<u16, Datum>,
    content: Option<Vec<Datum>>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum State {
    TopLevel,
    DatumComment,
    DatumAssign(u16),
    List,
    Vector,
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
        Self::new(State::TopLevel)
    }
}

impl From<State> for IteratorState {
    fn from(v: State) -> Self {
        Self::new(v)
    }
}

impl IteratorState {
    #[inline(always)]
    pub(crate) fn new(state: State) -> Self {
        Self {
            state,
            context: ReadContext::TopLevel,
            ref_table: Default::default(),
            content: Default::default(),
        }
    }

    #[inline(always)]
    pub(crate) fn with_context(self, context: ReadContext) -> Self {
        match context {
            ReadContext::TopLevel => Self { context, ..self },
            ReadContext::InList => Self {
                context,
                content: Some(Vec::default()),
                ..self
            },
            ReadContext::InVector => Self {
                context,
                content: Some(Vec::default()),
                ..self
            },
            ReadContext::InByteVector => Self {
                context,
                content: Some(Vec::default()),
                ..self
            },
        }
    }

    #[inline(always)]
    pub(crate) fn state(&self) -> State {
        self.state
    }

    #[inline(always)]
    pub(crate) fn context(&self) -> ReadContext {
        self.context
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

    #[inline(always)]
    pub(crate) fn add_content(&mut self, datum: Datum) {
        if let Some(content) = self.content.as_mut() {
            content.push(datum);
        } else {
            panic!();
        }
    }

    // [inline(always)]
    // ub(crate) fn content(&self) -> impl Iterator<Item = &Datum> {
    //    if let Some(content) = &self.content {
    //        content.iter()
    //    } else {
    //        panic!();
    //    }
    //

    #[inline(always)]
    pub(crate) fn into_content(self) -> Vec<Datum> {
        if let Some(content) = self.content {
            content
        } else {
            panic!();
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
