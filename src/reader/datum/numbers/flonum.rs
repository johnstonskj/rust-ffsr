use crate::reader::datum::numbers::{Fixnum, Float, Number, Ratnum};
use crate::syntax::{
    NEGATIVE_INFINITY, NEGATIVE_NAN, NUMERIC_PREFIX_INEXACT, POSITIVE_INFINITY, POSITIVE_NAN,
};
use std::fmt::{Debug, Display};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Reals
///
/// Inexact floating-point real numbers. Using double-type of underlying C
/// compiler, usually IEEE 64-bit floating point number. The set of numbers
/// that describes all possible positions along a one-dimensional line. This
/// includes rationals as well as irrational numbers.
///
#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Flonum(Float);

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

macro_rules! flonum_only_from {
    ($unsigned:ty, $signed:ty) => {
        flonum_only_from!($unsigned);
        flonum_only_from!($signed);
    };
    ($from_type:ty) => {
        impl From<$from_type> for Flonum {
            fn from(v: $from_type) -> Self {
                Self(Float::from(v))
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

number_impl!(Flonum, Float);

impl Display for Flonum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_infinite() && self.0.is_sign_positive() {
            write!(f, "{POSITIVE_INFINITY}")
        } else if self.0.is_infinite() && self.0.is_sign_negative() {
            write!(f, "{NEGATIVE_INFINITY}")
        } else if self.0.is_nan() && self.0.is_sign_positive() {
            write!(f, "{POSITIVE_NAN}")
        } else if self.0.is_nan() && self.0.is_sign_negative() {
            write!(f, "{NEGATIVE_NAN}")
        } else {
            write!(f, "{:?}", self.0)
        }
    }
}

impl Debug for Flonum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_infinite() || self.0.is_nan() {
            write!(f, "{self}")
        } else {
            write!(f, "{NUMERIC_PREFIX_INEXACT}{self}")
        }
    }
}

flonum_only_from!(f32);
flonum_only_from!(u8, i8);
flonum_only_from!(u16, i16);
flonum_only_from!(u32, i32);

impl From<Fixnum> for Flonum {
    fn from(v: Fixnum) -> Self {
        Flonum::from(v.into_value() as Float)
    }
}

impl From<Ratnum> for Flonum {
    fn from(v: Ratnum) -> Self {
        Flonum::from((*v.value().numer() as f64) / (*v.value().denom() as f64))
    }
}

impl Number<Float> for Flonum {
    fn value(&self) -> &Float {
        &self.0
    }

    fn into_value(self) -> Float {
        self.0
    }
}
