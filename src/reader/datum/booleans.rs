/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::reader::datum::SimpleDatumValue;
use crate::syntax::{
    BOOLEAN_VALUE_FALSE, BOOLEAN_VALUE_FALSE_UC, BOOLEAN_VALUE_TRUE, BOOLEAN_VALUE_TRUE_UC,
};
use crate::{
    error::{invalid_boolean_input, Error},
    lexer::token::Span,
};
use std::fmt::{Debug, Display};
use tracing::error;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SBoolean(bool);

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for SBoolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if self.0 {
                BOOLEAN_VALUE_TRUE
            } else {
                BOOLEAN_VALUE_FALSE
            }
        )
    }
}

impl Debug for SBoolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl_datum_value!(Boolean, SBoolean, infallible bool);

impl_simple_datum_from_str!(Boolean, SBoolean);

impl SimpleDatumValue for SBoolean {
    fn from_str_in_span(s: &str, span: Span) -> Result<Self, Error> {
        let _span = ::tracing::trace_span!("from_str_in_span", s, ?span);
        let _scope = _span.enter();

        if s == BOOLEAN_VALUE_TRUE || s == BOOLEAN_VALUE_TRUE_UC {
            Ok(SBoolean::from(true))
        } else if s == BOOLEAN_VALUE_FALSE || s == BOOLEAN_VALUE_FALSE_UC {
            Ok(SBoolean::from(false))
        } else {
            error!("Invalid value for boolean {s:?}");
            invalid_boolean_input(span)
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------
