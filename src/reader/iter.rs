/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::error::{
    duplicate_datum_label, incomplete_byte_vector, incomplete_datum_comment, incomplete_list,
    incomplete_quasi_quote, incomplete_quote, incomplete_unquote, incomplete_unquote_splicing,
    incomplete_vector, invalid_datum_label, unexpected_token, unknown_datum_label, Error,
};
use crate::lexer::iter::TokenIter;
use crate::lexer::token::{Span, TokenKind};
use crate::reader::datum::SNumber;
use crate::reader::{
    datum::{
        Datum, SBoolean, SChar, SComment, SIdentifier, SList, SString, SVector, SimpleDatumValue,
    },
    internals::{IteratorState, QuoteKind, State},
};
use crate::Sourced;
use std::collections::HashMap;
use std::ops::{Range, RangeInclusive};
use std::str::FromStr;
use tracing::{error, trace, trace_span};

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

#[derive(Debug)]
pub struct SyntaxDatum<'a> {
    source: TokenIter<'a>,
    span: Span,
    datum: Datum,
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

impl From<SyntaxDatum<'_>> for Datum {
    fn from(v: SyntaxDatum<'_>) -> Self {
        v.datum
    }
}

impl Sourced for SyntaxDatum<'_> {
    #[inline(always)]
    fn source_id(&self) -> &crate::SourceId {
        self.source.source_id()
    }

    #[inline(always)]
    fn source_str(&self) -> &str {
        self.source.source_str()
    }
}

impl<'a> SyntaxDatum<'a> {
    #[inline(always)]
    pub fn new(source: TokenIter<'a>, span: Span, datum: Datum) -> Self {
        Self {
            source,
            span,
            datum,
        }
    }

    #[inline(always)]
    pub fn start(&self) -> usize {
        self.span.start()
    }

    #[inline(always)]
    pub fn end(&self) -> usize {
        self.span.end()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.span.is_empty()
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.span.len()
    }

    #[inline(always)]
    pub fn as_range(&self) -> Range<usize> {
        self.span.as_range()
    }

    #[inline(always)]
    pub fn as_range_inclusive(&self) -> RangeInclusive<usize> {
        self.span.as_range_inclusive()
    }

    #[inline(always)]
    pub fn datum(&self) -> &Datum {
        &self.datum
    }
}

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

macro_rules! push_new_state {
    ($me:expr, $current_state:expr, $new_state:expr) => {
        $me.state_stack.push($current_state);
        $current_state = IteratorState::from($new_state);
        trace!("pushed new state, current: {:?}", $current_state);
    };
}

macro_rules! pop_state {
    ($me:expr, $current_state:expr) => {
        $current_state = $me.state_stack.pop().unwrap();
        trace!("popped state, current: {:?}", $current_state);
    };
}

macro_rules! atomic_datum {
    ($me:expr, $current_state:expr, $ref_table:expr, $datum:expr) => {
        // do this before deciding what to do with the datum
        if let State::DatumAssign(label) = $current_state.state() {
            trace!("assigning datum {:?} to label {label:?}", $datum);
            pop_state!($me, $current_state);
            $ref_table.insert(label, $datum.clone().into());
        }
        let mut datum = Datum::from($datum);
        while let State::Quote(q, _span) = $current_state.state() {
            datum = match q {
                QuoteKind::Quote => datum.quote(),
                QuoteKind::QuasiQuote => datum.quasiquote(),
                QuoteKind::Unquote => datum.unquote(),
                QuoteKind::UnquoteSplicing => datum.unquote_splicing(),
            };
            trace!("quoted datum {:?}", datum);
            pop_state!($me, $current_state);
        }
        match $current_state.state() {
            State::DatumComment(_) => {
                trace!("ignoring datum {datum:?}");
                pop_state!($me, $current_state);
            }
            State::List(_) | State::Vector(_) | State::ByteVector(_) => {
                trace!("adding datum {datum:?} to open list");
                $current_state.add_content(datum);
            }
            _ => {
                trace!("return datum {datum:?}");
                return Some(Ok(datum));
            }
        }
    };
}

macro_rules! atomic_datum_from_str {
    ($me:expr, $current_state:expr, $ref_table:expr, $token:expr => $datum_type:ty) => {
        let datum = <$datum_type>::from_str_in_span($me.source.token_str(&$token), $token.span());
        match datum {
            Ok(datum) => {
                atomic_datum!($me, $current_state, $ref_table, datum);
            }
            Err(e) => {
                error!("return error {e:?}");
                return Some(Err(e));
            }
        }
    };
}

