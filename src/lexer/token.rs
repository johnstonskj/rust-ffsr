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
    NumericExactnessPrefix,
    NumericRadixPrefix,
    Numeric,
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
        Self::zero()
    }
}

impl Span {
    #[inline(always)]
    pub fn new(start: usize, end: usize) -> Self {
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
    pub fn zero() -> Self {
        Self::new(0, 0)
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
    pub fn as_range(&self) -> Range<usize> {
        self.start..self.end
    }

    #[inline(always)]
    pub fn as_range_inclusive(&self) -> RangeInclusive<usize> {
        self.start..=self.end
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

    #[inline(always)]
    pub fn is_open_parenthesis(&self) -> bool {
        matches!(&self.kind, TokenKind::OpenParenthesis)
    }

    #[inline(always)]
    pub fn is_close_parenthesis(&self) -> bool {
        matches!(&self.kind, TokenKind::CloseParenthesis)
    }

    #[inline(always)]
    pub fn is_quote(&self) -> bool {
        matches!(&self.kind, TokenKind::Quote)
    }

    #[inline(always)]
    pub fn is_quasiquote(&self) -> bool {
        matches!(&self.kind, TokenKind::QuasiQuote)
    }

    #[inline(always)]
    pub fn is_unquote(&self) -> bool {
        matches!(&self.kind, TokenKind::Unquote)
    }

    #[inline(always)]
    pub fn is_unquote_splicing(&self) -> bool {
        matches!(&self.kind, TokenKind::UnquoteSplicing)
    }

    #[inline(always)]
    pub fn is_dot(&self) -> bool {
        matches!(&self.kind, TokenKind::Dot)
    }

    #[inline(always)]
    pub fn is_open_vector(&self) -> bool {
        matches!(&self.kind, TokenKind::OpenVector)
    }

    #[inline(always)]
    pub fn is_open_byte_vector(&self) -> bool {
        matches!(&self.kind, TokenKind::OpenByteVector)
    }

    #[inline(always)]
    pub fn is_identifier(&self) -> bool {
        matches!(&self.kind, TokenKind::Identifier)
    }

    #[inline(always)]
    pub fn is_character(&self) -> bool {
        matches!(&self.kind, TokenKind::Character)
    }

    #[inline(always)]
    pub fn is_string(&self) -> bool {
        matches!(&self.kind, TokenKind::String)
    }

    #[inline(always)]
    pub fn is_numeric(&self) -> bool {
        matches!(&self.kind, TokenKind::Numeric)
    }

    #[inline(always)]
    pub fn is_numeric_exactness_prefix(&self) -> bool {
        matches!(&self.kind, TokenKind::NumericExactnessPrefix)
    }

    #[inline(always)]
    pub fn is_numeric_radix_prefix(&self) -> bool {
        matches!(&self.kind, TokenKind::NumericRadixPrefix)
    }

    #[inline(always)]
    pub fn is_boolean(&self) -> bool {
        matches!(&self.kind, TokenKind::Boolean)
    }

    #[inline(always)]
    pub fn is_line_comment(&self) -> bool {
        matches!(&self.kind, TokenKind::LineComment)
    }

    #[inline(always)]
    pub fn is_block_comment(&self) -> bool {
        matches!(&self.kind, TokenKind::BlockComment)
    }

    #[inline(always)]
    pub fn is_datum_comment(&self) -> bool {
        matches!(&self.kind, TokenKind::DatumComment)
    }

    #[inline(always)]
    pub fn is_datum_assignment(&self) -> bool {
        matches!(&self.kind, TokenKind::DatumAssign)
    }

    #[inline(always)]
    pub fn is_datum_reference(&self) -> bool {
        matches!(&self.kind, TokenKind::DatumRef)
    }

    #[inline(always)]
    pub fn is_directive(&self) -> bool {
        matches!(&self.kind, TokenKind::Directive)
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::OpenParenthesis => "list start",
                Self::CloseParenthesis => "list end",
                Self::Quote => "quote",
                Self::QuasiQuote => "quasiquote",
                Self::Unquote => "unquote",
                Self::UnquoteSplicing => "unquote-splicing",
                Self::Dot => "dot",
                Self::OpenVector => "vector start",
                Self::OpenByteVector => "byte-vector start",
                Self::Identifier => "identifier",
                Self::Character => "character",
                Self::String => "string",
                Self::Numeric => "numeric",
                Self::NumericExactnessPrefix => "exactness prefix",
                Self::NumericRadixPrefix => "radix prefix",
                Self::Boolean => "boolean",
                Self::LineComment => "line comment[",
                Self::BlockComment => "block comment",
                Self::DatumComment => "datum comment",
                Self::DatumAssign => "datum assignment",
                Self::DatumRef => "datum reference",
                Self::Directive => "directive",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
