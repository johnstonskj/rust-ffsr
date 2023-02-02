/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::lexer::token::Span;
use crate::reader::datum::Datum;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub(crate) struct IteratorState {
    state: State,
    content: Option<Vec<Datum>>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum State {
    TopLevel,
    Quote(QuoteKind, Span),
    DatumComment(Span),
    DatumAssign(u16),
    List(Span),
    Vector(Span),
    ByteVector(Span),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum QuoteKind {
    Quote,
    QuasiQuote,
    Unquote,
    UnquoteSplicing,
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
            content: match state {
                State::List(_) | State::Vector(_) | State::ByteVector(_) => Some(Vec::default()),
                _ => Default::default(),
            },
        }
    }

    #[inline(always)]
    pub(crate) fn state(&self) -> State {
        self.state
    }

    #[inline(always)]
    pub(crate) fn add_content(&mut self, datum: Datum) {
        if let Some(content) = self.content.as_mut() {
            content.push(datum);
        } else {
            panic!();
        }
    }

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