impl Iterator for DatumIter<'_> {
    type Item = Result<Datum, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let _span = trace_span!("next-datum", ?self.state_stack);
        let _scope = _span.enter();

        let mut ref_table: HashMap<u16, Datum> = Default::default();
        let mut current_state = if let Some(state) = self.state_stack.pop() {
            state
        } else {
            IteratorState::default()
        };

        while let Some(token) = self.source.next() {
            let token = match token {
                Ok(t) => t,
                Err(e) => {
                    return Some(Err(e));
                }
            };

            trace!("match ({current_state:?}, {token:?})");

            match (current_state.state(), token.kind()) {
                (_, TokenKind::Quote) => {
                    push_new_state!(
                        self,
                        current_state,
                        State::Quote(QuoteKind::Quote, token.span())
                    );
                }
                (_, TokenKind::QuasiQuote) => {
                    push_new_state!(
                        self,
                        current_state,
                        State::Quote(QuoteKind::QuasiQuote, token.span())
                    );
                }
                (_, TokenKind::Unquote) => {
                    push_new_state!(
                        self,
                        current_state,
                        State::Quote(QuoteKind::Unquote, token.span())
                    );
                }
                (_, TokenKind::UnquoteSplicing) => {
                    push_new_state!(
                        self,
                        current_state,
                        State::Quote(QuoteKind::UnquoteSplicing, token.span())
                    );
                }
                (_, TokenKind::OpenParenthesis) => {
                    push_new_state!(self, current_state, State::List(token.span()));
                }
                (State::List(_), TokenKind::CloseParenthesis) => {
                    trace!("closing an open list");
                    let datum = SList::from(current_state.into_content());
                    pop_state!(self, current_state);
                    atomic_datum!(self, current_state, ref_table, datum);
                }
                (_, TokenKind::OpenVector) => {
                    push_new_state!(self, current_state, State::Vector(token.span()));
                }
                (State::Vector(_), TokenKind::CloseParenthesis) => {
                    trace!("closing an open vector");
                    let datum = SVector::from(current_state.into_content());
                    pop_state!(self, current_state);
                    atomic_datum!(self, current_state, ref_table, datum);
                }
                (_, TokenKind::OpenByteVector) => {
                    push_new_state!(self, current_state, State::ByteVector(token.span()));
                }
                (State::ByteVector(_), TokenKind::CloseParenthesis) => {
                    trace!("closing an open byte vector");
                    //let datum = SByteVector::from(current_state.into_content());
                    // current_state = self.state_stack.pop().unwrap();
                    //     atomic_datum!(self, current_state, ref_table, datum);
                    unimplemented!()
                }
                (_, TokenKind::Identifier) => {
                    atomic_datum_from_str!(self, current_state, ref_table, token => SIdentifier);
                }
                (_, TokenKind::Boolean) => {
                    atomic_datum_from_str!(self, current_state, ref_table, token => SBoolean);
                }
                (_, TokenKind::Character) => {
                    atomic_datum_from_str!(self, current_state, ref_table, token => SChar);
                }
                (_, TokenKind::String) => {
                    atomic_datum_from_str!(self, current_state, ref_table, token => SString);
                }
                (_, TokenKind::Number) => {
                    atomic_datum_from_str!(self, current_state, ref_table, token => SNumber);
                }
                (_, TokenKind::BlockComment) if self.return_comments => {
                    let content = self.source.token_str(&token);
                    let content = content[2..content.len() - 2].trim().to_string();
                    return Some(Ok(SComment::Block(content).into()));
                }
                (_, TokenKind::LineComment) if self.return_comments => {
                    let content = self.source.token_str(&token);
                    // TODO: remove *all* prefix ';'
                    let content = content[1..].trim().to_string();
                    return Some(Ok(SComment::Line(content).into()));
                }
                (_, TokenKind::DatumComment) => {
                    push_new_state!(self, current_state, State::DatumComment(token.span()));
                }
                (State::DatumComment(_), k) if is_datum(k) && !self.return_comments => {
                    trace!("dropping token {token:?}; you said it was commented out");
                    pop_state!(self, current_state);
                }
                (_, TokenKind::DatumAssign) => {
                    let label_str = self.source.token_str(&token);
                    let label_str = &label_str[1..label_str.len() - 1];
                    let label = match u16::from_str(label_str) {
                        Ok(label) => label,
                        Err(e) => {
                            error!("label error {}", e);
                            return Some(invalid_datum_label(token.span()));
                        }
                    };
                    if ref_table.contains_key(&label) {
                        error!("Datum label {label} already in use");
                        return Some(duplicate_datum_label(token.span(), label));
                    }
                    push_new_state!(self, current_state, State::DatumAssign(label));
                }
                (_, TokenKind::DatumRef) => {
                    let label_str = self.source.token_str(&token);
                    let label_str = &label_str[1..label_str.len() - 1];
                    let label = match u16::from_str(label_str) {
                        Ok(label) => label,
                        Err(e) => {
                            error!("label error {}", e);
                            return Some(invalid_datum_label(token.span()));
                        }
                    };
                    if let Some(datum) = ref_table.get(&label).cloned() {
                        atomic_datum!(self, current_state, ref_table, datum);
                    } else {
                        error!("No existing datum with label {label}");
                        return Some(unknown_datum_label(token.span(), label));
                    }
                }
                (_, TokenKind::LineComment | TokenKind::BlockComment) => {}
                (s, kind) => {
                    error!("not expecting {kind:?} in state {s:?}");
                    return Some(unexpected_token(token.span(), kind));
                }
            }
        }
        match current_state.state() {
            State::List(span) => Some(incomplete_list(span)),
            State::Vector(span) => Some(incomplete_vector(span)),
            State::ByteVector(span) => Some(incomplete_byte_vector(span)),
            State::DatumComment(span) => Some(incomplete_datum_comment(span)),
            State::Quote(kind, span) => match kind {
                QuoteKind::Quote => Some(incomplete_quote(span)),
                QuoteKind::QuasiQuote => Some(incomplete_quasi_quote(span)),
                QuoteKind::Unquote => Some(incomplete_unquote(span)),
                QuoteKind::UnquoteSplicing => Some(incomplete_unquote_splicing(span)),
            },
            _ => None,
        }
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
    matches!(
        token,
        TokenKind::Identifier
            | TokenKind::Boolean
            | TokenKind::Character
            | TokenKind::Number
            | TokenKind::String
    )
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
