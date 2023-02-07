/*!
One-line description.

More detailed description, with

# Example

YYYYY


*/

use crate::error::{invalid_numeric_input, Error};
use crate::lexer::token::Span;
use crate::reader::datum::{Datum, SimpleDatumValue};
use num_bigint::BigInt;
use num_complex::{Complex64, ComplexFloat};
use num_rational::{BigRational, Ratio};
use num_traits::Zero;
use std::fmt::{Binary, Debug, LowerHex, Octal, UpperHex};
use std::ops::Deref;
use std::{fmt::Display, str::FromStr};
use tracing::{error, trace};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

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
pub struct Fixnum(BigInt);

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
pub struct Ratnum(BigRational);

///
/// Reals
///
/// Inexact floating-point real numbers. Using double-type of underlying C
/// compiler, usually IEEE 64-bit floating point number. The set of numbers
/// that describes all possible positions along a one-dimensional line. This
/// includes rationals as well as irrational numbers.
///
#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Flonum(f64);

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
pub struct Complexnum(Complex64);

#[derive(Clone, PartialEq)]
pub enum SNumber {
    Fixnum(Fixnum),
    Ratnum(Ratnum),
    Flonum(Flonum),
    Complexnum(Complexnum),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exactness {
    Exact,
    Inexact,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Radix {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
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

        impl From<$number> for SNumber {
            fn from(v: $number) -> Self {
                Self::$number(v)
            }
        }

        impl From<$number> for Datum {
            fn from(v: $number) -> Self {
                Self::Number(SNumber::$number(v))
            }
        }

        impl From<$inner_type> for Datum {
            fn from(v: $inner_type) -> Self {
                Self::Number($number::from(v).into())
            }
        }
    };
}

macro_rules! fixnum_from {
    ($unsigned:ty, $signed:ty) => {
        impl From<$unsigned> for Fixnum {
            fn from(v: $unsigned) -> Self {
                Self(BigInt::from(v))
            }
        }

        impl From<$signed> for Fixnum {
            fn from(v: $signed) -> Self {
                Self(BigInt::from(v))
            }
        }

        impl From<$unsigned> for Datum {
            fn from(v: $unsigned) -> Self {
                Self::Number(Fixnum(BigInt::from(v)).into())
            }
        }

        impl From<$signed> for Datum {
            fn from(v: $signed) -> Self {
                Self::Number(Fixnum(BigInt::from(v)).into())
            }
        }
    };
}

macro_rules! change_state {
    ($current:expr => $state:ident) => {
        trace!("change state {:?} => {:?}", $current, ParseState::$state,);
        $current = ParseState::$state;
    };
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ParseState {
    Start,
    InPrefix,
    InSign,
    InInteger,
    InFractional,
    InExponent,
    InRational,
    InComplex,
}

#[allow(unused)]
#[derive(Clone, Debug, PartialEq)]
enum ParseResult {
    Fixnum,
    Flonum,
    Ratnum(Fixnum),
    ExComplexnum(Ratnum),
    Complexnum(Flonum),
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Radix {
    #[inline(always)]
    fn default() -> Self {
        Self::Decimal
    }
}

impl Display for Radix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", char::from(*self))
    }
}

impl TryFrom<char> for Radix {
    type Error = Error;

    #[inline(always)]
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'b' => Ok(Self::Binary),
            'o' => Ok(Self::Octal),
            'd' => Ok(Self::Decimal),
            'x' => Ok(Self::Hexadecimal),
            _ => unreachable!(),
        }
    }
}

impl TryFrom<u32> for Radix {
    type Error = Error;

    #[inline(always)]
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Self::Binary),
            8 => Ok(Self::Octal),
            10 => Ok(Self::Decimal),
            16 => Ok(Self::Hexadecimal),
            _ => unreachable!(),
        }
    }
}

impl From<Radix> for char {
    #[inline(always)]
    fn from(radix: Radix) -> Self {
        match radix {
            Radix::Binary => 'b',
            Radix::Octal => 'o',
            Radix::Decimal => 'd',
            Radix::Hexadecimal => 'x',
        }
    }
}

impl From<Radix> for u32 {
    #[inline(always)]
    fn from(radix: Radix) -> Self {
        match radix {
            Radix::Binary => 2,
            Radix::Octal => 8,
            Radix::Decimal => 10,
            Radix::Hexadecimal => 16,
        }
    }
}

