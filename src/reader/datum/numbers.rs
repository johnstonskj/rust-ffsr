/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::error::{invalid_numeric_input, Error};
use crate::lexer::token::Span;
use crate::reader::datum::{Datum, DatumValue, SimpleDatumValue};
use std::fmt::{Binary, Debug, LowerHex, Octal, UpperHex};
use std::{fmt::Display, str::FromStr};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum SNumber {
    Fixnum(Fixnum),
    Flonum(Flonum),
    Ratnum(Ratnum),
    ExactComplexnum(ExactComplexnum),
    Complexnum(Complexnum),
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fixnum(i64);

#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Flonum(f64);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ratnum(Fixnum, Fixnum);

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExactComplexnum(Ratnum, Ratnum);

#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Complexnum(Flonum, Flonum);

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
// Public Functions
// ------------------------------------------------------------------------------------------------

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

#[derive(Clone, Copy, Debug, PartialEq)]
enum ParseResult {
    Fixnum,
    Flonum,
    Ratnum(Fixnum),
    ExComplexnum(Ratnum),
    Complexnum(Flonum),
}

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
    };
}

macro_rules! tuple_number_impl {
    ($number:ident, $inner_type:ty, $left:ident, $right:ident) => {
        impl From<($inner_type, $inner_type)> for $number {
            fn from(tuple: ($inner_type, $inner_type)) -> Self {
                Self(tuple.0, tuple.1)
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

        impl $number {
            pub fn $left(&self) -> $inner_type {
                self.0
            }

            pub fn $right(&self) -> $inner_type {
                self.1
            }
        }
    };
}

macro_rules! change_state {
    ($current:expr => $state:ident) => {
        println!(
            "datum [{}] number state {:?} => {:?}",
            line!(),
            $current,
            ParseState::$state,
        );
        $current = ParseState::$state;
    };
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

number_impl!(Fixnum, i64);

pub const FIXNUM_ZERO: Fixnum = Fixnum(0_i64);
pub const FIXNUM_ONE: Fixnum = Fixnum(1_i64);

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

// ------------------------------------------------------------------------------------------------

number_impl!(Flonum, f64);

pub const FLONUM_ZERO: Flonum = Flonum(0_f64);
pub const FLONUM_ONE: Flonum = Flonum(1_f64);

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
        write!(f, "#i{self}")
    }
}

// ------------------------------------------------------------------------------------------------

tuple_number_impl!(Ratnum, Fixnum, numerator, denominator);

pub const RATNUM_ZERO: Ratnum = Ratnum(FIXNUM_ZERO, FIXNUM_ONE);
pub const RATNUM_ONE: Ratnum = Ratnum(FIXNUM_ONE, FIXNUM_ONE);

impl From<Fixnum> for Ratnum {
    fn from(numerator: Fixnum) -> Self {
        assert!(i64::from(numerator).is_positive());
        Self(numerator, 1.into())
    }
}

impl Default for Ratnum {
    fn default() -> Self {
        Self::from(Fixnum::default())
    }
}

impl Display for Ratnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.0, self.1)
    }
}

impl Debug for Ratnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#e{}/{}", self.0, self.1)
    }
}

impl Ratnum {
    pub fn new(numerator: Fixnum, denominator: Fixnum) -> Self {
        assert!(i64::from(numerator).is_positive());
        let d = i64::from(denominator);
        assert!(d.is_positive() && d != 0);
        Self(numerator, denominator)
    }
}

// ------------------------------------------------------------------------------------------------

tuple_number_impl!(ExactComplexnum, Ratnum, real, imaginary);

pub const EXACT_COMPLEXNUM_ZERO: ExactComplexnum = ExactComplexnum(RATNUM_ZERO, RATNUM_ZERO);

impl Display for ExactComplexnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{:+}i", self.0, self.1)
    }
}

impl Debug for ExactComplexnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#e{self}")
    }
}

impl ExactComplexnum {
    pub fn new(real: Ratnum, imaginary: Ratnum) -> Self {
        Self(real, imaginary)
    }
}

// ------------------------------------------------------------------------------------------------

tuple_number_impl!(Complexnum, Flonum, real, imaginary);

pub const COMPLEXNUM_ZERO: Complexnum = Complexnum(FLONUM_ZERO, FLONUM_ZERO);

impl Display for Complexnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{:+}i", self.0, self.1)
    }
}

impl Debug for Complexnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#i{self}")
    }
}

impl Complexnum {
    pub fn new(real: Flonum, imaginary: Flonum) -> Self {
        Self(real, imaginary)
    }
}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

impl Display for SNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Fixnum(v) => format!("{v}"),
                Self::Flonum(v) => format!("{v}"),
                Self::Ratnum(v) => format!("{v}"),
                Self::ExactComplexnum(v) => format!("{v}"),
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
                Self::ExactComplexnum(v) => format!("{v:?}"),
                Self::Complexnum(v) => format!("{v:?}"),
            }
        )
    }
}

impl From<SNumber> for Datum {
    fn from(v: SNumber) -> Self {
        Self::Number(v)
    }
}

impl FromStr for SNumber {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_in_span(s, Span::new_char_span_from(s))
    }
}

impl DatumValue for SNumber {}

