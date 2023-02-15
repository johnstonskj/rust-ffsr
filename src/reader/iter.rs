/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::error::{
    duplicate_datum_label, incomplete_byte_vector, incomplete_datum_assignment,
    incomplete_datum_comment, incomplete_list, incomplete_quasi_quote, incomplete_quote,
    incomplete_unquote, incomplete_unquote_splicing, incomplete_vector, invalid_datum_label,
    pair_missing_cdr, pair_too_many_cdr, unexpected_token, unknown_datum_label, Error,
};
use crate::lexer::iter::TokenIter;
use crate::lexer::token::{Span, TokenKind};
use crate::reader::datum::{
    Datum, SBoolean, SChar, SComment, SDirective, SIdentifier, SList, SNumber, SString,
    SimpleDatumValue,
};
use crate::reader::internals::{QuoteKind, State};
use crate::Sourced;
use std::collections::HashMap;
use std::ops::{Range, RangeInclusive};
use std::rc::Rc;
use std::str::FromStr;
use tracing::{debug, error, trace, trace_span};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct DatumIter<'a> {
    source: TokenIter<'a>,
    return_comments: bool,
    state_stack: Vec<State>,
    ref_table: HashMap<u16, Datum>,
}

#[derive(Debug)]
pub struct SyntaxDatum<'a> {
    source: TokenIter<'a>,
    span: Span,
    datum: Datum,
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

macro_rules! push_new_state {
    ($me:expr, $current_state:expr, $new_state:expr) => {
        $me.state_stack.push($current_state);
        $current_state = $new_state;
        trace!(current_state = ?$current_state, "pushed new state");
    };
}

macro_rules! pop_state {
    ($me:expr, $current_state:expr) => {
        $current_state = $me.state_stack.pop().unwrap();
        trace!(current_state = ?$current_state, "popped state");
    };
}

macro_rules! handle_error {
    ($fallible:expr) => {
        match $fallible {
            Ok(ok) => ok,
            Err(e) => {
                error!("handle_error: {}", e);
                return Some(Err(e));
            }
        }
    };
}
macro_rules! handle_datum {
    ($self:expr, $current_state:expr, $datum:expr) => {
        let (datum, state) = match $self.handle_datum($current_state, $datum) {
            Ok(result) => result,
            Err(e) => {
                return Some(Err(e));
            }
        };
        $current_state = state;
        if let Some(datum) = datum {
            return Some(Ok(datum));
        }
    };
}

macro_rules! handle_datum_from_str {
    ($datum_type:ty, $token:expr => $me:expr, $current_state:expr) => {
        let datum = handle_error!(<$datum_type>::from_str_in_span(
            $me.source.token_str(&$token),
            $token.span()
        ));
        handle_datum!($me, $current_state, Datum::from(datum.clone()));
    };
}

macro_rules! return_error {
    ($span:expr => $error_fn:expr) => {
        return_error!("", $span => $error_fn);
    };
    ($message:expr, $span:expr => $error_fn:expr) => {
        let err = $error_fn($span);
        error!("{}{:?}", $message, err);
        return Some(err);
    };
    ($span:expr => $error_fn:expr, $second:expr ) => {
        return_error!("", $span => $error_fn, $second );
    };
    ($message:expr, $span:expr => $error_fn:expr, $second:expr ) => {
        let err = $error_fn($span, $second);
        error!("{}{:?}", $message, err);
        return Some(err);
    };
}

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
            ref_table: Default::default(),
        }
    }
}

