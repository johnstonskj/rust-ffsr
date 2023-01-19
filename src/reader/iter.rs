/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::error::Error;
use crate::lexer::iter::TokenIter;
use crate::lexer::token::TokenKind;
use crate::reader::{
    datum::{Comment, Datum},
    internals::IteratorState,
};
use std::collections::HashMap;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct DatumIter<'a> {
    source: TokenIter<'a>,
    return_comments: bool,
    state_stack: Vec<IteratorState>,
    ref_table: HashMap<u16, Datum>,
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

impl<'a> From<TokenIter<'a>> for DatumIter<'a> {
    fn from(source: TokenIter<'a>) -> Self {
        Self {
            source,
            return_comments: false,
            state_stack: Default::default(),
            ref_table: Default::default(),
        }
    }
}

impl Iterator for DatumIter<'_> {
    type Item = Result<Datum, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let current_state = IteratorState::default();
        while let Some(token) = self.source.next() {
            let token = match token {
                Ok(t) => t,
                Err(e) => {
                    return Some(Err(e));
                }
            };

            match (current_state.state(), token.kind()) {
                (_, TokenKind::BlockComment) if self.return_comments => {
                    let content = self.source.token_str(&token);
                    return Some(Ok(Comment::Block(content.to_string()).into()));
                }
                (_, TokenKind::LineComment) if self.return_comments => {
                    let content = self.source.token_str(&token);
                    return Some(Ok(Comment::Line(content.to_string()).into()));
                }
                _ => {
                    eprintln!("{:?}", token);
                }
            }
        }
        None
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
