/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::input::indices::Index;
use std::ops::Range;

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
    Numeric,
    Symbol,
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
}

// ------------------------------------------------------------------------------------------------

impl Token {
    pub(crate) fn new(kind: TokenKind, start: Index, end: Index) -> Self {
        Self {
            kind,
            character_span: Span::new(start.character(), end.character()),
            byte_span: Span::new(start.byte(), end.byte() + 1),
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
    pub fn is_symbol(&self) -> bool {
        matches!(&self.kind, TokenKind::Symbol)
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
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
