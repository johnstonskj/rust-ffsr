/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::error::{invalid_datum_label, unexpected_token, Error};
use crate::lexer::iter::TokenIter;
use crate::lexer::token::TokenKind;
use crate::reader::datum::SNumber;
use crate::reader::{
    datum::{Datum, SBoolean, SChar, SComment, SString, SVector, SimpleDatumValue},
    internals::IteratorState,
    internals::State,
};
use std::str::FromStr;

use super::datum::{SIdentifier, SList};
use super::ReadContext;

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

macro_rules! atomic_datum {
    ($me:expr, $current_state:expr, $datum:expr) => {
        if $current_state.state() == State::List || $current_state.state() == State::Vector {
            println!("reader adding {:?} to open list", $datum);
            $current_state.add_content($datum.clone().into());
        } else {
            if let State::DatumAssign(label) = $current_state.state() {
                println!("reading assigning {:?} to label {:?}", $datum, label);
                $current_state = $me.state_stack.pop().unwrap();
                $current_state.insert_labeled(label, $datum.clone().into());
            }
            println!("reader datum {:?}", $datum);
            return Some(Ok($datum.into()));
        }
    };
}

macro_rules! atomic_datum_from_str {
    ($me:expr, $current_state:expr, $token:expr => $datum_type:ty) => {
        let datum = <$datum_type>::from_str_in_span($me.source.token_str(&$token), $token.span());
        match datum {
            Ok(datum) => {
                atomic_datum!($me, $current_state, datum);
            }
            Err(e) => {
                println!("reader error {:?}", e);
                return Some(Err(e));
            }
        }
    };
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

            println!("reader match ({:?}, {:?})", current_state, token);

            match (current_state.state(), token.kind()) {
                (_, TokenKind::Identifier) => {
                    atomic_datum_from_str!(self, current_state, token => SIdentifier);
                }
                (_, TokenKind::Boolean) => {
                    atomic_datum_from_str!(self, current_state, token => SBoolean);
                }
                (_, TokenKind::Character) => {
                    atomic_datum_from_str!(self, current_state, token => SChar);
                }
                (_, TokenKind::String) => {
                    atomic_datum_from_str!(self, current_state, token => SString);
                }
                (_, TokenKind::Numeric) => {
                    atomic_datum_from_str!(self, current_state, token => SNumber);
                }
                (_, TokenKind::OpenParenthesis) => {
                    self.state_stack.push(current_state);
                    current_state =
                        IteratorState::from(State::List).with_context(ReadContext::InList);
                }
                (State::List, TokenKind::CloseParenthesis) => {
                    let datum = SList::from(current_state.into_content());
                    current_state = self.state_stack.pop().unwrap();
                    atomic_datum!(self, current_state, datum);
                }
                (_, TokenKind::OpenVector) => {
                    self.state_stack.push(current_state);
                    current_state =
                        IteratorState::from(State::Vector).with_context(ReadContext::InList);
                }
                (State::Vector, TokenKind::CloseParenthesis) => {
                    let datum = SVector::from(current_state.into_content());
                    current_state = self.state_stack.pop().unwrap();
                    atomic_datum!(self, current_state, datum);
                }
                (_, TokenKind::BlockComment) if self.return_comments => {
                    let content = self.source.token_str(&token);
                    let content = content[2..content.len() - 2].trim().to_string();
                    return Some(Ok(SComment::Block(content).into()));
                }
                (_, TokenKind::LineComment) if self.return_comments => {
                    let content = self.source.token_str(&token);
                    let content = content[1..].trim().to_string();
                    return Some(Ok(SComment::Line(content).into()));
                }
                (_, TokenKind::DatumComment) => {
                    self.state_stack.push(current_state);
                    current_state = State::DatumComment.into();
                }
                (State::DatumComment, k) if is_datum(k) && !self.return_comments => {
                    println!(
                        "reader dropping token {:?}; you said it was commented out",
                        token
                    );
                    current_state = self.state_stack.pop().unwrap();
                }
                (_, TokenKind::DatumAssign) => {
                    let label_str = self.source.token_str(&token);
                    let label_str = &label_str[1..label_str.len() - 1];
                    let label = u16::from_str(label_str).expect("not a number?");
                    if current_state.contains_label(label) {
                        return Some(Err(invalid_datum_label(token.span())));
                    }
                    self.state_stack.push(current_state);
                    current_state = State::DatumAssign(label).into();
                }
                (_, TokenKind::DatumRef) => {
                    let label_str = self.source.token_str(&token);
                    let label_str = &label_str[1..label_str.len() - 1];
                    let label = u16::from_str(label_str).expect("not a number?");
                    return Some(current_state.get_labeled(label, token.span()));
                }
                (s, kind) => {
                    eprintln!("reader not expecting {:?} in state {:?}", kind, s);
                    return Some(Err(unexpected_token(
                        kind,
                        current_state.context(),
                        token.span(),
                    )));
                }
            }
        }
        None
    }
}

impl DatumIter<'_> {
    pub(crate) fn with_comments(self) -> Self {
        Self {
            return_comments: true,
            ..self
        }
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