impl Iterator for DatumIter<'_> {
    type Item = Result<Datum, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let _span = trace_span!("next-datum", ?self.state_stack);
        let _scope = _span.enter();

        let mut current_state = if let Some(state) = self.state_stack.pop() {
            state
        } else {
            State::default()
        };

        while let Some(token) = self.source.next() {
            let token = match token {
                Ok(t) => t,
                Err(e) => {
                    return Some(Err(e));
                }
            };

            trace!(current_state = ?current_state, token = ?token, "match");

            match (&current_state, token.kind()) {
                // --------------------------------------------------------------------------------
                (_, TokenKind::Quote) => {
                    push_new_state!(
                        self,
                        current_state,
                        State::Quote(token.span(), QuoteKind::Quote)
                    );
                }
                (_, TokenKind::QuasiQuote) => {
                    push_new_state!(
                        self,
                        current_state,
                        State::Quote(token.span(), QuoteKind::QuasiQuote)
                    );
                }
                (_, TokenKind::Unquote) => {
                    push_new_state!(
                        self,
                        current_state,
                        State::Quote(token.span(), QuoteKind::Unquote)
                    );
                }
                (_, TokenKind::UnquoteSplicing) => {
                    push_new_state!(
                        self,
                        current_state,
                        State::Quote(token.span(), QuoteKind::UnquoteSplicing)
                    );
                }
                // --------------------------------------------------------------------------------
                (_, TokenKind::Identifier) => {
                    handle_datum_from_str!(SIdentifier, token => self, current_state);
                }
                (_, TokenKind::Boolean) => {
                    handle_datum_from_str!(SBoolean, token => self, current_state);
                }
                (_, TokenKind::Character) => {
                    handle_datum_from_str!(SChar, token => self, current_state);
                }
                (_, TokenKind::String) => {
                    handle_datum_from_str!(SString, token => self, current_state);
                }
                (_, TokenKind::Number) => {
                    handle_datum_from_str!(SNumber, token => self, current_state);
                }
                (_, TokenKind::Directive) => {
                    handle_datum_from_str!(SDirective, token => self, current_state);
                }
                // --------------------------------------------------------------------------------
                (State::FastForward(_), TokenKind::OpenParenthesis) => {
                    trace!(token = ?token, "fast forwarded");
                    let err = current_state.into_error();
                    pop_state!(self, current_state);
                    return Some(Err(err));
                }
                // --------------------------------------------------------------------------------
                (_, TokenKind::OpenParenthesis) => {
                    push_new_state!(
                        self,
                        current_state,
                        State::List(token.span(), SList::empty())
                    );
                }
                (State::List(_, _), TokenKind::CloseParenthesis) => {
                    let datum = current_state.into_list();
                    pop_state!(self, current_state);
                    handle_datum!(self, current_state, datum.into());
                }
                (State::List(_, _), TokenKind::Dot) => {
                    push_new_state!(self, current_state, State::Dot(token.span(), None));
                }
                (State::Dot(span, None), TokenKind::CloseParenthesis) => {
                    return_error!(*span => pair_missing_cdr);
                }
                (State::Dot(span, Some(cdr)), TokenKind::CloseParenthesis) => {
                    let span = *span;
                    let cdr = cdr.clone();
                    pop_state!(self, current_state);
                    let mut datum = current_state.into_list();
                    pop_state!(self, current_state);
                    handle_error!(datum.append_improper(cdr.into(), Some(span)));
                    handle_datum!(self, current_state, datum.into());
                }
                (State::Dot(span, _), TokenKind::Dot) => {
                    return_error!(*span => pair_too_many_cdr);
                }
                // --------------------------------------------------------------------------------
                (_, TokenKind::OpenVector) => {
                    push_new_state!(
                        self,
                        current_state,
                        State::Vector(token.span(), Default::default())
                    );
                }
                (State::Vector(_, _), TokenKind::CloseParenthesis) => {
                    let datum = current_state.into_vector();
                    pop_state!(self, current_state);
                    handle_datum!(self, current_state, datum.into());
                }
                // --------------------------------------------------------------------------------
                (_, TokenKind::OpenByteVector) => {
                    push_new_state!(
                        self,
                        current_state,
                        State::ByteVector(token.span(), Default::default())
                    );
                }
                (State::ByteVector(_, _), TokenKind::CloseParenthesis) => {
                    let datum = current_state.into_byte_vector();
                    pop_state!(self, current_state);
                    handle_datum!(self, current_state, datum.into());
                }
                // --------------------------------------------------------------------------------
                (_, TokenKind::BlockComment) if self.return_comments => {
                    let content = self.source.token_str(&token);
                    let content = content[2..content.len() - 2].trim().to_string();
                    return Some(Ok(SComment::Block(content).into()));
                }
                (_, TokenKind::LineComment) if self.return_comments => {
                    let content = self.source.token_str(&token);
                    return Some(Ok(SComment::Line(content.into()).into()));
                }
                (_, TokenKind::BlockComment | TokenKind::LineComment) => {}
                // --------------------------------------------------------------------------------
                (_, TokenKind::DatumComment) => {
                    push_new_state!(self, current_state, State::DatumComment(token.span()));
                }
                (State::DatumComment(_), k) if is_datum(k) && !self.return_comments => {
                    // this saves creating and then discarding the datum
                    pop_state!(self, current_state);
                }
                // --------------------------------------------------------------------------------
                (_, TokenKind::DatumAssign) => {
                    let label_str = self.source.token_str(&token);
                    let label_str = &label_str[1..label_str.len() - 1];
                    let label = match u16::from_str(label_str) {
                        Ok(label) => label,
                        Err(e) => {
                            error!("error making label from {label_str:?}, error: {e}");
                            return_error!(token.span() => invalid_datum_label);
                        }
                    };
                    if self.ref_table.contains_key(&label) {
                        return_error!(token.span() => duplicate_datum_label, label);
                    }
                    push_new_state!(self, current_state, State::DatumAssign(token.span(), label));
                }
                (_, TokenKind::DatumRef) => {
                    let label_str = self.source.token_str(&token);
                    let label_str = &label_str[1..label_str.len() - 1];
                    let label = match u16::from_str(label_str) {
                        Ok(label) => label,
                        Err(e) => {
                            error!("error making label from {label_str:?}, error: {e}");
                            return_error!(token.span() => invalid_datum_label);
                        }
                    };
                    if let Some(datum) = self.ref_table.get(&label).cloned() {
                        handle_datum!(self, current_state, datum);
                    } else {
                        return_error!(token.span() => unknown_datum_label, label);
                    }
                }
                // --------------------------------------------------------------------------------
                (state, kind) => {
                    debug!(state = ?state, token = ?token, "unexpected");
                    return_error!(token.span() => unexpected_token, kind);
                }
            }
        }
        match current_state {
            State::List(span, _) => Some(incomplete_list(span)),
            State::Vector(span, _) => Some(incomplete_vector(span)),
            State::ByteVector(span, _) => Some(incomplete_byte_vector(span)),
            State::DatumAssign(span, label) => Some(incomplete_datum_assignment(span, label)),
            State::DatumComment(span) => Some(incomplete_datum_comment(span)),
            State::Quote(span, kind) => match kind {
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

    fn handle_datum(
        &mut self,
        mut current_state: State,
        mut datum: Datum,
    ) -> Result<(Option<Datum>, State), Error> {
        // do this before deciding what to do with the datum
        if let State::DatumAssign(_, label) = current_state {
            trace!(datum = ?datum, "assigning datum to label {label:?}");
            pop_state!(self, current_state);
            self.ref_table.insert(label, datum.clone());
        }

        while let State::Quote(_, q) = current_state {
            datum = match q {
                QuoteKind::Quote => datum.quote(),
                QuoteKind::QuasiQuote => datum.quasiquote(),
                QuoteKind::Unquote => datum.unquote(),
                QuoteKind::UnquoteSplicing => datum.unquote_splicing(),
            };
            trace!(datum = ?datum, "quoted datum");
            pop_state!(self, current_state);
        }

        match current_state {
            State::DatumComment(_) => {
                trace!(datum = ?datum, "ignoring datum");
                pop_state!(self, current_state);
                Ok((None, current_state))
            }
            State::Dot(span, None) => {
                trace!(datum = ?datum, "adding datum to cdr of pair");
                Ok((None, State::Dot(span, Some(datum))))
            }
            State::Dot(span, Some(_)) => pair_too_many_cdr(span),
            State::List(span, ref mut pair) => {
                trace!(datum = ?datum, "adding datum to open list");
                pair.append(Rc::new(datum), Some(span))?;
                Ok((None, current_state))
            }
            State::Vector(_, ref mut vector) => {
                trace!(datum = ?datum, "adding datum to open vector");
                vector.append(datum);
                Ok((None, current_state))
            }
            State::ByteVector(span, ref mut byte_vector) => {
                trace!(datum = ?datum, "adding datum to open byte vector");
                byte_vector.try_append_datum(datum, span)?;
                Ok((None, current_state))
            }
            _ => {
                trace!(datum = ?datum, "return datum");
                Ok((Some(datum), current_state))
            }
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
