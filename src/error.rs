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
    IncompleteDatumAssignment {
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
    // Pairs & Lists
    // --------------------------------------------------------------
    IncompleteList {
        span: Span,
    },
    IncompletePair {
        span: Span,
    },
    PairMissingCar {
        span: Span,
    },
    PairMissingCdr {
        span: Span,
    },
    PairAdditionalCdr {
        span: Span,
    },
    InvalidPairInput {
        span: Span,
        source: Option<Box<Error>>,
    },
    CannotAppendToImproperPair {
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
    InvalidByteInput {
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

/// Construct an `IncompleteDatumAssignment` Error with the provided span.
#[inline]
pub fn incomplete_datum_assignment<T>(span: Span, label: u16) -> Result<T> {
    Err(Error::IncompleteDatumAssignment { span, label })
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
// Pairs & Lists
// --------------------------------------------------------------

/// Construct an `IncompleteList` Error with the provided span.
#[inline]
pub fn incomplete_list<T>(span: Span) -> Result<T> {
    Err(Error::IncompleteList { span })
}

/// Construct an `IncompletePair` Error with the provided span.
#[inline]
pub fn incomplete_pair<T>(span: Span) -> Result<T> {
    Err(Error::IncompletePair { span })
}

/// Construct an `PairMissingLeft` Error with the provided span.
#[inline]
pub fn pair_missing_car<T>(span: Span) -> Result<T> {
    Err(Error::PairMissingCar { span })
}

/// Construct an `PairMissingRhs` Error with the provided span.
#[inline]
pub fn pair_missing_cdr<T>(span: Span) -> Result<T> {
    Err(Error::PairMissingCdr { span })
}

/// Construct an `PairAdditionalRhs` Error with the provided span.
#[inline]
pub fn pair_too_many_cdr<T>(span: Span) -> Result<T> {
    Err(Error::PairAdditionalCdr { span })
}

/// Construct an `InvalidPairInput` Error with the provided span.
#[inline]
pub fn invalid_pair_for<T>(span: Span, source: Error) -> Result<T> {
    Err(Error::InvalidPairInput {
        span,
        source: Some(Box::from(source)),
    })
}

/// Construct an `InvalidPairInput` Error with the provided span.
#[inline]
pub fn invalid_pair<T>(span: Span) -> Result<T> {
    Err(Error::InvalidPairInput { span, source: None })
}

/// Construct an `CannotAppendToImproperPair` Error with the provided span.
#[inline]
pub fn cannot_append_to_improper_pair<T>(span: Span) -> Result<T> {
    Err(Error::CannotAppendToImproperPair { span })
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

/// Construct an `InvalidByteInput` Error with the provided span.
#[inline]
pub fn invalid_byte_input<T>(span: Span) -> Result<T> {
    Err(Error::InvalidByteInput { span })
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
                        format!("Incomplete special form, span: {span}"),
                    Self::InvalidDirectiveInput { span } => format!(
                        "Invalid, or badly formed, directive input; span: {span}"
                    ),
                    // --------------------------------------------------------------
                    Self::UnknownDirectiveName { span, name } => format!(
                        "The directive name {name:?} is not known; span: {span}"
                    ),
                    // --------------------------------------------------------------
                    Self::InvalidDatumLabel { span } => format!(
                        "Invalid, or badly formed, datum label assignment or reference; span: {span}"
                    ),
                    Self::DuplicateDatumLabel { span, label } => format!(
                        "The label `{label}` has already been defined; span: {span}"
                    ),
                    Self::UnknownDatumLabel { span, label } => format!(
                        "The label `{label}` referenced has not been defined; span: {span}"
                    ),
                    Self::IncompleteDatumAssignment { span, label } => format!(
                        "Datum assignment to label `{label}` not followed by an actual datum; span: {span}"
                    ),
                    // --------------------------------------------------------------
                    Self::IncompleteIdentifier { span } => format!(
                        "Incomplete identifier, expecting a terminating `#\\|`; span: {span}"
                    ),
                    Self::InvalidIdentifierMnemonicEscape { span } => format!(
                        "Invalid mnemonic escape in identifier; span: {span}"
                    ),
                    Self::InvalidIdentifierHexEscape { span } => format!(
                        "Invalid hex escape in identifier span: {span}"
                    ),
                    Self::InvalidIdentifierInput { span, source } => format!(
                        "Invalid, or badly formed, identifier input; span: {span}{}",
                        if let Some(source) = source {
                            format!(". Based on: {}.", source)
                        } else {
                            String::new()
                        }
                    ),
                    // --------------------------------------------------------------
                    Self::InvalidBooleanInput { span } => format!(
                        "Invalid, or badly formed, boolean input; span: {span}"
                    ),
                    // --------------------------------------------------------------
                    Self::UnknownCharName { span, name } => format!(
                        "Unknown character name {name:?}; span: {span}"
                    ),
                    Self::InvalidUnicodeValue { span } => format!(
                        "Could not convert to a valid Unicode codepoint; span: {span}"
                    ),
                    Self::InvalidCharInput { span, source } => format!(
                        "Invalid, or badly formed, character input; span: {span}{}",
                       if let Some(source) = source {
                            format!(". Based on: {}.", source)
                        } else {
                            String::new()
                        }
                    ),
                    // --------------------------------------------------------------
                    Self::IncompleteString { span } => format!(
                        "Incomplete string, expecting a terminating `#\\\"`; span: {span}"
                    ),
                    Self::InvalidStringMnemonicEscape { span } => format!(
                        "Invalid mnemonic escape in string; span: {span}"
                    ),
                    Self::InvalidStringHexEscape { span } =>
                        format!("Invalid hex escape in string span: {span}"),
                    Self::InvalidStringInput { span, source } => format!(
                        "Invalid, or badly formed, string input; span: {span}{}",
                        if let Some(source) = source {
                            format!(". Based on: {}.", source)
                        } else {
                            String::new()
                        }
                    ),
                    // --------------------------------------------------------------
                    Self::InvalidNumericInput { span, source } => format!(
                        "Invalid, or badly formed, numeric input; span: {span}{}",
                        if let Some(source) = source {
                            format!(". Based on: {}.", source)
                        } else {
                            String::new()
                        }
                    ),
                    // --------------------------------------------------------------
                    Self::IncompleteList { span } =>
                        format!("Didn't find an end to the open list; span: {span}"),
                    Self::IncompletePair { span } => format!("Didn't find an end to the open pair; span: {span}"),
                    Self::PairMissingCar {span } => format!("Dotted pair missing a car value; span: {span}"),
                    Self::PairMissingCdr { span } => format!("Dotted pair missing a cdr value; span: {span}"),
                    Self::PairAdditionalCdr { span } => format!("Dotted pair already has a cdr value; span: {span}"),
                    Self::InvalidPairInput { span, source } => format!("Invalid, or badly formed, dotted pair; span: {span}{}",
                        if let Some(source) = source {
                            format!(". Based on: {}.", source)
                        } else {
                            String::new()
                        }
                    ),
                    Self::CannotAppendToImproperPair { span } => format!("Cannot use cons or append with improper pair; span: {span}"),
                    // --------------------------------------------------------------
                    Self::IncompleteVector { span } =>
                        format!("Didn't find an end to the open list; span: {span}"),
                    // --------------------------------------------------------------
                    Self::InvalidByteVectorPrefix { span } => format!(
                        "Invalid or incomplete byte vector prefix; span: {span}"
                    ),
                    Self::IncompleteByteVector { span } =>
                        format!("Didn't find an end to the open list; span: {span}"),
                    Self::InvalidByteInput { span } => format!("Not an exact integer in the range 0..=255; span: {span}"),
                    // --------------------------------------------------------------
                    Self::IncompleteQuote { span } => format!(
                        "The quote symbol \"'\" was not followed by a datum; span: {span}"
                    ),
                    Self::IncompleteQuasiQuote { span } => format!(
                        "The quasi-quote symbol \"`\" was not followed by a datum; span: {span}"
                    ),
                    Self::IncompleteUnquote { span } => format!(
                        "The unquote symbol \",\" was not followed by a datum; span: {span}"
                    ),
                    Self::IncompleteUnquoteSplicing { span } => format!(
                        "The unquote-splicing symbol \",@\" was not followed by a datum; span: {span}"
                    ),
                    // --------------------------------------------------------------
                    Self::IncompleteBlockComment { span } => format!(
                        "Incomplete block comment, expecting a terminating `#|`; span: {span}"
                    ),
                    Self::IncompleteDatumComment { span } => format!(
                        "Datum comment symbol not followed by an actual datum; span: {span}"
                    ),
                    // --------------------------------------------------------------
                    Self::UnexpectedToken {
                        token,
                        span,
                        within,
                    } => format!(
                        "The token {token} was not expected; span: {span}{}",
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
            Self::IoError { source } => Some(source),
            // TODO: Self::InvalidIdentifierInput { span: _, source } => source.map(|s| s.as_ref()),
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
            Self::IncompleteDatumAssignment { span: _, label: _ } => 16,
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
            Self::IncompletePair { span: _ } => 81,
            Self::PairMissingCar { span: _ } => 82,
            Self::PairMissingCdr { span: _ } => 83,
            Self::PairAdditionalCdr { span: _ } => 84,
            Self::InvalidPairInput { span: _, source: _ } => 85,
            Self::IncompleteVector { span: _ } => 86,
            Self::InvalidByteVectorPrefix { span: _ } => 87,
            Self::IncompleteByteVector { span: _ } => 88,
            Self::InvalidByteInput { span: _ } => 89,
            Self::CannotAppendToImproperPair { span: _ } => 90,
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
        const SYNTAX: Color = Color::Magenta;
        const TYPES: Color = Color::Blue;
        const FUNCTIONS: Color = Color::Cyan;
        const VALUES: Color = Color::Green;

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
                                format!("{} is not a directive name", name.fg(SYNTAX))),
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
                            "#⟨nn⟩=⟨datum⟩".fg(SYNTAX),
                            "#nn#".fg(SYNTAX)
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
                                format!("The label {} has already been defined", label.to_string().fg(VALUES))),
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
                            format!("The label `{}` has not been defined", label.to_string().fg(VALUES))),
                    )
                   .finish(),
            ),
            Self::IncompleteDatumAssignment { span, label } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Incomplete datum assignment")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message(
                            format!(
                                    "The datum assignment {} was not followed by a datum",
                                    "#⟨nn⟩=".fg(SYNTAX)
                                )),
                    )
                    .with_label(
                        Label::new(span.as_range())
                            .with_message(
                            format!("The label `{}` is therefore not defined", label)),
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
                            .with_message(format!("Starts with {} here", "#\\|".fg(SYNTAX))),
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
                            "#\\a #\\b #\\t #\\n #\\r #\\\" #\\\\ #\\|".fg(SYNTAX),
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
                            "\\x⟨nn⟩;".fg(SYNTAX)
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
                            "#t".fg(VALUES),
                            "#f".fg(VALUES)
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
                            "#\\⟨char⟩".fg(SYNTAX),
                             "#\\⟨name⟩".fg(SYNTAX),
                            "#\\x⟨hh⟩;".fg(SYNTAX)
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
                    .with_note(format!("Expecting a closing {} character", "#\\\"".fg(SYNTAX)))
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
                            "#\\a #\\b #\\t #\\n #\\r #\\\" #\\\\ #\\|".fg(SYNTAX),
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
                            "\\x⟨nn⟩;".fg(SYNTAX)
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
                    .with_note(format!("Expecting a closing {}", "#\\)".fg(SYNTAX)))
                    .finish(),
            ),
            Self::IncompletePair { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Incomplete pair")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("List starts here"),
                    )
                    .with_note(format!("Expecting a closing {}", "#\\)".fg(SYNTAX)))
                    .finish(),
            ),
            Self::PairMissingCar { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message(format!("Pair is missing a {} value", "car".fg(FUNCTIONS)))
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("need a datum left of this dot"),
                    )
                    .finish(),
            ),
            Self::PairMissingCdr { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message(format!("Pair is missing a {} value", "cdr".fg(FUNCTIONS)))
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("need a datum right of this dot"),
                    )
                    .finish(),
            ),
            Self::PairAdditionalCdr { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message(format!("Pair already has a {} value", "cdr".fg(FUNCTIONS)))
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("too many values right of this dot"),
                    )
                    .finish(),
            ),
            Self::InvalidPairInput { span, source: _ } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Invalid, or badly formed, pair")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("Could not construct a pair from this"),
                    )
                    .finish(),
            ),
            Self::CannotAppendToImproperPair { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message(
                        format!(
                            "Cannot use {} or {} with an improper pair",
                            "cons".fg(FUNCTIONS),
                            "append".fg(FUNCTIONS)
                        ))
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("This pair"),
                    )
                    .with_note(format!("Expecting a {} value", "list?".fg(TYPES)))
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
                    .with_note(format!("Expecting a closing {}", "#\\)".fg(SYNTAX)))
                    .finish(),
            ),
            // --------------------------------------------------------------
            Self::InvalidByteVectorPrefix { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Too many datum values right of dot")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("This is not a valid byte vector prefix"),
                    )
                    .with_note(format!("Expecting the prefix {}", "#u8(".fg(SYNTAX)))
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
                    .with_note(format!("Expecting a closing {}", ")".fg(SYNTAX)))
                    .finish(),
            ),
            Self::InvalidByteInput { span } => Some(
                Report::build(ReportKind::Error, (), span.start())
                    .with_code(self.code())
                    .with_message("Invalid value in byte-vector")
                    .with_label(
                        Label::new(span.as_range())
                            .with_message("This is not a valid byte"),
                    )
                    .with_note(format!("Expecting an exact integer in the range 0..=255"))
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
                                    "⟨'⟩".fg(SYNTAX)
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
                                    "⟨`⟩".fg(SYNTAX)
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
                                    "⟨,⟩".fg(SYNTAX)
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
                                    "⟨,@⟩".fg(SYNTAX)
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
                    .with_note(format!("Expecting a closing {}", "|#".fg(SYNTAX)))
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
                                    "#;".fg(SYNTAX)
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
