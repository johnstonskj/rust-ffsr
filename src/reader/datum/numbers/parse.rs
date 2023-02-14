use crate::error::{invalid_numeric_input, Error};
use crate::lexer::token::Span;
use crate::reader::datum::numbers::{Complexnum, Fixnum, Float, Flonum, Integer, Ratnum, SNumber};
use crate::syntax::{
    NEGATIVE_INFINITY, NEGATIVE_NAN, NUMERIC_ALT_EXPONENT_MARK, NUMERIC_COMPLEX_MARK,
    NUMERIC_DECIMAL_POINT, NUMERIC_EXPONENT_MARK, NUMERIC_LONG_EXPONENT_MARK, NUMERIC_NEGATIVE,
    NUMERIC_POLAR_SEPARATOR, NUMERIC_POSITIVE, POSITIVE_INFINITY, POSITIVE_NAN,
    SPECIAL_PREFIX_CHAR,
};
use std::fmt::{Debug, Display};
use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;
use tracing::{error, trace};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

macro_rules! change_state {
    ($inner:expr => $inner_id:ident) => {
        trace!(
            "change state (_, {:?}) => (_, {:?})",
            $inner,
            InnerState::$inner_id,
        );
        $inner = InnerState::$inner_id;
    };
    ($outer:expr, $inner:expr => $outer_id:ident, $inner_id:ident) => {
        trace!(
            "change state {:?} => {:?}",
            ($outer, $inner),
            (ParseState::$outer_id, InnerState::$inner_id)
        );
        $outer = ParseState::$outer_id;
        $inner = InnerState::$inner_id;
    };
}