impl Radix {
    #[inline(always)]
    pub fn is_valid(c: char) -> bool {
        matches!(c, 'b' | 'o' | 'd' | 'x')
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Exactness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", char::from(*self))
    }
}

impl TryFrom<char> for Exactness {
    type Error = Error;

    #[inline(always)]
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'e' => Ok(Self::Exact),
            'i' => Ok(Self::Inexact),
            _ => unreachable!(),
        }
    }
}

impl From<Exactness> for char {
    #[inline(always)]
    fn from(exactness: Exactness) -> Self {
        match exactness {
            Exactness::Exact => 'e',
            Exactness::Inexact => 'i',
        }
    }
}

impl Exactness {
    #[inline(always)]
    pub fn is_valid(c: char) -> bool {
        matches!(c, 'e' | 'i')
    }
}

// ------------------------------------------------------------------------------------------------

number_impl!(Fixnum, BigInt);

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
        write!(f, "#e{self}")
    }
}

impl Binary for Fixnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#b{:b}", self.0)
    }
}

impl Octal for Fixnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#o{:o}", self.0)
    }
}

impl LowerHex for Fixnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#x{:x}", self.0)
    }
}

impl UpperHex for Fixnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#x{:X}", self.0)
    }
}

impl Deref for Fixnum {
    type Target = BigInt;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fixnum_from!(u8, i8);
fixnum_from!(u16, i16);
fixnum_from!(u32, i32);
fixnum_from!(u64, i64);
fixnum_from!(u128, i128);
fixnum_from!(usize, isize);

// ------------------------------------------------------------------------------------------------

number_impl!(Ratnum, BigRational);

impl From<Fixnum> for Ratnum {
    fn from(numerator: Fixnum) -> Self {
        Self(Ratio::from_integer(numerator.into()))
    }
}

impl Display for Ratnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.0.numer(), self.0.denom())
    }
}

impl Debug for Ratnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#e{}/{}", self.0.numer(), self.0.denom())
    }
}

impl Binary for Ratnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#b{:b}/{:b}", self.0.numer(), self.0.denom())
    }
}

impl Octal for Ratnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#o{:o}/{:o}", self.0.numer(), self.0.denom())
    }
}

impl LowerHex for Ratnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#x{:x}/{:x}", self.0.numer(), self.0.denom())
    }
}

impl UpperHex for Ratnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#x{:X}/{:X}", self.0.numer(), self.0.denom())
    }
}

impl Ratnum {
    pub fn new(numerator: Fixnum, denominator: Fixnum) -> Self {
        Self(Ratio::new(numerator.into(), denominator.into()))
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

number_impl!(Flonum, f64);

impl Display for Flonum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_infinite() && self.0.is_sign_positive() {
            write!(f, "+inf.0")
        } else if self.0.is_infinite() && self.0.is_sign_negative() {
            write!(f, "-inf.0")
        } else if self.0.is_nan() && self.0.is_sign_positive() {
            write!(f, "+nan.0")
        } else if self.0.is_nan() && self.0.is_sign_negative() {
            write!(f, "-nan.0")
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
            write!(f, "#i{self}")
        }
    }
}

impl From<f32> for Flonum {
    fn from(v: f32) -> Self {
        Self(v as f64)
    }
}

impl Flonum {
    pub fn is_rational(&self) -> bool {
        BigRational::from_float(self.0).is_some()
    }

    pub fn as_rational(&self) -> Option<Ratnum> {
        BigRational::from_float(self.0).map(Ratnum)
    }
}

// ------------------------------------------------------------------------------------------------

number_impl!(Complexnum, Complex64);

impl Display for Complexnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{:+}i", self.0.re(), self.0.im())
    }
}

impl Debug for Complexnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#i{self}")
    }
}

impl From<Flonum> for Complexnum {
    fn from(v: Flonum) -> Self {
        Self(Complex64::from(f64::from(v)))
    }
}

impl Complexnum {
    pub fn new(real: Flonum, imaginary: Flonum) -> Self {
        Self(num_complex::Complex::new(real.into(), imaginary.into()))
    }

    pub fn is_real(&self) -> bool {
        self.0.im().is_zero()
    }

    pub fn as_real(&self) -> Option<Flonum> {
        if self.is_real() {
            Some(self.0.im().into())
        } else {
            None
        }
    }
}

