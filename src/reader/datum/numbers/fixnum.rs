use crate::reader::datum::numbers::{Integer, Number};
use crate::reader::datum::Datum;
use crate::syntax::{
    NUMERIC_PREFIX_BINARY, NUMERIC_PREFIX_EXACT, NUMERIC_PREFIX_HEXADECIMAL, NUMERIC_PREFIX_OCTAL,
};
use std::fmt::Display;
use std::fmt::{Binary, Debug, LowerHex, Octal, UpperHex};
use std::ops::Deref;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Integers
///
/// Multi-precision exact integer. There’s no limit of the size of number
/// except the memory of the machine. Whole numbers, positive or negative;
/// e.g. –5, 0, 18.
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fixnum(Integer);

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

macro_rules! fixnum_from {
    ($unsigned:ty, $signed:ty) => {
        fixnum_from!($unsigned);
        fixnum_from!($signed);
    };
    ($from_type:ty) => {
        impl From<$from_type> for Fixnum {
            fn from(v: $from_type) -> Self {
                Self(Integer::from(v))
            }
        }

        impl From<$from_type> for Datum {
            fn from(v: $from_type) -> Self {
                Self::Number(Fixnum(Integer::from(v)).into())
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

number_impl!(Fixnum, Integer);

impl Display for Fixnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.sign_plus() {
            write!(f, "{:+}", self.0)
        } else {
            write!(f, "{}", self.0)
        }
    }
}

impl Debug for Fixnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{NUMERIC_PREFIX_EXACT}{self}")
    }
}

impl Binary for Fixnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{NUMERIC_PREFIX_BINARY}{:b}", self.0)
    }
}

impl Octal for Fixnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{NUMERIC_PREFIX_OCTAL}{:o}", self.0)
    }
}

impl LowerHex for Fixnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{NUMERIC_PREFIX_HEXADECIMAL}{:x}", self.0)
    }
}

impl UpperHex for Fixnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{NUMERIC_PREFIX_HEXADECIMAL}{:X}", self.0)
    }
}

impl Deref for Fixnum {
    type Target = Integer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fixnum_from!(u8, i8);
fixnum_from!(u16, i16);
fixnum_from!(u32, i32);

impl Number<Integer> for Fixnum {
    fn value(&self) -> &Integer {
        &self.0
    }

    fn into_value(self) -> Integer {
        self.0
    }
}
