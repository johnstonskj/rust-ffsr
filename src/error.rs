/*!
Provides the crate's Error and Result types as well as helper
functions.

 */

use crate::lexer::token::{Span, TokenKind};
use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
use std::fmt::{Debug, Display};
use tracing::error;

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
    // --------------------------------------------------------------
    // Special Forms
    // --------------------------------------------------------------
    IncompleteSpecial {
        span: Span,
    },
    // --------------------------------------------------------------
    // Directives
    // --------------------------------------------------------------
    InvalidDirectiveInput {
        span: Span,
    },
    UnknownDirectiveName {
        span: Span,
        name: String,
    },
    // --------------------------------------------------------------
    // Datum Labels
    // --------------------------------------------------------------
    InvalidDatumLabel {
        span: Span,
    },
    DuplicateDatumLabel {
        span: Span,
        label: u16,
    },
    UnknownDatumLabel {
        span: Span,
        label: u16,
    },
    // --------------------------------------------------------------
    // Identifiers
    // --------------------------------------------------------------
    IncompleteIdentifier {
        span: Span,
    },
    InvalidIdentifierMnemonicEscape {
        span: Span,
    },
    InvalidIdentifierHexEscape {
        span: Span,
    },
    InvalidIdentifierInput {
        span: Span,
        source: Option<Box<Error>>,
    },
    // --------------------------------------------------------------
    // Booleans
    // --------------------------------------------------------------
    InvalidBooleanInput {
        span: Span,
    },
    // --------------------------------------------------------------
    // Characters
    // --------------------------------------------------------------
    UnknownCharName {
        span: Span,
        name: String,
    },
    InvalidUnicodeValue {
        span: Span,
    },
    InvalidCharInput {
        span: Span,
        source: Option<Box<Error>>,
    },
    // --------------------------------------------------------------
    // Strings
    // --------------------------------------------------------------
    IncompleteString {
        span: Span,
    },
    InvalidStringMnemonicEscape {
        span: Span,
    },
    InvalidStringHexEscape {
        span: Span,
    },
    InvalidStringInput {
        span: Span,
        source: Option<Box<Error>>,
    },
    // --------------------------------------------------------------
    // Numbers
    // --------------------------------------------------------------
    InvalidNumericInput {
        span: Span,
        source: Option<Box<Error>>,
    },
    // --------------------------------------------------------------
    // Lists
    // --------------------------------------------------------------
    IncompleteList {
        span: Span,
    },
    // --------------------------------------------------------------
    // Vectors
    // --------------------------------------------------------------
    IncompleteVector {
        span: Span,
    },
    // --------------------------------------------------------------
    // Byte Vectors
    // --------------------------------------------------------------
    InvalidByteVectorPrefix {
        span: Span,
    },
    IncompleteByteVector {
        span: Span,
    },
    // --------------------------------------------------------------
    // Quotes
    // --------------------------------------------------------------
    IncompleteQuote {
        span: Span,
    },
    IncompleteQuasiQuote {
        span: Span,
    },
    IncompleteUnquote {
        span: Span,
    },
    IncompleteUnquoteSplicing {
        span: Span,
    },
    // --------------------------------------------------------------
    // Comments
    // --------------------------------------------------------------
    IncompleteBlockComment {
        span: Span,
    },
    IncompleteDatumComment {
        span: Span,
    },
    // --------------------------------------------------------------
    // Unexpected
    // --------------------------------------------------------------
    UnexpectedToken {
        span: Span,
        token: TokenKind,
        within: Option<Span>,
    },
}

///
/// A Result type that specifically uses this crate's Error.
///
pub type Result<T> = std::result::Result<T, Error>;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

/// Construct an `Error` from the provided source.
#[inline]
pub fn io_error(source: std::io::Error) -> Error {
    Error::IoError { source }
}

// --------------------------------------------------------------
// Special Forms
// --------------------------------------------------------------

/// Construct an `IncompleteToken*` Error with the provided span.
#[inline]
pub fn incomplete_special<T>(span: Span) -> Result<T> {
    Err(Error::IncompleteSpecial { span })
}

// --------------------------------------------------------------
// Directives
// --------------------------------------------------------------

