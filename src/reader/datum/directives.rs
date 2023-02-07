/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::{
    error::{invalid_directive_input, unknown_directive_name, Error},
    lexer::token::Span,
    reader::datum::SimpleDatumValue,
    syntax::{DIRECTIVE_FOLD_CASE, DIRECTIVE_NO_FOLD_CASE, DIRECTIVE_PREFIX_STR},
};
use std::fmt::{Debug, Display};
use tracing::error;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SDirective {
    FoldCase(bool),
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

impl Display for SDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Debug for SDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", DIRECTIVE_PREFIX_STR, self.as_str())
    }
}

impl_datum_value!(Directive, SDirective);

impl_simple_datum_from_str!(Directive, SDirective);

impl SimpleDatumValue for SDirective {
    fn from_str_in_span(s: &str, span: Span) -> Result<Self, Error> {
        let _span = ::tracing::trace_span!("from_str_in_span", s, ?span);
        let _scope = _span.enter();

        if s.len() > 2 && s.starts_with(DIRECTIVE_PREFIX_STR) {
            let s = &s[2..];
            if s == DIRECTIVE_FOLD_CASE {
                Ok(SDirective::FoldCase(true))
            } else if s == DIRECTIVE_NO_FOLD_CASE {
                Ok(SDirective::FoldCase(false))
            } else {
                error!("Directive name {s:?} not known");
                unknown_directive_name(span, s)
            }
        } else {
            error!("Directive does not start with prefix {DIRECTIVE_PREFIX_STR:?}");
            invalid_directive_input(span)
        }
    }
}

impl SDirective {
    pub fn as_str(&self) -> &str {
        match self {
            SDirective::FoldCase(true) => DIRECTIVE_FOLD_CASE,
            SDirective::FoldCase(false) => DIRECTIVE_NO_FOLD_CASE,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
