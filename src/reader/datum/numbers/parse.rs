use crate::error::Error;
use crate::lexer::token::Span;
use crate::reader::datum::numbers::{
    ratnum::Rational, Complexnum, Fixnum, Float, Flonum, Integer, Ratnum, SNumber,
};
use crate::syntax::NUMERIC_RATIONAL_SEPARATOR;
use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;
use tracing::{error, trace};

#[cfg(feature = "regex-parser")]
use crate::reader::datum::numbers::regex_parser::{parse, Part};

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

#[cfg(feature = "regex-parser")]
pub(crate) fn from_str_in_span(s: &str, span: Span) -> Result<SNumber, Error> {
    let _span = ::tracing::trace_span!("from_str_in_span", s, ?span);
    let _scope = _span.enter();

    let parts = parse(s).ok_or(Error::InvalidNumericInput { span, source: None })?;

    let real = from_part(parts.real(), span)?;

    trace!(
        "results need to be cast as exact? {:?}, inexact? {:?}",
        parts.results_in_exact(),
        parts.results_in_inexact(),
    );

    if let Some(imaginary) = parts.imaginary() {
        let imaginary = from_part(imaginary, span)?;
        if parts.is_polar_complex().unwrap() {
            Ok(Complexnum::new_polar(real.cast_as_flonum(), imaginary.cast_as_flonum()).into())
        } else {
            Ok(Complexnum::new(real.cast_as_flonum(), imaginary.cast_as_flonum()).into())
        }
    } else {
        Ok(real)
    }

    // TODO: cast result!
}

#[cfg(not(feature = "regex-parser"))]
pub(crate) fn from_str_in_span(s: &str, span: Span) -> Result<SNumber, Error> {
    let _span = ::tracing::trace_span!("from_str_in_span", s, ?span);
    let _scope = _span.enter();

    todo!()
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

const RADIX_DECIMAL: u32 = 10;
const EXPONENT_BASE: u32 = 10;

#[cfg(feature = "regex-parser")]
fn from_part(part: &Part<'_>, span: Span) -> Result<SNumber, Error> {
    if part.is_infinity() {
        if part.is_sign_positive() {
            Ok(Flonum::from(f64::INFINITY).into())
        } else {
            Ok(Flonum::from(f64::NEG_INFINITY).into())
        }
    } else if part.is_nan() {
        if part.is_sign_positive() {
            Ok(Flonum::from(f64::NAN).into())
        } else {
            Ok(Flonum::from(-f64::NAN).into())
        }
    } else if part.is_integer() {
        Ok(Fixnum::from(integer_from_str(part.value(), part.radix(), span)?).into())
    } else if part.is_rational() {
        Ok(Ratnum::from(rational_from_str(part.value(), part.radix(), span)?).into())
    } else if part.is_float() {
        Ok(Flonum::from(float_from_str(
            part.value(),
            part.radix(),
            part.exponent_mark().map(|(_, i)| i),
            span,
        )?)
        .into())
    } else {
        unreachable!()
    }
}

fn float_from_str<S>(s: S, radix: u32, exponent: Option<usize>, span: Span) -> Result<Float, Error>
where
    S: AsRef<str>,
{
    let s = s.as_ref();
    if radix == RADIX_DECIMAL {
        Float::from_str(s).map_err(|e| invalid_float_value(span, e))
    } else {
        let exponent = exponent.unwrap_or(s.len());
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
            base * (EXPONENT_BASE.pow(integer_from_str(&s[(exponent + 1)..], radix, span)? as u32)
                as Float)
        })
    }
}

#[inline]
fn integer_from_str<S>(s: S, radix: u32, span: Span) -> Result<Integer, Error>
where
    S: AsRef<str>,
{
    Integer::from_str_radix(s.as_ref(), radix).map_err(|e| invalid_integer_value(span, e))
}

#[inline]
fn rational_from_str<S>(s: S, radix: u32, span: Span) -> Result<Rational, Error>
where
    S: AsRef<str>,
{
    let mut parts = s.as_ref().split(NUMERIC_RATIONAL_SEPARATOR);
    let numer = Integer::from_str_radix(parts.next().unwrap(), radix)
        .map_err(|e| invalid_integer_value(span, e))?;
    let denom = Integer::from_str_radix(parts.next().unwrap(), radix)
        .map_err(|e| invalid_integer_value(span, e))?;

    Ok(Rational::new(numer, denom))
}

#[inline]
fn fl_integer_from_str<S>(s: S, radix: u32, span: Span) -> Result<i32, Error>
where
    S: AsRef<str>,
{
    i32::from_str_radix(s.as_ref(), radix).map_err(|e| invalid_integer_value(span, e))
}

#[inline]
fn fraction_from_str<S>(s: S, radix: u32, _span: Span) -> Result<Float, Error>
where
    S: AsRef<str>,
{
    Ok(s.as_ref()
        .chars()
        .enumerate()
        .map(|(i, c)| radix_char_to_integer(radix, c) as f64 * (radix as f64).powi(-(i as i32 + 1)))
        .sum())
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

#[inline(always)]
fn radix_char_to_integer(radix: u32, c: char) -> u32 {
    let n = match c {
        '0'..='9' => (c as u32) - ('0' as u32),
        'a'..='z' => (c as u32) - ('a' as u32) + 10,
        'A'..='Z' => (c as u32) - ('A' as u32) + 10,
        _ => 99,
    };
    assert!(n < radix);
    n
}
