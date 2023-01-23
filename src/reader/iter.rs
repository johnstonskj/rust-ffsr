/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::error::{invalid_datum_label, unexpected_token, Error, ReadContext};
use crate::lexer::iter::TokenIter;
use crate::lexer::token::TokenKind;
use crate::reader::{
    datum::{Datum, SBoolean, SChar, SComment, SString, SimpleDatumValue},
    internals::IteratorState,
    internals::State,
};
use std::str::FromStr;

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
        }
    }
}

impl Iterator for DatumIter<'_> {
    type Item = Result<Datum, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current_state = IteratorState::default();
        while let Some(token) = self.source.next() {
            let token = match token {
                Ok(t) => t,
                Err(e) => {
                    return Some(Err(e));
                }
            };

            match (current_state.state(), token.kind()) {
                (State::Nothing | State::DatumAssign(_), TokenKind::Boolean) => {
                    let datum =
                        SBoolean::from_str_in_span(self.source.token_str(&token), token.span());
                    let datum = datum.unwrap(); // TODO: FIX THIS
                    if let State::DatumAssign(label) = current_state.state() {
                        current_state = self.state_stack.pop().unwrap();
                        current_state.insert_labeled(label, datum.into());
                    }
                    return Some(Ok(datum.into()));
                }
                (State::Nothing | State::DatumAssign(_), TokenKind::Character) => {
                    let datum =
                        SChar::from_str_in_span(self.source.token_str(&token), token.span());
                    let datum = datum.unwrap(); // TODO: FIX THIS
                    if let State::DatumAssign(label) = current_state.state() {
                        current_state = self.state_stack.pop().unwrap();
                        current_state.insert_labeled(label, datum.into());
                    }
                    return Some(Ok(datum.into()));
                }
                (State::Nothing | State::DatumAssign(_), TokenKind::String) => {
                    let datum =
                        SString::from_str_in_span(self.source.token_str(&token), token.span());
                    let datum = datum.unwrap(); // TODO: FIX THIS
                    if let State::DatumAssign(label) = current_state.state() {
                        current_state = self.state_stack.pop().unwrap();
                        current_state.insert_labeled(label, datum.clone().into());
                    }
                    return Some(Ok(datum.into()));
                }
                (State::Nothing, TokenKind::BlockComment) if self.return_comments => {
                    let content = self.source.token_str(&token);
                    let content = content[2..content.len() - 2].trim().to_string();
                    return Some(Ok(SComment::Block(content).into()));
                }
                (State::Nothing, TokenKind::LineComment) if self.return_comments => {
                    let content = self.source.token_str(&token);
                    let content = content[1..].trim().to_string();
                    return Some(Ok(SComment::Line(content).into()));
                }
                (State::Nothing, TokenKind::DatumComment) => {
                    self.state_stack.push(current_state);
                    current_state = State::DatumComment.into();
                }
                (State::DatumComment, k) if is_datum(k) && !self.return_comments => {
                    println!("Dropping token {:?}; you said it was commented out", token);
                    current_state = self.state_stack.pop().unwrap();
                }
                (State::Nothing, TokenKind::DatumAssign) => {
                    let label_str = self.source.token_str(&token);
                    let label_str = &label_str[1..label_str.len() - 1];
                    let label = u16::from_str(label_str).expect("not a number?");
                    if current_state.contains_label(label) {
                        return Some(Err(invalid_datum_label(token.span())));
                    }
                    self.state_stack.push(current_state);
                    current_state = State::DatumAssign(label).into();
                }
                (State::Nothing, TokenKind::DatumRef) => {
                    let label_str = self.source.token_str(&token);
                    let label_str = &label_str[1..label_str.len() - 1];
                    let label = u16::from_str(label_str).expect("not a number?");
                    return Some(current_state.get_labeled(label, token.span()));
                }
                (_, kind) => {
                    return Some(Err(unexpected_token(
                        kind,
                        ReadContext::TopLevel,
                        token.span(),
                    )));
                }
            }
        }
        None
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

#[inline(always)]
fn is_datum(token: TokenKind) -> bool {
    matches!(token, TokenKind::String)
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
