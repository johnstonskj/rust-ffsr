/*!
Provides the crate's Error and Result types as well as helper
functions.

 */

use crate::lexer::token::{Span, TokenKind};
use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
use std::fmt::{Debug, Display};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The Error type for this crate.
///
#[derive(Debug)]
pub enum Error {
    /// An error was signaled by the standard library I/O functions.
    IoError {
        source: std::io::Error,
    },
    UnclosedTokenSpecial {
        span: Span,
    },
    UnclosedTokenString {
        span: Span,
    },
    UnclosedTokenBlockComment {
        span: Span,
    },
    UnclosedTokenOpenByteVector {
        span: Span,
    },
}

///
/// A Result type that specifically uses this crate's Error.
///
pub type Result<T> = std::result::Result<Error, T>;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

/// Construct an `Error` from the provided source.
#[inline]
pub fn io_error(source: std::io::Error) -> Error {
    Error::IoError { source }
}

/// Construct an `UnclosedToken*` Error with the provided span.
#[inline]
pub fn unclosed_special(span: Span) -> Error {
    Error::UnclosedTokenSpecial { span }
}

/// Construct an `UnclosedToken*` Error with the provided span.
#[inline]
pub fn unclosed_string(span: Span) -> Error {
    Error::UnclosedTokenString { span }
}

/// Construct an `UnclosedToken*` Error with the provided span.
#[inline]
pub fn unclosed_block_comment(span: Span) -> Error {
    Error::UnclosedTokenBlockComment { span }
}

/// Construct an `UnclosedToken*` Error with the provided span.
#[inline]
pub fn unclosed_byte_vector(span: Span) -> Error {
    Error::UnclosedTokenOpenByteVector { span }
}

/// Construct an `UnclosedToken*` Error with the provided token type and span.
#[inline]
pub fn unclosed_token(kind: TokenKind, span: Span) -> Error {
    match kind {
        TokenKind::String => unclosed_string(span),
        TokenKind::BlockComment => unclosed_block_comment(span),
        TokenKind::OpenByteVector => unclosed_byte_vector(span),
        _ => panic!(),
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::IoError { source } => format!("An I/O error occurred; source: {}", source),
                Self::UnclosedTokenSpecial { span } => unclosed_token_string("special", span),
                Self::UnclosedTokenString { span } => unclosed_token_string("string", span),
                Self::UnclosedTokenBlockComment { span } =>
                    unclosed_token_string("block-comment", span),
                Self::UnclosedTokenOpenByteVector { span } =>
                    unclosed_token_string("byte-vector", span),
            }
        )
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IoError { source } => Some(source),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(source: std::io::Error) -> Self {
        io_error(source)
    }
}

impl Error {
    pub fn stop(&self) -> bool {
        if let Self::IoError { source: _ } = self {
            true
        } else {
            false
        }
    }

    pub fn code(&self) -> u16 {
        match self {
            Self::IoError { source: _ } => 1,
            Self::UnclosedTokenSpecial { span: _ } => 20,
            Self::UnclosedTokenString { span: _ } => 21,
            Self::UnclosedTokenBlockComment { span: _ } => 22,
            Self::UnclosedTokenOpenByteVector { span: _ } => 23,
        }
    }
    pub fn report(&self) -> Option<Report> {
        let syntax = Color::Fixed(81);

        match self {
            Self::UnclosedTokenSpecial { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Incomplete special form")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("This is not a valid/complete special form"),
                    )
                    .with_note("Expecting a directive, boolean, character, numeric prefix, vector, or block/datum comment")
                    .finish(),
            ),
            Self::UnclosedTokenString { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Incomplete string")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("String starts here"),
                    ).with_label(
                        Label::new(span.as_range())
                            .with_message("Reported here"),
                    )
                    .with_note(format!("Expecting a closing {} character", "#\\\"".fg(syntax)))
                    .finish(),
            ),
            Self::UnclosedTokenBlockComment { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Incomplete block-comment")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("Comment starts here"),
                    )
                    .with_note(format!("Expecting a closing {}", "|#".fg(syntax)))
                    .finish(),
            ),
            Self::UnclosedTokenOpenByteVector { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Incomplete byte vector form")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("This is not a valid byte vector"),
                    )
                    .with_note(format!("Expecting the prefix {}", "#u8(".fg(syntax)))
                    .finish(),
            ),
            _ => None,
        }
    }

    pub fn print<S>(&self, source: S)
    where
        S: AsRef<str>,
    {
        if let Some(report) = self.report() {
            report
                .print(Source::from(source.as_ref()))
                .expect("Could not write error as report");
        } else {
            println!("{}", self);
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

pub fn unclosed_token_string(kind: &str, span: &Span) -> String {
    format!("Token '{}' not closed, span: {:?}", kind, span.as_range())
}
