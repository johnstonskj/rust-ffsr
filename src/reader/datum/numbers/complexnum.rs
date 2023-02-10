use crate::reader::datum::numbers::{Float, Flonum};
use crate::syntax::{NUMERIC_COMPLEX_MARK, NUMERIC_PREFIX_INEXACT};
use num_complex::Complex as NumComplex;
use num_traits::identities::Zero;
use std::fmt::{Debug, Display};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub(crate) type Complex = NumComplex<Float>;

///
/// Cartesian Complex
///
/// Inexact floating-point real numbers. Using double-type of underlying C
/// compiler, usually IEEE 64-bit floating point number. The set of numbers
/// that describes all possible positions in a two dimensional space. This
/// includes real as well as imaginary numbers (a+bi, where a is the real
/// part, b is the imaginary part, and i is the square root of −1.)
///
#[derive(Clone, Copy, Default, PartialEq)]
pub struct Complexnum(Complex);

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

number_impl!(Complexnum, Complex);

impl Display for Complexnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{:+}{NUMERIC_COMPLEX_MARK}", self.0.re, self.0.im)
    }
}

impl Debug for Complexnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{NUMERIC_PREFIX_INEXACT}{self}")
    }
}

impl From<Flonum> for Complexnum {
    fn from(v: Flonum) -> Self {
        Self(Complex::from(Float::from(v)))
    }
}

impl Complexnum {
    pub fn new(real: Flonum, imaginary: Flonum) -> Self {
        Self(num_complex::Complex::new(real.into(), imaginary.into()))
    }

    pub fn is_real(&self) -> bool {
        self.0.im.is_zero()
    }

    pub fn as_real(&self) -> Option<Flonum> {
        if self.is_real() {
            Some(self.0.im.into())
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