// ------------------------------------------------------------------------------------------------
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
        let _span = ::tracing::trace_span!("from_str_in_span", s, ?span);
        let _scope = _span.enter();

        if s.is_empty() {
            error!("Number may not be an empty string");
            return invalid_numeric_input(span);
        } else if s == "+inf.0" {
            return Ok(Flonum::from(f64::INFINITY).into());
        } else if s == "-inf.0" {
            return Ok(Flonum::from(f64::NEG_INFINITY).into());
        } else if s == "+nan.0" {
            return Ok(Flonum::from(f64::NAN).into());
        } else if s == "-nan.0" {
            return Ok(Flonum::from(-f64::NAN).into());
        }

        let mut current_state = ParseState::Start;
        let mut exactness: Option<Exactness> = None;
        let mut radix: Option<Radix> = None;
        let mut post_prefix_mark: usize = 0;
        let mut result = ParseResult::Fixnum;

        for (i, c) in s.char_indices() {
            trace!("match ({i}, {c:?})");

            match (current_state, c) {
                (ParseState::Start, '#') => {
                    change_state!(current_state => InPrefix);
                }
                (ParseState::InPrefix, c) if Exactness::is_valid(c) => {
                    if exactness.is_none() {
                        exactness = Some(Exactness::try_from(c)?);
                        change_state!(current_state => Start);
                    } else {
                        error!("Number cannot have more than one exactness prefix");
                        return invalid_numeric_input(span);
                    }
                }
                (ParseState::InPrefix, c) if Radix::is_valid(c) => {
                    if radix.is_none() {
                        radix = Some(Radix::try_from(c)?);
                        change_state!(current_state => Start);
                    } else {
                        error!("Number cannot have more than one radix prefix");
                        return invalid_numeric_input(span);
                    }
                }
                (ParseState::Start | ParseState::InExponent, '+' | '-') => {
                    if current_state == ParseState::Start && i != 0 {
                        post_prefix_mark = i;
                    }
                    change_state!(current_state => InSign);
                }
                (ParseState::Start | ParseState::InInteger, '.') => {
                    if current_state == ParseState::Start && i != 0 {
                        post_prefix_mark = i;
                    }
                    result = ParseResult::Flonum;
                    change_state!(current_state => InFractional);
                }
                (ParseState::Start | ParseState::InSign, c) if c.is_ascii_digit() => {
                    if current_state == ParseState::Start && i != 0 {
                        post_prefix_mark = i;
                    }
                    change_state!(current_state => InInteger);
                }
                (ParseState::InInteger | ParseState::InFractional | ParseState::InExponent, c)
                    if c.is_ascii_digit() => {}
                (ParseState::InInteger, 'e') => {
                    result = ParseResult::Fixnum;
                    change_state!(current_state => InExponent);
                }
                (ParseState::InFractional, 'e') => {
                    result = ParseResult::Flonum;
                    change_state!(current_state => InExponent);
                }
                (ParseState::InInteger, '/') => {
                    let lhs = Self::make(
                        &s[post_prefix_mark..i],
                        result,
                        exactness,
                        radix.unwrap_or_default(),
                        span,
                    )?;
                    result = ParseResult::Ratnum(lhs.as_fixnum().cloned().unwrap());
                    change_state!(current_state => InRational);
                }
                (ParseState::InInteger | ParseState::InFractional, '+' | '-') => {
                    // ensure ends with 'i'
                    assert!(s.ends_with('i'));
                    // create lhs
                    // set result
                    change_state!(current_state => InComplex);
                }
                (s, c) => {
                    error!("Not expecting {c:?} in state {s:?}");
                    return invalid_numeric_input(span);
                }
            }
        }

        Self::make(
            &s[post_prefix_mark..],
            result,
            exactness,
            radix.unwrap_or_default(),
            span,
        )
    }
}

impl SNumber {
    fn make(
        s: &str,
        expecting: ParseResult,
        exactness: Option<Exactness>,
        radix: Radix,
        span: Span,
    ) -> Result<Self, Error> {
        match (&expecting, exactness) {
            (ParseResult::Fixnum, Some(Exactness::Exact) | None) => Ok(SNumber::Fixnum(
                Fixnum::from(BigInt::parse_bytes(s.as_bytes(), radix.into()).unwrap()),
            )),
            (ParseResult::Flonum, Some(Exactness::Inexact) | None) => {
                // TODO: radix MUST be 10
                Ok(SNumber::Flonum(Flonum::from(f64::from_str(s).unwrap())))
            }
            _ => {
                error!("Couldn't make a {expecting:?}; exactness: {exactness:?}, radix: {radix}");
                invalid_numeric_input(span)
            }
        }
    }

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