/// Construct an `InvalidDirectiveInput` Error with the provided span.
#[inline]
pub fn invalid_directive_input<T>(span: Span) -> Result<T> {
    Err(Error::InvalidDirectiveInput { span })
}

/// Construct an `UnknownDirectiveName` Error with the provided span.
#[inline]
pub fn unknown_directive_name<T, S>(span: Span, name: S) -> Result<T>
where
    S: Into<String>,
{
    Err(Error::UnknownDirectiveName {
        span,
        name: name.into(),
    })
}

// --------------------------------------------------------------
// Datum Labels
// --------------------------------------------------------------

/// Construct an `InvalidDatumLabel` Error with the provided span.
#[inline]
pub fn invalid_datum_label<T>(span: Span) -> Result<T> {
    Err(Error::InvalidDatumLabel { span })
}

/// Construct an `DuplicateDatumLabel` Error with the provided label and span.
#[inline]
pub fn duplicate_datum_label<T>(span: Span, label: u16) -> Result<T> {
    Err(Error::DuplicateDatumLabel { span, label })
}

/// Construct an `UnknownDatumLabel` Error with the provided label and span.
#[inline]
pub fn unknown_datum_label<T>(span: Span, label: u16) -> Result<T> {
    Err(Error::UnknownDatumLabel { span, label })
}

// --------------------------------------------------------------
// Identifiers
// --------------------------------------------------------------

/// Construct an `IncompleteIdentifier` Error with the provided span.
#[inline]
pub fn incomplete_identifier<T>(span: Span) -> Result<T> {
    Err(Error::IncompleteIdentifier { span })
}

/// Construct an `InvalidIdentifierMnemonicEscape` Error with the provided span.
#[inline]
pub fn invalid_identifier_mnemonic_escape<T>(span: Span) -> Result<T> {
    Err(Error::InvalidIdentifierMnemonicEscape { span })
}

/// Construct an `InvalidIdentifierHexEscape` Error with the provided span.
#[inline]
pub fn invalid_identifier_hex_escape<T>(span: Span) -> Result<T> {
    Err(Error::InvalidIdentifierHexEscape { span })
}

/// Construct an `InvalidIdentifierInput` Error with the provided span.
#[inline]
pub fn invalid_identifier_input_for<T>(span: Span, source: Error) -> Result<T> {
    Err(Error::InvalidIdentifierInput {
        span,
        source: Some(Box::from(source)),
    })
}

/// Construct an `InvalidIdentifierInput` Error with the provided span.
#[inline]
pub fn invalid_identifier_input<T>(span: Span) -> Result<T> {
    Err(Error::InvalidIdentifierInput { span, source: None })
}

// --------------------------------------------------------------
// Booleans
// --------------------------------------------------------------

/// Construct an `InvalidBooleanInput` Error with the provided span.
#[inline]
pub fn invalid_boolean_input<T>(span: Span) -> Result<T> {
    Err(Error::InvalidBooleanInput { span })
}

// --------------------------------------------------------------
// Characters
// --------------------------------------------------------------

/// Construct an `UnknownCharName` Error with the provided span.
#[inline]
pub fn unknown_char_name<S, T>(span: Span, name: S) -> Result<T>
where
    S: Into<String>,
{
    Err(Error::UnknownCharName {
        span,
        name: name.into(),
    })
}

/// Construct an `InvalidUnicodeValue` Error with the provided span.
#[inline]
pub fn invalid_unicode_value<T>(span: Span) -> Result<T> {
    Err(Error::InvalidUnicodeValue { span })
}

/// Construct an `InvalidCharInput` Error with the provided span.
#[inline]
pub fn invalid_char_input<T>(span: Span) -> Result<T> {
    Err(Error::InvalidCharInput { span, source: None })
}

/// Construct an `InvalidCharInput` Error with the provided span.
#[inline]
pub fn invalid_char_input_for<T>(span: Span, source: Error) -> Result<T> {
    Err(Error::InvalidCharInput {
        span,
        source: Some(Box::from(source)),
    })
}

// --------------------------------------------------------------
// Strings
// --------------------------------------------------------------

