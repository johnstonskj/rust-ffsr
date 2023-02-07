/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::input::indices::{CharIndex, Index};
use std::{
    fmt::Display,
    ops::{Range, RangeInclusive},
};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Span {
    start: usize,
    end: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Token {
    kind: TokenKind,
    character_span: Span,
    byte_span: Span,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TokenKind {
    OpenParenthesis,
    CloseParenthesis,
    Quote,
    QuasiQuote,
    Unquote,
    UnquoteSplicing,
    Dot,
    OpenVector,
    OpenByteVector,
    Identifier,
    Character,
    String,
    Number,
    Boolean,
    LineComment,
    BlockComment,
    DatumComment,
    DatumAssign,
    DatumRef,
    Directive,
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

impl Default for Span {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.as_range())
    }
}

impl From<usize> for Span {
    fn from(v: usize) -> Self {
        Self::new(v, v)
    }
}

impl Span {
    #[inline(always)]
    pub fn new(start: usize, end: usize) -> Self {
        assert!(start <= end);
        Self { start, end }
    }

    #[inline(always)]
    pub fn new_byte_span_from(s: &str) -> Self {
        Self::new(0, s.len() - 1)
    }

    #[inline(always)]
    pub fn new_char_span_from(s: &str) -> Self {
        Self::new(0, s.chars().count())
    }

    #[inline(always)]
    pub fn with_start_from(&self, other: Self) -> Self {
        Self {
            start: other.start,
            end: self.end,
        }
    }

    #[inline(always)]
    pub fn with_end_from(&self, other: Self) -> Self {
        Self {
            start: self.start,
            end: other.end,
        }
    }

    #[inline(always)]
    pub fn start(&self) -> usize {
        self.start
    }

    #[inline(always)]
    pub fn end(&self) -> usize {
        self.end
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    #[inline(always)]
    pub fn as_range(&self) -> Range<usize> {
        self.start..self.end
    }

    #[inline(always)]
    pub fn as_range_inclusive(&self) -> RangeInclusive<usize> {
        self.start..=self.end
    }

    #[inline(always)]
    pub fn as_start_range(&self) -> Range<usize> {
        self.start..self.start
    }

    #[inline(always)]
    pub fn as_start_range_inclusive(&self) -> RangeInclusive<usize> {
        self.start..=self.start
    }

    #[inline(always)]
    pub fn as_end_range(&self) -> Range<usize> {
        self.end..self.end
    }

    #[inline(always)]
    pub fn as_end_range_inclusive(&self) -> RangeInclusive<usize> {
        self.end..=self.end
    }
}

// ------------------------------------------------------------------------------------------------

impl Token {
    pub(crate) fn new(kind: TokenKind, start: Index, end: CharIndex) -> Self {
        Self {
            kind,
            character_span: Span::new(start.character(), end.index().character()),
            byte_span: Span::new(start.byte(), end.index().byte()),
        }
    }

    pub(crate) fn new_and_add_char(kind: TokenKind, start: Index, end: CharIndex) -> Self {
        Self {
            kind,
            character_span: Span::new(start.character(), end.index().character()),
            byte_span: Span::new(start.byte(), end.index().byte() + end.char_width()),
        }
    }

    #[inline(always)]
    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    #[inline(always)]
    pub fn span(&self) -> Span {
        self.character_span
    }

    #[inline(always)]
    pub(super) fn byte_span(&self) -> Span {
        self.byte_span
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.character_span.start == self.character_span.end
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.character_span.end - self.character_span.start
    }

    #[inline(always)]
    pub fn start(&self) -> usize {
        self.character_span.start
    }

    #[inline(always)]
    pub fn end(&self) -> usize {
        self.character_span.end
    }

    is_variant!(
        kind,
        (open_parenthesis, TokenKind::OpenParenthesis),
        (close_parenthesis, TokenKind::CloseParenthesis),
        (quote, TokenKind::Quote),
        (quasi_quote, TokenKind::QuasiQuote),
        (unquote, TokenKind::Unquote),
        (unquote_splicing, TokenKind::UnquoteSplicing),
        (dot, TokenKind::Dot),
        (open_vector, TokenKind::OpenVector),
        (open_byte_vector, TokenKind::OpenByteVector),
        (identifier, TokenKind::Identifier),
        (character, TokenKind::Character),
        (string, TokenKind::String),
        (number, TokenKind::Number),
        (boolean, TokenKind::Boolean),
        (line_comment, TokenKind::LineComment),
        (block_comment, TokenKind::BlockComment),
        (datum_comment, TokenKind::DatumComment),
        (datum_assignment, TokenKind::DatumAssign),
        (datum_reference, TokenKind::DatumRef),
        (directive, TokenKind::Directive)
    );
}

// ------------------------------------------------------------------------------------------------

impl_display_into_str!(
    TokenKind,
    (TokenKind::OpenParenthesis => "list start"),
    (TokenKind::CloseParenthesis => "list end"),
    (TokenKind::Quote => "quote"),
    (TokenKind::QuasiQuote => "quasiquote"),
    (TokenKind::Unquote => "unquote"),
    (TokenKind::UnquoteSplicing => "unquote-splicing"),
    (TokenKind::Dot => "dot"),
    (TokenKind::OpenVector => "vector start"),
    (TokenKind::OpenByteVector => "byte-vector start"),
    (TokenKind::Identifier => "identifier"),
    (TokenKind::Character => "character"),
    (TokenKind::String => "string"),
    (TokenKind::Number => "number"),
    (TokenKind::Boolean => "boolean"),
    (TokenKind::LineComment => "line comment"),
    (TokenKind::BlockComment => "block comment"),
    (TokenKind::DatumComment => "datum comment"),
    (TokenKind::DatumAssign => "datum assignment"),
    (TokenKind::DatumRef => "datum reference"),
    (TokenKind::Directive => "directive")
);

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
