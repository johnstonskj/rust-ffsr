use crate::reader::datum::numbers::{Fixnum, Integer};
use crate::syntax::{
    NUMERIC_PREFIX_BINARY, NUMERIC_PREFIX_EXACT, NUMERIC_PREFIX_HEXADECIMAL, NUMERIC_PREFIX_OCTAL,
    NUMERIC_RATIONAL_SEPARATOR,
};
use num_rational::Ratio as NumRational;
use std::fmt::Display;
use std::fmt::{Binary, Debug, LowerHex, Octal, UpperHex};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub(crate) type Rational = NumRational<Integer>;

///
/// Rationals
///
/// Multi-precision exact non-integral rational numbers. Both denominator and
/// numerator are represented by exact integers. There’s no limit of the size
/// of number except the memory of the machine. The set of numbers that can be
/// expressed as p/q where p and q are integers; e.g. 9/16 works, but pi (an
/// irrational number) doesn’t. These include integers (n/1).
///
#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ratnum(Rational);

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

number_impl!(Ratnum, Rational);

impl From<Fixnum> for Ratnum {
    fn from(numerator: Fixnum) -> Self {
        Self(Rational::from_integer(numerator.into()))
    }
}

impl Display for Ratnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{NUMERIC_RATIONAL_SEPARATOR}{}",
            self.0.numer(),
            self.0.denom()
        )
    }
}

impl Debug for Ratnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{NUMERIC_PREFIX_EXACT}{}{NUMERIC_RATIONAL_SEPARATOR}{}",
            self.0.numer(),
            self.0.denom()
        )
    }
}

impl Binary for Ratnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{NUMERIC_PREFIX_BINARY}{:b}{NUMERIC_RATIONAL_SEPARATOR}{:b}",
            self.0.numer(),
            self.0.denom()
        )
    }
}

impl Octal for Ratnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{NUMERIC_PREFIX_OCTAL}{:o}{NUMERIC_RATIONAL_SEPARATOR}{:o}",
            self.0.numer(),
            self.0.denom()
        )
    }
}

impl LowerHex for Ratnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{NUMERIC_PREFIX_HEXADECIMAL}{:x}{NUMERIC_RATIONAL_SEPARATOR}{:x}",
            self.0.numer(),
            self.0.denom()
        )
    }
}

impl UpperHex for Ratnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{NUMERIC_PREFIX_HEXADECIMAL}{:X}{NUMERIC_RATIONAL_SEPARATOR}{:X}",
            self.0.numer(),
            self.0.denom()
        )
    }
}

impl Ratnum {
    pub fn new(numerator: Fixnum, denominator: Fixnum) -> Self {
        Self(Rational::new(numerator.into(), denominator.into()))
    }

    pub fn is_integer(&self) -> bool {
        self.0.is_integer()
    }

    pub fn as_integer(&self) -> Option<Fixnum> {
        if self.0.is_integer() {
            Some(self.0.numer().clone().into())
        } else {
            None
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