impl SimpleDatumValue for SNumber {
    fn from_str_in_span(s: &str, span: Span) -> Result<Self, Error> {
        if s.is_empty() {
            eprintln!("Number is an empty string");
            return Err(invalid_numeric_input(span));
        }

        let mut current_state = ParseState::Start;
        let mut exactness: Option<Exactness> = None;
        let mut radix: Option<Radix> = None;
        let mut post_prefix_mark: usize = 0;
        let mut result = ParseResult::Fixnum;

        for (i, c) in s.char_indices() {
            println!("datum number ({i}, {c:?})");

            match (current_state, c) {
                (ParseState::Start, '#') => {
                    change_state!(current_state => InPrefix);
                }
                (ParseState::InPrefix, c) if Exactness::is_valid(c) => {
                    if exactness.is_none() {
                        exactness = Some(Exactness::try_from(c)?);
                        change_state!(current_state => Start);
                    } else {
                        eprintln!("Number cannot have more than one exactness prefix");
                        return Err(invalid_numeric_input(span));
                    }
                }
                (ParseState::InPrefix, c) if Radix::is_valid(c) => {
                    if radix.is_none() {
                        radix = Some(Radix::try_from(c)?);
                        change_state!(current_state => Start);
                    } else {
                        eprintln!("Number cannot have more than one radix prefix");
                        return Err(invalid_numeric_input(span));
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
                    result = ParseResult::Ratnum(lhs.as_fixnum().unwrap());
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
                    eprintln!("Not expecting {c:?} in state {s:?}");
                    return Err(invalid_numeric_input(span));
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
        match (expecting, exactness) {
            (ParseResult::Fixnum, Some(Exactness::Exact) | None) => Ok(SNumber::Fixnum(
                Fixnum::from(i64::from_str_radix(s, radix.into()).unwrap()),
            )),
            (ParseResult::Flonum, Some(Exactness::Inexact) | None) => {
                // TODO: radix MUST be 10
                Ok(SNumber::Flonum(Flonum::from(f64::from_str(s).unwrap())))
            }
            _ => {
                eprintln!(
                    "Couldn't make a {expecting:?}; exactness: {exactness:?}, radix: {radix}"
                );
                Err(invalid_numeric_input(span))
            }
        }
    }

    pub fn is_fixnum(&self) -> bool {
        matches!(self, Self::Fixnum(_))
    }

    pub fn as_fixnum(&self) -> Option<Fixnum> {
        match self {
            Self::Fixnum(v) => Some(*v),
            _ => None,
        }
    }

    pub fn is_flonum(&self) -> bool {
        matches!(self, Self::Flonum(_))
    }

    pub fn as_flonum(&self) -> Option<Flonum> {
        match self {
            Self::Flonum(v) => Some(*v),
            _ => None,
        }
    }

    pub fn is_ratnum(&self) -> bool {
        matches!(self, Self::Ratnum(_))
    }

    pub fn as_ratnum(&self) -> Option<Ratnum> {
        match self {
            Self::Ratnum(v) => Some(*v),
            _ => None,
        }
    }

    pub fn is_exact_complexnum(&self) -> bool {
        matches!(self, Self::ExactComplexnum(_))
    }

    pub fn as_exact_complexnum(&self) -> Option<ExactComplexnum> {
        match self {
            Self::ExactComplexnum(v) => Some(*v),
            _ => None,
        }
    }

    pub fn is_complexnum(&self) -> bool {
        matches!(self, Self::Complexnum(_))
    }

    pub fn as_complexnum(&self) -> Option<Complexnum> {
        match self {
            Self::Complexnum(v) => Some(*v),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::reader::datum::{Fixnum, Flonum, SNumber};
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    #[test]
    fn fixnum_display() {
        assert_eq!(format!("{}", Fixnum::from(109)).as_str(), "109")
    }

    #[test]
    fn fixnum_display_negative() {
        assert_eq!(format!("{}", Fixnum::from(-109)).as_str(), "-109")
    }

    #[test]
    fn fixnum_display_with_sign() {
        assert_eq!(format!("{:+}", Fixnum::from(109)).as_str(), "+109")
    }

    #[test]
    fn fixnum_debug() {
        assert_eq!(format!("{:?}", Fixnum::from(109)).as_str(), "#e109")
    }

    #[test]
    fn fixnum_display_binary() {
        assert_eq!(format!("{:b}", Fixnum::from(109)).as_str(), "#b1101101")
    }

    #[test]
    fn fixnum_display_octal() {
        assert_eq!(format!("{:o}", Fixnum::from(109)).as_str(), "#o155")
    }

    #[test]
    fn fixnum_display_hex_lower() {
        assert_eq!(format!("{:x}", Fixnum::from(109)).as_str(), "#x6d")
    }

    #[test]
    fn fixnum_display_hex_upper() {
        assert_eq!(format!("{:X}", Fixnum::from(109)).as_str(), "#x6D")
    }

    #[test]
    fn from_str_fixnum_zero() {
        assert_eq!(
            SNumber::from_str("0").unwrap(),
            SNumber::Fixnum(Fixnum::from(0))
        );
    }

    #[test]
    fn from_str_fixnum() {
        assert_eq!(
            SNumber::from_str("9762457").unwrap(),
            SNumber::Fixnum(Fixnum::from(9762457))
        );
    }

    #[test]
    fn from_str_fixnum_pos() {
        assert_eq!(
            SNumber::from_str("+9762457").unwrap(),
            SNumber::Fixnum(Fixnum::from(9762457))
        );
    }

    #[test]
    fn from_str_fixnum_neg() {
        assert_eq!(
            SNumber::from_str("-9762457").unwrap(),
            SNumber::Fixnum(Fixnum::from(-9762457))
        );
    }

    #[test]
    fn from_str_flonum_zero() {
        assert_eq!(
            SNumber::from_str("0.0").unwrap(),
            SNumber::Flonum(Flonum::from(0.0))
        );
    }

    #[test]
    fn from_str_flonum() {
        assert_eq!(
            SNumber::from_str("123.45").unwrap(),
            SNumber::Flonum(Flonum::from(123.45))
        );
    }
}
