use crate::error::Error;
use crate::lexer::token::Span;
use crate::reader::datum::{Datum, SByteVector, SList, SVector};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub(crate) enum State {
    #[default]
    TopLevel,
    Quote(Span, QuoteKind),
    DatumComment(Span),
    DatumAssign(Span, u16),
    List(Span, SList),
    Dot(Span, Option<Datum>),
    Vector(Span, SVector),
    ByteVector(Span, SByteVector),
    #[allow(dead_code)]
    FastForward(Error),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum QuoteKind {
    Quote,
    QuasiQuote,
    Unquote,
    UnquoteSplicing,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl State {
    #[inline(always)]
    pub(crate) fn into_list(self) -> SList {
        match self {
            State::List(_, list) => list,
            _ => panic!(),
        }
    }

    #[inline(always)]
    pub(crate) fn into_vector(self) -> SVector {
        match self {
            State::Vector(_, vector) => vector,
            _ => panic!(),
        }
    }

    #[inline(always)]
    pub(crate) fn into_byte_vector(self) -> SByteVector {
        match self {
            State::ByteVector(_, byte_vector) => byte_vector,
            _ => panic!(),
        }
    }

    #[inline(always)]
    pub(crate) fn into_error(self) -> Error {
        match self {
            State::FastForward(err) => err,
            _ => panic!(),
        }
    }
}