/// Construct an `IncompleteToken*` Error with the provided span.
#[inline]
pub fn incomplete_string<T>(span: Span) -> Result<T> {
    Err(Error::IncompleteString { span })
}

/// Construct an `InvalidStringMnemonicEscape` Error with the provided span.
#[inline]
pub fn invalid_string_mnemonic_escape<T>(span: Span) -> Result<T> {
    Err(Error::InvalidStringMnemonicEscape { span })
}

/// Construct an `InvalidStringHexEscape` Error with the provided span.
#[inline]
pub fn invalid_string_hex_escape<T>(span: Span) -> Result<T> {
    Err(Error::InvalidStringHexEscape { span })
}

/// Construct an `InvalidStringInput` Error with the provided span.
#[inline]
pub fn invalid_string_input_for<T>(span: Span, source: Error) -> Result<T> {
    Err(Error::InvalidStringInput {
        span,
        source: Some(Box::from(source)),
    })
}

/// Construct an `InvalidStringInput` Error with the provided span.
#[inline]
pub fn invalid_string_input<T>(span: Span) -> Result<T> {
    Err(Error::InvalidStringInput { span, source: None })
}

// --------------------------------------------------------------
// Numbers
// --------------------------------------------------------------

/// Construct an `InvalidNumericInput` Error with the provided span.
#[inline]
pub fn invalid_numeric_input_for<T>(span: Span, source: Error) -> Result<T> {
    Err(Error::InvalidNumericInput {
        span,
        source: Some(Box::from(source)),
    })
}

/// Construct an `InvalidNumericInput` Error with the provided span.
#[inline]
pub fn invalid_numeric_input<T>(span: Span) -> Result<T> {
    Err(Error::InvalidNumericInput { span, source: None })
}

// --------------------------------------------------------------
// Lists
// --------------------------------------------------------------

/// Construct an `IncompleteList` Error with the provided span.
#[inline]
pub fn incomplete_list<T>(span: Span) -> Result<T> {
    Err(Error::IncompleteList { span })
}

// --------------------------------------------------------------
// Vectors
// --------------------------------------------------------------

/// Construct an `IncompleteVector` Error with the provided span.
#[inline]
pub fn incomplete_vector<T>(span: Span) -> Result<T> {
    Err(Error::IncompleteVector { span })
}

// --------------------------------------------------------------
// Byte Vectors
// --------------------------------------------------------------

/// Construct an `InvalidByteVectorPrefix` Error with the provided span.
#[inline]
pub fn invalid_byte_vector_prefix<T>(span: Span) -> Result<T> {
    Err(Error::InvalidByteVectorPrefix { span })
}

/// Construct an `IncompleteByteVector` Error with the provided span.
#[inline]
pub fn incomplete_byte_vector<T>(span: Span) -> Result<T> {
    Err(Error::IncompleteByteVector { span })
}

// --------------------------------------------------------------
// Quotes
// --------------------------------------------------------------

/// Construct an `IncompleteQuote` Error with the provided span.
#[inline]
pub fn incomplete_quote<T>(span: Span) -> Result<T> {
    Err(Error::IncompleteQuote { span })
}

/// Construct an `IncompleteQuasiQuote` Error with the provided span.
#[inline]
pub fn incomplete_quasi_quote<T>(span: Span) -> Result<T> {
    Err(Error::IncompleteQuasiQuote { span })
}

/// Construct an `IncompleteUnquote` Error with the provided span.
#[inline]
pub fn incomplete_unquote<T>(span: Span) -> Result<T> {
    Err(Error::IncompleteUnquote { span })
}

/// Construct an `IncompleteUnquoteSplicing` Error with the provided span.
#[inline]
pub fn incomplete_unquote_splicing<T>(span: Span) -> Result<T> {
    Err(Error::IncompleteUnquoteSplicing { span })
}

// --------------------------------------------------------------
// Comments
// --------------------------------------------------------------

/// Construct an `IncompleteToken*` Error with the provided span.
#[inline]
pub fn incomplete_block_comment<T>(span: Span) -> Result<T> {
    Err(Error::IncompleteBlockComment { span })
}

