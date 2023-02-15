/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::input::Input;
use crate::lexer::iter::TokenIter;
use crate::lexer::token::Token;
use crate::Sourced;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Lexer<'a> {
    source: Input<'a>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<'a> From<&'a str> for Lexer<'a> {
    fn from(s: &'a str) -> Self {
        Self::from(Input::from(s))
    }
}

impl From<String> for Lexer<'_> {
    fn from(s: String) -> Self {
        Self::from(Input::from(s))
    }
}

impl<'a> From<Input<'a>> for Lexer<'a> {
    fn from(source: Input<'a>) -> Self {
        Self { source }
    }
}

impl Sourced for Lexer<'_> {
    #[inline(always)]
    fn source_id(&self) -> &crate::SourceId {
        self.source.source_id()
    }

    #[inline(always)]
    fn source_str(&self) -> &str {
        self.source.source_str()
    }
}

impl<'a> Lexer<'a> {
    pub fn tokens(&'a self) -> TokenIter<'a> {
        TokenIter::from(self.source.char_indices())
    }

    #[inline(always)]
    pub fn token_str(&self, token: &Token) -> &str {
        self.get(token.byte_span().as_range()).unwrap()
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod internals;

pub mod token;

pub mod iter;
