/*!
One-line description.

More detailed description, with

ℕ ⊂ ℤ ⊂ ℚ ⊂ ℝ ⊂ ℂ

where ℕ ≍ ℤ⁰⁺

also ℤ⁺, ℤ⁰⁻, ℤ⁻,

Reals include Rationals ℝ and irrationals ℝ∖ℚ

# Example

YYYYY

```text
number:
    prefix? sign? inner right_hand_part?

inner:
    ( digits '.' digits? ) | ( '.' digits ) ( exponent )?

prefix:
    ( exactness_prefix radix_prefix? ) | ( radix_prefix exactness_prefix? )

exactness_prefix:
    '#' ( 'e' | 'i' )

radix_prefix:
    '#' ( 'b' | 'o' | 'd' | 'x')

sign:
    '+' | '-'

digits:
    RADIX_DIGIT+

exponent:
    'e' sign? digits

right_hand_part:
    rational_part | polar_complex_part | rectangular_complex_part

rational_part:
    '/' inner

polar_complex_part:
    '@' sign? inner 'i'

rectangular_complex_part:
    sign inner 'i'
```

*/

use crate::error::Error;
use crate::lexer::token::Span;
use crate::reader::datum::SimpleDatumValue;
use std::fmt::{Debug, Display};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The set ℕ of Natural numbers. This includes `0` as per
/// [ISO 80000-2](https://www.iso.org/standard/64973.html).
///
pub type Natural = u64;

///
/// The set ℤ of Integers.
///
pub type Integer = i64;

///
/// The set ℝ of Real Numbers
///
pub type Float = f64;

#[derive(Clone, PartialEq)]
pub enum SNumber {
    /// i64
    Fixnum(Fixnum),
    /// i64/i64
    Ratnum(Ratnum),
    /// f64
    Flonum(Flonum),
    /// (f64, f64)
    Complexnum(Complexnum),
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

macro_rules! number_impl {
    ($number:ident, $inner_type:ty) => {
        impl From<$inner_type> for $number {
            fn from(v: $inner_type) -> Self {
                Self(v)
            }
        }

        impl From<$number> for $inner_type {
            fn from(v: $number) -> Self {
                v.0
            }
        }

        impl From<$number> for $crate::reader::datum::numbers::SNumber {
            fn from(v: $number) -> Self {
                Self::$number(v)
            }
        }

        impl From<$number> for $crate::reader::datum::Datum {
            fn from(v: $number) -> Self {
                Self::Number($crate::reader::datum::numbers::SNumber::$number(v))
            }
        }

        impl From<$inner_type> for $crate::reader::datum::Datum {
            fn from(v: $inner_type) -> Self {
                Self::Number($number::from(v).into())
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for SNumber {
    fn default() -> Self {
        Self::Fixnum(Default::default())
    }
}

impl Display for SNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Fixnum(v) => format!("{v}"),
                Self::Flonum(v) => format!("{v}"),
                Self::Ratnum(v) => format!("{v}"),
                Self::Complexnum(v) => format!("{v}"),
            }
        )
    }
}

impl Debug for SNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Fixnum(v) => format!("{v:?}"),
                Self::Flonum(v) => format!("{v:?}"),
                Self::Ratnum(v) => format!("{v:?}"),
                Self::Complexnum(v) => format!("{v:?}"),
            }
        )
    }
}

impl_datum_value!(Number, SNumber);

impl_simple_datum_from_str!(Number, SNumber);

impl SimpleDatumValue for SNumber {
    fn from_str_in_span(s: &str, span: Span) -> Result<Self, Error> {
        parse::from_str_in_span(s, span)
    }
}

impl SNumber {
    pub fn is_exact(&self) -> bool {
        matches!(self, Self::Fixnum(_) | Self::Ratnum(_))
    }

    pub fn is_inexact(&self) -> bool {
        !self.is_exact()
    }

    is_as_variant!(
        (fixnum, Fixnum, Fixnum),
        (flonum, Flonum, Flonum),
        (ratnum, Ratnum, Ratnum),
        (complexnum, Complexnum, Complexnum)
    );

    into_variant!(
        (fixnum, Fixnum, Fixnum),
        (flonum, Flonum, Flonum),
        (ratnum, Ratnum, Ratnum),
        (complexnum, Complexnum, Complexnum)
    );

    pub fn type_string(&self) -> &'static str {
        match_into_str!(
            self,
            (Self::Fixnum(_) => "fixnum"),
            (Self::Ratnum(_) => "ratnum"),
            (Self::Flonum(_) => "flonum"),
            (Self::Complexnum(_) => "complexnum")
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
mod fixnum;
pub use fixnum::Fixnum;

#[doc(hidden)]
mod ratnum;
pub use ratnum::Ratnum;

#[doc(hidden)]
mod flonum;
pub use flonum::Flonum;

#[doc(hidden)]
mod complexnum;
pub use complexnum::Complexnum;

#[doc(hidden)]
mod parse;