/// Construct an `IncompleteDatumComment` Error with the provided span.
#[inline]
pub fn incomplete_datum_comment<T>(span: Span) -> Result<T> {
    Err(Error::IncompleteDatumComment { span })
}

// --------------------------------------------------------------
// Unexpected
// --------------------------------------------------------------

/// Construct an `UnexpectedToken` Error with the provided span.
#[inline]
pub fn unexpected_token<T>(span: Span, token: TokenKind) -> Result<T> {
    Err(Error::UnexpectedToken {
        span,
        token,
        within: None,
    })
}

/// Construct an `UnexpectedToken` Error with the provided span.
#[inline]
pub fn unexpected_token_within<T>(span: Span, token: TokenKind, within: Span) -> Result<T> {
    Err(Error::UnexpectedToken {
        span,
        token,
        within: Some(within),
    })
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
                // --------------------------------------------------------------
                Self::IncompleteSpecial { span } =>
                    format!("Incomplete special form, span: {:?}", span.as_range()),

                Self::InvalidDirectiveInput { span } => format!(
                    "Invalid, or badly formed, directive input; span: {:?}",
                    span.as_range()
                ),
                // --------------------------------------------------------------
                Self::UnknownDirectiveName { span, name } => format!(
                    "The directive names {:?} is not known; span: {:?}",
                    span.as_range(),
                    name
                ),
                // --------------------------------------------------------------
                Self::InvalidDatumLabel { span } => format!(
                    "Invalid, or badly formed, datum label assignment or reference; span: {:?}",
                    span.as_range()
                ),
                Self::DuplicateDatumLabel { span, label } => format!(
                    "The label `{}` has already been defined; span: {:?}",
                    label, span
                ),
                Self::UnknownDatumLabel { span, label } => format!(
                    "The label `{}` referenced has not been defined; span: {:?}",
                    label, span
                ),
                // --------------------------------------------------------------
                Self::IncompleteIdentifier { span } => format!(
                    "Incomplete identifier, expecting a terminating `#\\|`; span: {:?}",
                    span.as_range()
                ),
                Self::InvalidIdentifierMnemonicEscape { span } => format!(
                    "Invalid mnemonic escape in identifier; span: {:?}",
                    span.as_range()
                ),
                Self::InvalidIdentifierHexEscape { span } => format!(
                    "Invalid hex escape in identifier span: {:?}",
                    span.as_range()
                ),
                Self::InvalidIdentifierInput { span, source } => format!(
                    "Invalid, or badly formed, identifier input; span: {:?}{}",
                    span.as_range(),
                    if let Some(source) = source {
                        format!(". Based on: {}.", source)
                    } else {
                        String::new()
                    }
                ),
                // --------------------------------------------------------------
                Self::InvalidBooleanInput { span } => format!(
                    "Invalid, or badly formed, boolean input; span: {:?}",
                    span.as_range()
                ),
                // --------------------------------------------------------------
                Self::UnknownCharName { span, name } => format!(
                    "Unknown character name {:?}; span: {:?}",
                    name,
                    span.as_range()
                ),
                Self::InvalidUnicodeValue { span } => format!(
                    "Could not convert to a valid Unicode codepoint; span: {:?}",
                    span.as_range()
                ),
                Self::InvalidCharInput { span, source } => format!(
                    "Invalid, or badly formed, character input; span: {:?}{}",
                    span.as_range(),
                    if let Some(source) = source {
                        format!(". Based on: {}.", source)
                    } else {
                        String::new()
                    }
                ),
                // --------------------------------------------------------------
                Self::IncompleteString { span } => format!(
                    "Incomplete string, expecting a terminating `#\\\"`; span: {:?}",
                    span.as_range()
                ),
                Self::InvalidStringMnemonicEscape { span } => format!(
                    "Invalid mnemonic escape in string; span: {:?}",
                    span.as_range()
                ),
                Self::InvalidStringHexEscape { span } =>
                    format!("Invalid hex escape in string span: {:?}", span.as_range()),
                Self::InvalidStringInput { span, source } => format!(
                    "Invalid, or badly formed, string input; span: {:?}{}",
                    span.as_range(),
                    if let Some(source) = source {
                        format!(". Based on: {}.", source)
                    } else {
                        String::new()
                    }
                ),
                // --------------------------------------------------------------
                Self::InvalidNumericInput { span, source } => format!(
                    "Invalid, or badly formed, numeric input; span: {:?}{}",
                    span.as_range(),
                    if let Some(source) = source {
                        format!(". Based on: {}.", source)
                    } else {
                        String::new()
                    }
                ),
                // --------------------------------------------------------------
                Self::IncompleteList { span } =>
                    format!("Didn't find an end to the open list; span: {:?}", span),
                // --------------------------------------------------------------
                Self::IncompleteVector { span } =>
                    format!("Didn't find an end to the open list; span: {:?}", span),
                // --------------------------------------------------------------
                Self::InvalidByteVectorPrefix { span } => format!(
                    "Invalid or incomplete byte vector prefix; span: {:?}",
                    span.as_range()
                ),
                Self::IncompleteByteVector { span } =>
                    format!("Didn't find an end to the open list; span: {:?}", span),
                // --------------------------------------------------------------
                Self::IncompleteQuote { span } => format!(
                    "The quote symbol \"'\" was not followed by a datum; span: {:?}",
                    span
                ),
                Self::IncompleteQuasiQuote { span } => format!(
                    "The quasi-quote symbol \"`\" was not followed by a datum; span: {:?}",
                    span
                ),
                Self::IncompleteUnquote { span } => format!(
                    "The unquote symbol \",\" was not followed by a datum; span: {:?}",
                    span
                ),
                Self::IncompleteUnquoteSplicing { span } => format!(
                    "The unquote-splicing symbol \",@\" was not followed by a datum; span: {:?}",
                    span
                ),
                // --------------------------------------------------------------
                Self::IncompleteBlockComment { span } => format!(
                    "Incomplete block comment, expecting a terminating `#|`; span: {:?}",
                    span.as_range()
                ),
                Self::IncompleteDatumComment { span } => format!(
                    "Datum comment symbol not followed by an actual datum; span: {:?}",
                    span
                ),
                // --------------------------------------------------------------
                Self::UnexpectedToken {
                    token,
                    span,
                    within,
                } => format!(
                    "The token {} was not expected; span: {:?}{}",
                    token,
                    span,
                    if let Some(within) = within {
                        format!(", within: {:?}", within.as_range())
                    } else {
                        "".into()
                    }
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
            Self::IoError { source: _ } => 1,
            // --------------------------------------------------------------
            Self::IncompleteSpecial { span: _ } => 10,
            Self::InvalidDirectiveInput { span: _ } => 11,
            Self::UnknownDirectiveName { span: _, name: _ } => 12,
            Self::InvalidDatumLabel { span: _ } => 13,
            Self::DuplicateDatumLabel { label: _, span: _ } => 14,
            Self::UnknownDatumLabel { label: _, span: _ } => 15,
            // --------------------------------------------------------------
            Self::IncompleteIdentifier { span: _ } => 20,
            Self::InvalidIdentifierMnemonicEscape { span: _ } => 21,
            Self::InvalidIdentifierHexEscape { span: _ } => 22,
            Self::InvalidIdentifierInput { span: _, source: _ } => 23,
            // --------------------------------------------------------------
            Self::InvalidBooleanInput { span: _ } => 30,
            // --------------------------------------------------------------
            Self::UnknownCharName { span: _, name: _ } => 51,
            Self::InvalidUnicodeValue { span: _ } => 52,
            Self::InvalidCharInput { span: _, source: _ } => 53,
            // --------------------------------------------------------------
            Self::IncompleteString { span: _ } => 60,
            Self::InvalidStringMnemonicEscape { span: _ } => 61,
            Self::InvalidStringHexEscape { span: _ } => 63,
            Self::InvalidStringInput { span: _, source: _ } => 64,
            // --------------------------------------------------------------
            Self::InvalidNumericInput { span: _, source: _ } => 70,
            // --------------------------------------------------------------
            Self::IncompleteList { span: _ } => 80,
            Self::IncompleteVector { span: _ } => 81,
            Self::InvalidByteVectorPrefix { span: _ } => 82,
            Self::IncompleteByteVector { span: _ } => 83,
            // --------------------------------------------------------------
            Self::IncompleteQuote { span: _ } => 91,
            Self::IncompleteQuasiQuote { span: _ } => 92,
            Self::IncompleteUnquote { span: _ } => 93,
            Self::IncompleteUnquoteSplicing { span: _ } => 94,
            // --------------------------------------------------------------
            Self::IncompleteBlockComment { span: _ } => 100,
            Self::IncompleteDatumComment { span: _ } => 101,
            // --------------------------------------------------------------
            Self::UnexpectedToken {
                token: _,
                span: _,
                within: _,
            } => 110,
        }
    }
    pub fn report(&self) -> Option<Report> {
        let syntax = Color::Fixed(81);

        match self {
            // --------------------------------------------------------------
            Self::IncompleteSpecial { span } => Some(
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
            // --------------------------------------------------------------
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
            Self::UnknownDirectiveName { span, name } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Unknown directive name")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message(
                                format!("{} is not a directive name", name.fg(syntax))),
                    )
                    .finish(),
            ),
            // --------------------------------------------------------------
            Self::InvalidDatumLabel { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Invalid, or badly formed, datum label")
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
            Self::DuplicateDatumLabel { span, label } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Duplicate datum label assignment")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message(
                                format!("The label `{}` has already been defined", label)),
                    )
                   .finish(),
            ),
             Self::UnknownDatumLabel { span, label } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Unknown datum label")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message(
                            format!("The label `{}` has not been defined", label)),
                    )
                   .finish(),
            ),
            // --------------------------------------------------------------
            Self::IncompleteIdentifier { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Incomplete identifier")
                    .with_label(
                        Label::new(span.as_start_range())
                            .with_message(format!("Starts with {} here", "#\\|".fg(syntax))),
                    )
                    .finish()
            ),
            Self::InvalidIdentifierMnemonicEscape { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Invalid, or badly formed, mnemonic escape in identifier")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("Not a valid escape sequence"),
                    )
                    .with_note(
                        format!(
                            "Expecting a mnemonic in {}",
                            "#\\a #\\b #\\t #\\n #\\r #\\\" #\\\\ #\\|".fg(syntax),
                        ))
                    .finish()
            ),
            Self::InvalidIdentifierHexEscape { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Invalid, or badly formed, hex escape in identifier")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("Not a valid escape sequence"),
                    )
                    .with_note(
                        format!(
                            "Expecting a hex escape in the form {}",
                            "\\x⟨nn⟩;".fg(syntax)
                        ))
                    .finish()
            ),
            Self::InvalidIdentifierInput { span, source: _ } => Some(if span.is_empty() {
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Invalid identifier")
                    .finish()
                } else {
                    Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Invalid, or badly formed, identifier input")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("Not a valid identifier"),
                    )
                    .finish()
                }),
            // --------------------------------------------------------------
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
            // --------------------------------------------------------------
            Self::UnknownCharName { span, name } => Some(
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
            Self::InvalidCharInput { span, source: _ } => Some(
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
            // --------------------------------------------------------------
            Self::IncompleteString { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Incomplete string")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("String starts here"),
                    ).with_label(
                        Label::new(span.as_end_range())
                            .with_message("Reported here"),
                    )
                    .with_note(format!("Expecting a closing {} character", "#\\\"".fg(syntax)))
                    .finish(),
            ),
            Self::InvalidStringMnemonicEscape { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Invalid, or badly formed, mnemonic escape in string")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("Not a valid escape sequence"),
                    )
                    .with_note(
                        format!(
                            "Expecting a mnemonic in {}",
                            "#\\a #\\b #\\t #\\n #\\r #\\\" #\\\\ #\\|".fg(syntax),
                        ))
                    .finish()
            ),
            Self::InvalidStringHexEscape { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Invalid, or badly formed, hex escape in string")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("Not a valid escape sequence"),
                    )
                    .with_note(
                        format!(
                            "Expecting a hex escape in the form {}",
                            "\\x⟨nn⟩;".fg(syntax)
                        ))
                    .finish()
            ),
            Self::InvalidStringInput { span, source: _ } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Invalid, or badly formed, string input")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("Not a valid string"),
                    )
                    .finish(),
            ),
            // --------------------------------------------------------------
            Self::InvalidNumericInput { span, source: _ } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Invalid, or badly formed, numeric input")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("Not a valid numeric value"),
                    )
                    .finish(),
            ),
            // --------------------------------------------------------------
            Self::IncompleteList { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Incomplete list")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("List starts here"),
                    )
                    .with_note(format!("Expecting a closing {}", "#\\)".fg(syntax)))
                    .finish(),
            ),
            // --------------------------------------------------------------
            Self::IncompleteVector { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Incomplete list")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("Vector starts here"),
                    )
                    .with_note(format!("Expecting a closing {}", "#\\)".fg(syntax)))
                    .finish(),
            ),
            // --------------------------------------------------------------
            Self::InvalidByteVectorPrefix { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Invalid, or incomplete, byte vector prefix")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("This is not a valid byte vector prefix"),
                    )
                    .with_note(format!("Expecting the prefix {}", "#u8(".fg(syntax)))
                    .finish(),
            ),
            Self::IncompleteByteVector { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Incomplete list")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("Byte vector starts here"),
                    )
                    .with_note(format!("Expecting a closing {}", ")".fg(syntax)))
                    .finish(),
            ),
            // --------------------------------------------------------------
             Self::IncompleteQuote { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                     .with_message("Incomplete quote form")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message(
                                format!(
                                    "The quote symbol {} was not followed by a datum",
                                    "⟨'⟩".fg(syntax)
                                )),
                    )
                   .finish(),
            ),
             Self::IncompleteQuasiQuote { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                     .with_message("Incomplete quasiquote form")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message(
                                format!(
                                    "The quasi-quote symbol {} was not followed by a datum",
                                    "⟨`⟩".fg(syntax)
                                )),
                    )
                   .finish(),
            ),
             Self::IncompleteUnquote { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                     .with_message("Incomplete unquote form")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message(
                                format!(
                                    "The unquote symbol {} was not followed by a datum",
                                    "⟨,⟩".fg(syntax)
                                )),
                    )
                   .finish(),
            ),
             Self::IncompleteUnquoteSplicing { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                     .with_message("Incomplete unquote splicing form")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message(
                                format!(
                                    "The unquote-splicing symbol {} was not followed by a datum",
                                    "⟨,@⟩".fg(syntax)
                                )),
                    )
                   .finish(),
            ),
             // --------------------------------------------------------------
            Self::IncompleteBlockComment { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Incomplete block comment")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("Comment starts here"),
                    )
                    .with_note(format!("Expecting a closing {}", "|#".fg(syntax)))
                    .finish(),
            ),
            Self::IncompleteDatumComment { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Incomplete datum comment")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message(
                            format!(
                                    "The datum quote form {} was not followed by a datum",
                                    "#;".fg(syntax)
                                )),
                    )
                    .finish(),
            ),
           // --------------------------------------------------------------
             Self::UnexpectedToken { token, span, within, } => {
                let mut report = Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                     .with_message(format!("A {} token was not expected here", token))
                    .with_label(
                        Label::new(span.as_range())
                            .with_message(format!("This {}", token)),
                    );
                if let Some(within) = within {
                    report = report.with_label(
                        Label::new(within.as_range())
                            .with_message("Within this context"),
                    );
                }
                 Some(report.finish())
             }
           _ => None,
        }
    }

    pub fn print<S>(&self, source: S)
    where
        S: AsRef<str>,
    {
        let source = source.as_ref();

        if let Some(err_source) = self.self_source() {
            err_source.print(source);
        }

        if let Some(report) = self.report() {
            report
                .print(Source::from(source))
                .expect("Could not write error as report");
        } else {
            println!("Error::print {self:?}");
            error!("{}", self);
        }
    }

    fn self_source(&self) -> &Option<Box<Self>> {
        match self {
            Self::InvalidIdentifierInput { span: _, source } => source,
            Self::InvalidCharInput { span: _, source } => source,
            Self::InvalidStringInput { span: _, source } => source,
            Self::InvalidNumericInput { span: _, source } => source,
            _ => &None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------
