/*!
Provides the crate's Error and Result types as well as helper
functions.

 */

use crate::{
    lexer::token::{Span, TokenKind},
    reader::ReadContext,
};
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
    InvalidByteVectorPrefix {
        span: Span,
    },
    InvalidEscapeString {
        span: Span,
    },
    InvalidBooleanInput {
        span: Span,
    },
    InvalidCharInput {
        span: Span,
    },
    InvalidCharName {
        name: String,
        span: Span,
    },
    InvalidUnicodeValue {
        span: Span,
    },
    InvalidStringInput {
        span: Span,
    },
    InvalidNumericInput {
        span: Span,
    },
    InvalidIdentifierInput {
        span: Span,
    },
    InvalidDirectiveInput {
        span: Span,
    },
    InvalidDatumLabel {
        span: Span,
    },
    DuplicateDatumLabel {
        label: u16,
        span: Span,
    },
    UnknownDatumLabel {
        label: u16,
        span: Span,
    },
    UnexpectedToken {
        token: TokenKind,
        context: ReadContext,
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

/// Construct an `InvalidByteVectorPrefix` Error with the provided span.
#[inline]
pub fn invalid_byte_vector_prefix(span: Span) -> Error {
    Error::InvalidByteVectorPrefix { span }
}

/// Construct an `InvalidEscapeString` Error with the provided span.
#[inline]
pub fn invalid_escape_string(span: Span) -> Error {
    Error::InvalidEscapeString { span }
}

/// Construct an `InvalidBooleanInput` Error with the provided span.
#[inline]
pub fn invalid_boolean_input(span: Span) -> Error {
    Error::InvalidBooleanInput { span }
}

/// Construct an `InvalidCharInput` Error with the provided span.
#[inline]
pub fn invalid_char_input(span: Span) -> Error {
    Error::InvalidCharInput { span }
}

/// Construct an `InvalidUnicodeValue` Error with the provided span.
#[inline]
pub fn invalid_unicode_value(span: Span) -> Error {
    Error::InvalidUnicodeValue { span }
}

/// Construct an `InvalidCharName` Error with the provided span.
#[inline]
pub fn invalid_char_name<S>(name: S, span: Span) -> Error
where
    S: Into<String>,
{
    Error::InvalidCharName {
        name: name.into(),
        span,
    }
}

/// Construct an `InvalidStringInput` Error with the provided span.
#[inline]
pub fn invalid_string_input(span: Span) -> Error {
    Error::InvalidStringInput { span }
}

/// Construct an `InvalidNumericInput` Error with the provided span.
#[inline]
pub fn invalid_numeric_input(span: Span) -> Error {
    Error::InvalidNumericInput { span }
}

/// Construct an `InvalidIdentifierInput` Error with the provided span.
#[inline]
pub fn invalid_identifier_input(span: Span) -> Error {
    Error::InvalidIdentifierInput { span }
}

/// Construct an `InvalidDirectiveInput` Error with the provided span.
#[inline]
pub fn invalid_directive_input(span: Span) -> Error {
    Error::InvalidDirectiveInput { span }
}

/// Construct an `InvalidDatumLabel` Error with the provided span.
#[inline]
pub fn invalid_datum_label(span: Span) -> Error {
    Error::InvalidDatumLabel { span }
}

/// Construct an `DuplicateDatumLabel` Error with the provided label and span.
#[inline]
pub fn duplicate_datum_label(label: u16, span: Span) -> Error {
    Error::DuplicateDatumLabel { label, span }
}

/// Construct an `UnknownDatumLabel` Error with the provided label and span.
#[inline]
pub fn unknown_datum_label(label: u16, span: Span) -> Error {
    Error::UnknownDatumLabel { label, span }
}

/// Construct an `UnexpectedToken` Error with the provided span.
#[inline]
pub fn unexpected_token(token: TokenKind, context: ReadContext, span: Span) -> Error {
    Error::UnexpectedToken {
        token,
        context,
        span,
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
                Self::InvalidByteVectorPrefix { span } => format!(
                    "Invalid or incomplete byte vector prefix; span: {:?}",
                    span.as_range()
                ),
                Self::InvalidEscapeString { span } => format!(
                    "Invalid, or badly formed, character escape string; span: {:?}",
                    span.as_range()
                ),
                Self::InvalidBooleanInput { span } => format!(
                    "Invalid, or badly formed, boolean input; span: {:?}",
                    span.as_range()
                ),
                Self::InvalidCharInput { span } => format!(
                    "Invalid, or badly formed, character input; span: {:?}",
                    span.as_range()
                ),
                Self::InvalidUnicodeValue { span } => format!(
                    "Could not convert to a valid Unicode codepoint; span: {:?}",
                    span.as_range()
                ),
                Self::InvalidCharName { name, span } => format!(
                    "Unknown character name {:?}; span: {:?}",
                    name,
                    span.as_range()
                ),
                Self::InvalidStringInput { span } => format!(
                    "Invalid, or badly formed, string input; span: {:?}",
                    span.as_range()
                ),
                Self::InvalidNumericInput { span } => format!(
                    "Invalid, or badly formed, numeric input; span: {:?}",
                    span.as_range()
                ),
                Self::InvalidIdentifierInput { span } => format!(
                    "Invalid, or badly formed, identifier input; span: {:?}",
                    span.as_range()
                ),
                Self::InvalidDirectiveInput { span } => format!(
                    "Invalid, or badly formed, directive input; span: {:?}",
                    span.as_range()
                ),
                Self::InvalidDatumLabel { span } => format!(
                    "Invalid, or badly formed, datum label assignment or reference; span: {:?}",
                    span.as_range()
                ),
                Self::DuplicateDatumLabel { label, span } => format!(
                    "The label `{}` has already been defined in this context; span: {:?}",
                    label, span
                ),
                Self::UnknownDatumLabel { label, span } => format!(
                    "The label `{}` referenced has not been defined in this context; span: {:?}",
                    label, span
                ),
                Self::UnexpectedToken {
                    token,
                    context,
                    span,
                } => format!(
                    "The token {} is not expected in a {} context; span: {:?}",
                    token, context, span
                ),
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
        matches!(self, Self::IoError { source: _ })
    }

    pub fn code(&self) -> u16 {
        match self {
            // Input errors
            Self::IoError { source: _ } => 1,
            // Lexer errors
            Self::UnclosedTokenSpecial { span: _ } => 20,
            Self::UnclosedTokenString { span: _ } => 21,
            Self::UnclosedTokenBlockComment { span: _ } => 22,
            Self::InvalidByteVectorPrefix { span: _ } => 23,
            Self::InvalidEscapeString { span: _ } => 24,
            Self::InvalidBooleanInput { span: _ } => 25,
            Self::InvalidCharInput { span: _ } => 26,
            Self::InvalidStringInput { span: _ } => 27,
            Self::InvalidNumericInput { span: _ } => 28,
            Self::InvalidIdentifierInput { span: _ } => 29,
            Self::InvalidDirectiveInput { span: _ } => 30,
            Self::InvalidDatumLabel { span: _ } => 31,
            // Reader errors
            Self::DuplicateDatumLabel { label: _, span: _ } => 41,
            Self::UnknownDatumLabel { label: _, span: _ } => 42,
            Self::InvalidUnicodeValue { span: _ } => 43,
            Self::InvalidCharName { name: _, span: _ } => 44,
            Self::UnexpectedToken {
                token: _,
                context: _,
                span: _,
            } => 59,
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
            Self::InvalidByteVectorPrefix { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Invalid or incomplete byte vector prefix")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("This is not a valid byte vector prefix"),
                    )
                    .with_note(format!("Expecting the prefix {}", "#u8(".fg(syntax)))
                    .finish(),
            ),
            Self::InvalidEscapeString { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Invalid, or badly formed, character escape string")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("Could not make this a valid mnemonic or hex escape"),
                    )
                    .with_note(
                        format!(
                            "Expecting a mnemonic in {} or hex escape in the form {}",
                            "#\\a #\\b #\\t #\\n #\\r #\\\" #\\\\ #\\|".fg(syntax),
                            "\\x⟨nn⟩;".fg(syntax)
                        ))
                    .finish(),
            ),
            Self::InvalidBooleanInput { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Invalid, or badly formed, boolean input")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("Not a valid boolean"),
                    )
                    .with_note(
                        format!(
                            "Expecting either {} or {}",
                            "#t".fg(syntax),
                            "#f".fg(syntax)
                        ))
                    .finish(),
            ),
            Self::InvalidCharInput { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Invalid, or badly formed, character input")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("Not a valid character"),
                    )
                    .with_note(
                        format!(
                            "Expecting {}, {}, or {}",
                            "#\\⟨char⟩".fg(syntax),
                             "#\\⟨name⟩".fg(syntax),
                            "#\\x⟨hh⟩;".fg(syntax)
                        ))
                    .finish(),
            ),
            Self::InvalidCharName { name, span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message(format!("Unknown character name {:?}", name))
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("Character referenced here"),
                    )
                    .finish(),
            ),
            Self::InvalidUnicodeValue { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Invalid Unicode codepoint")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("Hex value here"),
                    )
                    .with_note(
                        "Hex values must be in 32bit unsigned range, and not refer to a surrogate or private use codepoint"
                    )
                    .finish(),
            ),
            Self::InvalidStringInput { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Invalid, or badly formed, string input")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("Not a valid string"),
                    )
                    .finish(),
            ),
            Self::InvalidNumericInput { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Invalid, or badly formed, numeric input")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("Not a valid numer"),
                    )
                    .finish(),
            ),
             Self::InvalidIdentifierInput { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Invalid, or badly formed, identifier input")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("Not a valid identifier"),
                    )
                    .finish(),
            ),
            Self::InvalidDirectiveInput { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Invalid, or badly formed, directive input")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("Not a valid directive"),
                    )
                    .finish(),
            ),
            Self::InvalidDatumLabel { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Invalid, datum assignment or reference")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("This does not work"),
                    )
                    .with_note(
                        format!(
                            "Expecting a datum label assignment, {}, or reference, {}",
                            "#⟨nn⟩=⟨datum⟩".fg(syntax),
                            "#nn#".fg(syntax)
                        ))
                    .finish(),
            ),
            Self::DuplicateDatumLabel { label, span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message(format!("The label `{}` has already been defined in this context", label))
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("Label is re-defined here"),
                    )
                   .finish(),
            ),
             Self::UnknownDatumLabel { label, span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message(format!("The label `{}` referenced has not been defined in this context", label))
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("Label is referenced here"),
                    )
                   .finish(),
            ),
             Self::UnexpectedToken { token, context, span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                     .with_message(format!("Token {} is not expected in {} context", token, context))
                    .with_label(
                        Label::new(span.as_range())
                            .with_message(format!("This {}", token)),
                    )
                     // TODO: Add context span
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