macro_rules! mark_start_of_number {
    ($inner_state:expr, $i:expr => $mark:expr) => {
        if $inner_state == InnerState::Start && $i != 0 {
            $mark = $i;
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn from_str_in_span(s: &str, span: Span) -> Result<SNumber, Error> {
    let _span = ::tracing::trace_span!("from_str_in_span", s, ?span);
    let _scope = _span.enter();

    let (expecting, split_at) = if let Some(at) = s.find('/') {
        assert_eq!(str_count(s, '/'), 1);
        (Expecting::Rational, at)
    } else if s.ends_with('i') {
        assert_eq!(str_count(s, 'i'), 1);
        if let Some(at) = s.find('@') {
            assert_eq!(str_count(s, '@'), 1);
            (Expecting::PolarComplex, at)
        } else {
            (Expecting::Complex, 0)
        }
    } else {
        (Expecting::FloatOrInteger, 0)
    };
    trace!("Input {s:?} appears to be a {expecting:?}");

    if s.is_empty() {
        error!("Number may not be an empty string");
        return invalid_numeric_input(span);
    } else if s == POSITIVE_INFINITY {
        return Ok(Flonum::from(f64::INFINITY).into());
    } else if s == NEGATIVE_INFINITY {
        return Ok(Flonum::from(f64::NEG_INFINITY).into());
    } else if s == POSITIVE_NAN {
        return Ok(Flonum::from(f64::NAN).into());
    } else if s == NEGATIVE_NAN {
        return Ok(Flonum::from(-f64::NAN).into());
    }

    todo!("parse this thing")

    //    let mut outer_state = ParseState::InGeneral;
    //    let mut inner_state = InnerState::Start;
    //    let mut exactness: Option<Exactness> = None;
    //    let mut radix: Option<Radix> = None;
    //    let mut post_prefix_mark: usize = 0;
    //    let mut result = ParseResult::Fixnum;
    //
    //    for (i, c) in s.char_indices() {
    //        trace!("match ({i}, {c:?})");
    //
    //        match (outer_state, inner_state, c) {
    //            (ParseState::InGeneral, InnerState::Start, SPECIAL_PREFIX_CHAR) => {
    //                change_state!(inner_state => InPrefix);
    //            }
    //            (ParseState::InGeneral, InnerState::InPrefix, c) if Exactness::is_valid(c) => {
    //                if exactness.is_none() {
    //                    exactness = Some(Exactness::try_from(c)?);
    //                    change_state!(inner_state => Start);
    //                } else {
    //                    error!("Number cannot have more than one exactness prefix");
    //                    return invalid_numeric_input(span);
    //                }
    //            }
    //            (ParseState::InGeneral, InnerState::InPrefix, c) if Radix::is_valid(c) => {
    //                if radix.is_none() {
    //                    radix = Some(Radix::try_from(c)?);
    //                    change_state!(inner_state => Start);
    //                } else {
    //                    error!("Number cannot have more than one radix prefix");
    //                    return invalid_numeric_input(span);
    //                }
    //            }
    //            (
    //                ParseState::InGeneral,
    //                InnerState::Start | InnerState::InExponent,
    //                NUMERIC_POSITIVE | NUMERIC_NEGATIVE,
    //            ) => {
    //                mark_start_of_number!(inner_state, i => post_prefix_mark);
    //                change_state!(inner_state => InSign);
    //            }
    //            (ParseState::InGeneral, InnerState::Start, NUMERIC_DECIMAL_POINT) => {
    //                mark_start_of_number!(inner_state, i => post_prefix_mark);
    //                result = ParseResult::Flonum;
    //                change_state!(inner_state => InInitialFractional);
    //            }
    //            (ParseState::InGeneral, InnerState::InInteger, NUMERIC_DECIMAL_POINT) => {
    //                result = ParseResult::Flonum;
    //                change_state!(inner_state => InFractional);
    //            }
    //            (_, InnerState::Start | InnerState::InSign, c) if is_radix_digit(radix, c) => {
    //                mark_start_of_number!(inner_state, i => post_prefix_mark);
    //                change_state!(inner_state => InInteger);
    //            }
    //            (_, InnerState::InInitialFractional, c) if is_radix_digit(radix, c) => {
    //                change_state!(inner_state => InFractional);
    //            }
    //            (
    //                _,
    //                InnerState::InInteger
    //                | InnerState::InFractional
    //                | InnerState::InInitialFractional
    //                | InnerState::InExponent,
    //                c,
    //            ) if is_radix_digit(radix, c) => {}
    //            (_, InnerState::InInteger, c) if is_exponent_mark(radix, c) => {
    //                result = ParseResult::Flonum;
    //                change_state!(inner_state => InExponent);
    //            }
    //            (_, InnerState::InFractional, c) if is_exponent_mark(radix, c) => {
    //                result = ParseResult::Flonum;
    //                change_state!(inner_state => InExponent);
    //            }
    //            (ParseState::InGeneral, InnerState::InInteger, NUMERIC_RATIONAL_SEPARATOR) => {
    //                let lhs = Self::make(
    //                    &s[post_prefix_mark..i],
    //                    result,
    //                    exactness,
    //                    radix.unwrap_or_default(),
    //                    span,
    //                )?;
    //                result = ParseResult::Ratnum(lhs.as_fixnum().cloned().unwrap());
    //                change_state!(outer_state, inner_state => InRational, Start);
    //            }
    //            (
    //                ParseState::InGeneral,
    //                InnerState::InInteger | InnerState::InFractional,
    //                NUMERIC_POLAR_SEPARATOR,
    //            ) => {
    //                assert!(s.ends_with(NUMERIC_COMPLEX_MARK));
    //                // create lhs
    //                // set result
    //                change_state!(outer_state, inner_state => InPolarComplex, Start);
    //            }
    //            (
    //                ParseState::InGeneral,
    //                InnerState::InInteger | InnerState::InFractional,
    //                NUMERIC_POSITIVE | NUMERIC_NEGATIVE,
    //            ) => {
    //                assert!(s.ends_with(NUMERIC_COMPLEX_MARK));
    //                // create lhs
    //                // set result
    //                change_state!(outer_state, inner_state => InComplex, InSign);
    //            }
    //            (o, i, c) => {
    //                error!("Not expecting {c:?} in state ({o:?}, {i:?})");
    //                return invalid_numeric_input(span);
    //            }
    //        }
    //    }
    //
    //    Self::make(
    //        &s[post_prefix_mark..],
    //        result,
    //        exactness,
    //        radix.unwrap_or_default(),
    //        span,
    //    )
    //
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Exactness {
    Exact,
    Inexact,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Radix {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Exponent {
    base: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Expecting {
    Rational,
    PolarComplex,
    Complex,
    FloatOrInteger,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ParseState {
    InGeneral,
    InRational,
    InPolarComplex,
    InComplex,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum InnerState {
    Start,
    InPrefix,
    InSign,
    InInteger,
    InInitialFractional,
    InFractional,
    InExponentSign,
    InExponent,
}

// ------------------------------------------------------------------------------------------------
// Implementations
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
    fn is_valid(c: char) -> bool {
        matches!(c, 'e' | 'i')
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for Radix {
    #[inline(always)]
    fn default() -> Self {
        Self::Decimal
    }
}

impl Display for Radix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.mark_char())
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

impl Radix {
    #[inline(always)]
    fn is_mark_char(c: char) -> bool {
        matches!(c, 'b' | 'o' | 'd' | 'x')
    }

    #[inline(always)]
    fn mark_char(&self) -> char {
        match self {
            Radix::Binary => 'b',
            Radix::Octal => 'o',
            Radix::Decimal => 'd',
            Radix::Hexadecimal => 'x',
        }
    }

    #[inline(always)]
    fn is_valid_value(v: u32) -> bool {
        matches!(v, 2 | 8 | 10 | 16)
    }

    #[inline(always)]
    fn value(&self) -> u32 {
        match self {
            Radix::Binary => 2,
            Radix::Octal => 8,
            Radix::Decimal => 10,
            Radix::Hexadecimal => 16,
        }
    }

    #[inline(always)]
    fn is_radix_char(&self, c: char) -> bool {
        let n = match c {
            '0'..='9' => (c as u32) - ('0' as u32),
            'a'..='z' => (c as u32) - ('a' as u32) + 10,
            'A'..='Z' => (c as u32) - ('A' as u32) + 10,
            _ => 99,
        };
        n < self.value()
    }

    #[inline(always)]
    fn char_to_integer(&self, c: char) -> u32 {
        let n = match c {
            '0'..='9' => (c as u32) - ('0' as u32),
            'a'..='z' => (c as u32) - ('a' as u32) + 10,
            'A'..='Z' => (c as u32) - ('A' as u32) + 10,
            _ => 99,
        };
        assert!(n < self.value());
        n
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for Exponent {
    fn default() -> Self {
        Self { base: 10 }
    }
}

impl Exponent {
    fn base(&self) -> u32 {
        self.base
    }

    fn is_mark_char(&self, radix: Radix, c: char) -> bool {
        match (radix, c) {
            (Radix::Hexadecimal, NUMERIC_LONG_EXPONENT_MARK) => true,
            (Radix::Hexadecimal, NUMERIC_ALT_EXPONENT_MARK) => true,
            (_, NUMERIC_EXPONENT_MARK) => true,
            _ => false,
        }
    }

    fn mark_char(&self, radix: Radix) -> char {
        match radix {
            Radix::Hexadecimal => NUMERIC_LONG_EXPONENT_MARK,
            _ => NUMERIC_EXPONENT_MARK,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

//fn make(
//    s: &str,
//    expecting: ParseResult,
//    exactness: Option<Exactness>,
//    radix: Radix,
//    span: Span,
//) -> Result<Self, Error> {
//    trace!("make an {exactness:?} {radix:?} {expecting:?} from {s:?}");
//    match (&expecting, exactness) {
//        (ParseResult::Fixnum, Some(Exactness::Exact) | None) => {
//            let fixnum = integer_from_str(s, radix.into(), Some(span))?;
//            Ok(SNumber::Fixnum(Fixnum::from(fixnum)))
//        }
//        (ParseResult::Flonum, Some(Exactness::Inexact) | None) => {
//            let flonum = float_from_str(s, radix, Some(span))?;
//            Ok(SNumber::Flonum(Flonum::from(flonum)))
//        }
//        _ => {
//            error!("Couldn't make a {expecting:?}; exactness: {exactness:?}, radix: {radix}");
//            invalid_numeric_input(span)
//        }
//    }
//}

fn str_count(s: &str, target: char) -> usize {
    s.chars().filter(|c| *c == target).count()
}

fn integer_from_str<S>(s: S, radix: Radix, span: Option<Span>) -> Result<Integer, Error>
where
    S: AsRef<str>,
{
    Integer::from_str_radix(s.as_ref(), radix.value())
        .map_err(|e| invalid_integer_value(span.unwrap_or_default(), e))
}

fn fl_integer_from_str<S>(s: S, radix: Radix, span: Option<Span>) -> Result<i32, Error>
where
    S: AsRef<str>,
{
    i32::from_str_radix(s.as_ref(), radix.value())
        .map_err(|e| invalid_integer_value(span.unwrap_or_default(), e))
}

fn fraction_from_str<S>(s: S, radix: Radix, span: Option<Span>) -> Result<Float, Error>
where
    S: AsRef<str>,
{
    Ok(s.as_ref()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            radix.char_to_integer(c) as f64 * (radix.value() as f64).powi(-(i as i32 + 1))
        })
        .sum())
}

fn float_from_str<S>(s: S, radix: Radix, exp: Exponent, span: Option<Span>) -> Result<Float, Error>
where
    S: AsRef<str>,
{
    let s = s.as_ref();
    if radix == Radix::Decimal {
        Float::from_str(s).map_err(|e| invalid_float_value(span.unwrap_or_default(), e))
    } else {
        let exponent = s.find(exp.mark_char(radix)).unwrap_or(s.len());
        let decimal = s.find('.');
        let base = match decimal {
            None => Float::from(fl_integer_from_str(&s[0..exponent], radix, span)?),
            Some(0) => fraction_from_str(&s[1..exponent], radix, span)?,
            Some(n) => {
                Float::from(fl_integer_from_str(&s[0..n], radix, span)?)
                    + fraction_from_str(&s[(n + 1)..exponent], radix, span)?
            }
        };
        Ok(if exponent == s.len() {
            base
        } else {
            base * (exp
                .base()
                .pow(integer_from_str(&s[(exponent + 1)..], radix, span)? as u32)
                as Float)
        })
    }
}

#[inline]
fn invalid_integer_value(span: Span, source: ParseIntError) -> Error {
    error!("invalid_integer_value {source:?}");
    Error::InvalidNumericInput { span, source: None }
}

#[inline]
fn invalid_float_value(span: Span, source: ParseFloatError) -> Error {
    error!("invalid_float_value {source:?}");
    Error::InvalidNumericInput { span, source: None }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
