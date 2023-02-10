use crate::error::{invalid_numeric_input, Error};
use crate::lexer::token::Span;
use crate::reader::datum::numbers::{Complexnum, Fixnum, Float, Flonum, Integer, Ratnum, SNumber};
use crate::syntax::{
    NEGATIVE_INFINITY, NEGATIVE_NAN, NUMERIC_COMPLEX_MARK, NUMERIC_DECIMAL_POINT,
    NUMERIC_EXPONENT_MARK, NUMERIC_LONG_EXPONENT_MARK, NUMERIC_NEGATIVE, NUMERIC_POLAR_SEPARATOR,
    NUMERIC_POSITIVE, NUMERIC_RATIONAL_SEPARATOR, POSITIVE_INFINITY, POSITIVE_NAN,
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

pub fn from_str_in_span(s: &str, span: Span) -> Result<SNumber, Error> {
    let _span = ::tracing::trace_span!("from_str_in_span", s, ?span);
    let _scope = _span.enter();

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

    let mut outer_state = ParseState::InGeneral;
    let mut inner_state = InnerState::Start;
    let mut exactness: Option<Exactness> = None;
    let mut radix: Option<Radix> = None;
    let mut post_prefix_mark: usize = 0;
    let mut result = ParseResult::Fixnum;

    for (i, c) in s.char_indices() {
        trace!("match ({i}, {c:?})");

        match (outer_state, inner_state, c) {
            (ParseState::InGeneral, InnerState::Start, SPECIAL_PREFIX_CHAR) => {
                change_state!(inner_state => InPrefix);
            }
            (ParseState::InGeneral, InnerState::InPrefix, c) if Exactness::is_valid(c) => {
                if exactness.is_none() {
                    exactness = Some(Exactness::try_from(c)?);
                    change_state!(inner_state => Start);
                } else {
                    error!("Number cannot have more than one exactness prefix");
                    return invalid_numeric_input(span);
                }
            }
            (ParseState::InGeneral, InnerState::InPrefix, c) if Radix::is_valid(c) => {
                if radix.is_none() {
                    radix = Some(Radix::try_from(c)?);
                    change_state!(inner_state => Start);
                } else {
                    error!("Number cannot have more than one radix prefix");
                    return invalid_numeric_input(span);
                }
            }
            (
                ParseState::InGeneral,
                InnerState::Start | InnerState::InExponent,
                NUMERIC_POSITIVE | NUMERIC_NEGATIVE,
            ) => {
                mark_start_of_number!(inner_state, i => post_prefix_mark);
                change_state!(inner_state => InSign);
            }
            (ParseState::InGeneral, InnerState::Start, NUMERIC_DECIMAL_POINT) => {
                mark_start_of_number!(inner_state, i => post_prefix_mark);
                result = ParseResult::Flonum;
                change_state!(inner_state => InInitialFractional);
            }
            (ParseState::InGeneral, InnerState::InInteger, NUMERIC_DECIMAL_POINT) => {
                result = ParseResult::Flonum;
                change_state!(inner_state => InFractional);
            }
            (_, InnerState::Start | InnerState::InSign, c) if is_radix_digit(radix, c) => {
                mark_start_of_number!(inner_state, i => post_prefix_mark);
                change_state!(inner_state => InInteger);
            }
            (_, InnerState::InInitialFractional, c) if is_radix_digit(radix, c) => {
                change_state!(inner_state => InFractional);
            }
            (
                _,
                InnerState::InInteger
                | InnerState::InFractional
                | InnerState::InInitialFractional
                | InnerState::InExponent,
                c,
            ) if is_radix_digit(radix, c) => {}
            (_, InnerState::InInteger, c) if is_exponent_mark(radix, c) => {
                result = ParseResult::Flonum;
                change_state!(inner_state => InExponent);
            }
            (_, InnerState::InFractional, c) if is_exponent_mark(radix, c) => {
                result = ParseResult::Flonum;
                change_state!(inner_state => InExponent);
            }
            (ParseState::InGeneral, InnerState::InInteger, NUMERIC_RATIONAL_SEPARATOR) => {
                let lhs = Self::make(
                    &s[post_prefix_mark..i],
                    result,
                    exactness,
                    radix.unwrap_or_default(),
                    span,
                )?;
                result = ParseResult::Ratnum(lhs.as_fixnum().cloned().unwrap());
                change_state!(outer_state, inner_state => InRational, Start);
            }
            (
                ParseState::InGeneral,
                InnerState::InInteger | InnerState::InFractional,
                NUMERIC_POLAR_SEPARATOR,
            ) => {
                assert!(s.ends_with(NUMERIC_COMPLEX_MARK));
                // create lhs
                // set result
                change_state!(outer_state, inner_state => InPolarComplex, Start);
            }
            (
                ParseState::InGeneral,
                InnerState::InInteger | InnerState::InFractional,
                NUMERIC_POSITIVE | NUMERIC_NEGATIVE,
            ) => {
                assert!(s.ends_with(NUMERIC_COMPLEX_MARK));
                // create lhs
                // set result
                change_state!(outer_state, inner_state => InComplex, InSign);
            }
            (o, i, c) => {
                error!("Not expecting {c:?} in state ({o:?}, {i:?})");
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

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

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
// Private Functions
// ------------------------------------------------------------------------------------------------

fn make(
    s: &str,
    expecting: ParseResult,
    exactness: Option<Exactness>,
    radix: Radix,
    span: Span,
) -> Result<Self, Error> {
    trace!("make an {exactness:?} {radix:?} {expecting:?} from {s:?}");
    match (&expecting, exactness) {
        (ParseResult::Fixnum, Some(Exactness::Exact) | None) => {
            let fixnum = integer_from_str(s, radix.into(), Some(span))?;
            Ok(SNumber::Fixnum(Fixnum::from(fixnum)))
        }
        (ParseResult::Flonum, Some(Exactness::Inexact) | None) => {
            let flonum = float_from_str(s, radix, Some(span))?;
            Ok(SNumber::Flonum(Flonum::from(flonum)))
        }
        _ => {
            error!("Couldn't make a {expecting:?}; exactness: {exactness:?}, radix: {radix}");
            invalid_numeric_input(span)
        }
    }
}

fn integer_from_str<S>(s: S, radix: Radix, span: Option<Span>) -> Result<Integer, Error>
where
    S: AsRef<str>,
{
    Ok(Integer::from_str_radix(s.as_ref(), radix.into())
        .or_else(|e| Err(invalid_integer_value(span.unwrap_or_default(), e)))?)
}

fn float_from_str<S>(s: S, radix: Radix, span: Option<Span>) -> Result<Float, Error>
where
    S: AsRef<str>,
{
    assert_eq!(radix, Radix::Decimal);
    Ok(Float::from_str(s.as_ref())
        .or_else(|e| Err(invalid_float_value(span.unwrap_or_default(), e)))?)
}

#[inline(always)]
fn is_radix_digit(radix: Option<Radix>, c: char) -> bool {
    match (radix.unwrap_or_default(), c) {
        (Radix::Binary, '0' | '1') => true,
        (Radix::Octal, '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8') => true,
        (Radix::Decimal, c) if c.is_ascii_digit() => true,
        (Radix::Hexadecimal, c) if c.is_ascii_hexdigit() => true,
        _ => false,
    }
}

#[inline(always)]
fn is_exponent_mark(radix: Option<Radix>, c: char) -> bool {
    if (radix.unwrap_or_default() == Radix::Hexadecimal && c == NUMERIC_LONG_EXPONENT_MARK)
        || c == NUMERIC_EXPONENT_MARK
    // TODO: should support EITHER || c == NUMERIC_LONG_EXPONENT_MARK
    {
        true
    } else {
        false
    }
}

#[inline]
pub fn invalid_integer_value(span: Span, source: ParseIntError) -> Error {
    error!("invalid_integer_value {source:?}");
    Error::InvalidNumericInput { span, source: None }
}

#[inline]
pub fn invalid_float_value(span: Span, source: ParseFloatError) -> Error {
    error!("invalid_float_value {source:?}");
    Error::InvalidNumericInput { span, source: None }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
