use crate::syntax::{
    NEGATIVE_INFINITY, NEGATIVE_NAN, NUMERIC_ALT_EXPONENT_MARK, NUMERIC_DECIMAL_POINT,
    NUMERIC_EXPONENT_MARK, NUMERIC_LONG_EXPONENT_MARK, NUMERIC_NEGATIVE, NUMERIC_POLAR_SEPARATOR,
    NUMERIC_PREFIX_BINARY, NUMERIC_PREFIX_EXACT, NUMERIC_PREFIX_HEXADECIMAL,
    NUMERIC_PREFIX_INEXACT, NUMERIC_PREFIX_OCTAL, NUMERIC_RATIONAL_SEPARATOR, POSITIVE_INFINITY,
    POSITIVE_NAN,
};
use const_format::{concatcp, str_replace};
use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Cow;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub(crate) struct Parts<'a> {
    is_exact: Option<bool>,
    is_polar_complex: Option<bool>,
    real: Part<'a>,
    imaginary: Option<Part<'a>>,
}

#[derive(Debug)]
pub(crate) struct Part<'a> {
    value: Cow<'a, str>,
    radix: u32,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn has_valid_prefix(s: &str) -> bool {
    REGEX_PREFIX.is_match(s)
}

pub fn is_valid(s: &str) -> bool {
    if s.contains(NUMERIC_PREFIX_BINARY) {
        is_valid_binary(s)
    } else if s.contains(NUMERIC_PREFIX_OCTAL) {
        is_valid_octal(s)
    } else if s.contains(NUMERIC_PREFIX_HEXADECIMAL) {
        is_valid_hexadecimal(s)
    } else {
        is_valid_decimal(s)
    }
}

pub fn is_valid_binary(s: &str) -> bool {
    REGEX_BINARY.is_match(s)
}

pub fn is_valid_octal(s: &str) -> bool {
    REGEX_OCTAL.is_match(s)
}

pub fn is_valid_decimal(s: &str) -> bool {
    REGEX_DECIMAL.is_match(s)
}

pub fn is_valid_hexadecimal(s: &str) -> bool {
    REGEX_HEX.is_match(s)
}

pub(crate) fn parse(s: &str) -> Option<Parts<'_>> {
    let (captures, radix) = if s.contains(NUMERIC_PREFIX_BINARY) {
        (REGEX_BINARY.captures(s), 2)
    } else if s.contains(NUMERIC_PREFIX_OCTAL) {
        (REGEX_OCTAL.captures(s), 8)
    } else if s.contains(NUMERIC_PREFIX_HEXADECIMAL) {
        (REGEX_HEX.captures(s), 16)
    } else {
        (REGEX_DECIMAL.captures(s), 10)
    };
    if let Some(captures) = captures {
        let is_exact = if let Some(part) = captures.name("prefix") {
            let part = part.as_str();
            if part.contains(NUMERIC_PREFIX_EXACT) {
                Some(true)
            } else if part.contains(NUMERIC_PREFIX_INEXACT) {
                Some(false)
            } else {
                None
            }
        } else {
            None
        };
        let (is_polar_complex, imaginary) = if let Some(part) = captures.name("imaginary") {
            let part = part.as_str();
            let polar = part.starts_with(NUMERIC_POLAR_SEPARATOR);
            (
                Some(polar),
                Some(Part {
                    value: Cow::Borrowed(&part[usize::from(polar)..part.len() - 1]),
                    radix,
                }),
            )
        } else {
            (None, None)
        };
        Some(Parts {
            is_exact,
            is_polar_complex,
            real: Part {
                value: Cow::Borrowed(captures.name("real").unwrap().as_str()),
                radix,
            },
            imaginary,
        })
    } else {
        None
    }
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

const REGEX_FLAG_STR: &str = r##"(?ix)"##;

const REGEX_PREFIX_STR: &str = r##"^
    (?P<prefix>
      (?:
        (?:
          # Exactness
          \#[ei]
          # Radix
          (?:\#[bodx])?
        )
      )
      |
      (?:
        (?:
          # Radix
          \#[bodx]
          # Exactness
          (?:\#[ei])?
        )
      )
    )?"##;

const REGEX_TEMPLATE_STR: &str = r##"
    (?P<real>
      (?:
        # Optional sign
        [+-]?
        (?:
          # Rational number
          (?:[[:xdigit:]]+/[[:xdigit:]]+)
          |
          (?:
            (?:
              # Real, with optional fractional part
              (?:
                [[:xdigit:]]+
                (?:\.[[:xdigit:]]*)?
              )
              |
              # Real, with optional whole part
              (?:\.[[:xdigit:]]+)
            )
            # Real exponent
            (?:[el^][+-]?[[:xdigit:]]+)?
          )
        )
      )
      |
      # Infinities and NaNs
      (?:
        # Sign is required for these
        [+-](?:inf|nan)\.0
      )
    )
    (?P<imaginary>
      (?:
        (?:
          (?:
            # Polar complex, sign is optional
            @[+-]?
            |
            # Cartesian complex, sign is required
            [+-]
          )
          (?:
            # Rational number
            (?:[[:xdigit:]]+/[[:xdigit:]]+)
            |
            (?:
              (?:
                # Real, with optional fractional part
                (?:
                  [[:xdigit:]]+
                  (?:\.[[:xdigit:]]*)?
                )
                |
                # Real, with optional whole part
                (?:\.[[:xdigit:]]+)
              )
              # Real exponent
              (?:[el^][+-]?[[:xdigit:]]+)?
            )
          )
        )
        |
        # Infinities and NaNs
        (?:
          @?
          # Sign is required for these
          [+-](?:inf|nan)\.0
        )
      )
      # Complex number suffix
      i
    )?
    $
    "##;

const REGEX_ANY_STR: &str = concatcp!(REGEX_FLAG_STR, REGEX_PREFIX_STR, REGEX_TEMPLATE_STR);

const REGEX_BINARY_STR: &str = concatcp!(
    REGEX_FLAG_STR,
    str_replace!(REGEX_PREFIX_STR, "[bodx]", "b"),
    str_replace!(
        str_replace!(REGEX_TEMPLATE_STR, "[:xdigit:]", "0-1"),
        "[el^]",
        "[e^]"
    )
);

const REGEX_OCTAL_STR: &str = concatcp!(
    REGEX_FLAG_STR,
    str_replace!(REGEX_PREFIX_STR, "[bodx]", "o"),
    str_replace!(
        str_replace!(REGEX_TEMPLATE_STR, "[:xdigit:]", "0-7"),
        "[el^]",
        "[e^]"
    )
);

const REGEX_DECIMAL_STR: &str = concatcp!(
    REGEX_FLAG_STR,
    str_replace!(REGEX_PREFIX_STR, "[bodx]", "d"),
    str_replace!(
        str_replace!(REGEX_TEMPLATE_STR, "[:xdigit:]", "0-9"),
        "[el^]",
        "[e^]"
    )
);

const REGEX_HEX_STR: &str = concatcp!(
    REGEX_FLAG_STR,
    str_replace!(REGEX_PREFIX_STR, "[bodx]", "x"),
    str_replace!(
        str_replace!(REGEX_TEMPLATE_STR, "[:xdigit:]", "0-9a-f"),
        "[el^]",
        "[l^]"
    )
);

lazy_static! {
    static ref REGEX_PREFIX: Regex = Regex::new(REGEX_PREFIX_STR).unwrap();
    static ref REGEX_ANY: Regex = Regex::new(REGEX_ANY_STR).unwrap();
    static ref REGEX_BINARY: Regex = Regex::new(REGEX_BINARY_STR).unwrap();
    static ref REGEX_OCTAL: Regex = Regex::new(REGEX_OCTAL_STR).unwrap();
    static ref REGEX_DECIMAL: Regex = Regex::new(REGEX_DECIMAL_STR).unwrap();
    static ref REGEX_HEX: Regex = Regex::new(REGEX_HEX_STR).unwrap();
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<'a> Parts<'a> {
    #[inline(always)]
    pub(crate) fn results_in_exact(&self) -> Option<bool> {
        self.is_exact
    }

    #[inline(always)]
    pub(crate) fn results_in_inexact(&self) -> Option<bool> {
        self.is_exact.map(|b| !b)
    }

    // --------------------------------------------------------------------------------------------

    #[inline(always)]
    pub(crate) fn real(&self) -> &Part<'a> {
        &self.real
    }

    // --------------------------------------------------------------------------------------------

    #[inline(always)]
    pub(crate) fn is_polar_complex(&self) -> Option<bool> {
        self.is_polar_complex
    }

    // --------------------------------------------------------------------------------------------

    #[inline(always)]
    pub(crate) fn imaginary(&self) -> Option<&Part<'a>> {
        self.imaginary.as_ref()
    }
}

// ------------------------------------------------------------------------------------------------

impl<'a> Part<'a> {
    #[inline(always)]
    pub(crate) fn value(&self) -> &Cow<'a, str> {
        &self.value
    }

    pub(crate) fn exponent_mark(&self) -> Option<(char, usize)> {
        let marks = [
            if self.radix == 16 {
                NUMERIC_LONG_EXPONENT_MARK
            } else {
                NUMERIC_EXPONENT_MARK
            },
            NUMERIC_ALT_EXPONENT_MARK,
        ];
        for mark in marks {
            if let Some(index) = self.value.find(mark) {
                return Some((mark, index));
            }
        }
        None
    }

    #[inline(always)]
    pub(crate) fn exponent_value(&self) -> Option<Cow<'_, str>> {
        if let Some((_, index)) = self.exponent_mark() {
            Some(Cow::Borrowed(&self.value[(index + 1)..]))
        } else {
            None
        }
    }

    #[inline(always)]
    pub(crate) fn is_sign_negative(&self) -> bool {
        self.value.starts_with(NUMERIC_NEGATIVE)
    }

    #[inline(always)]
    pub(crate) fn is_sign_positive(&self) -> bool {
        !self.is_sign_negative()
    }

    #[inline(always)]
    pub(crate) fn is_integer(&self) -> bool {
        !self.is_rational() && !self.is_float()
    }

    #[inline(always)]
    pub(crate) fn is_rational(&self) -> bool {
        self.value.contains(NUMERIC_RATIONAL_SEPARATOR)
    }

    #[inline(always)]
    pub(crate) fn is_float(&self) -> bool {
        self.value.contains(NUMERIC_DECIMAL_POINT) || self.exponent_value().is_some()
    }

    #[inline(always)]
    pub(crate) fn is_infinity(&self) -> bool {
        self.value == POSITIVE_INFINITY || self.value == NEGATIVE_INFINITY
    }

    #[inline(always)]
    pub(crate) fn is_nan(&self) -> bool {
        self.value == POSITIVE_NAN || self.value == NEGATIVE_NAN
    }

    #[inline(always)]
    pub(crate) fn radix(&self) -> u32 {
        self.radix
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let passes = [
            "1",
            "#e1",
            "#x#e2a",
            "1.0",
            ".1",
            "1.",
            "1.1^36",
            "1/2",
            "-1/2",
            "1/2-2/3i",
            "1/2+2/3i",
            "1.0@2.0i",
            "1.0",
            "1^10",
            "1e-10",
            "#x1l-10",
            "#x#i1/2+1L-10i",
            "-inf.0",
            "+nan.0",
            "-inf.0+inf.0i",
            "+nan.0@-nan.0i",
        ];
        println!("!!! {REGEX_ANY_STR:?}");
        for example in passes {
            assert!(is_valid(example));
            for cap in REGEX_ANY.captures_iter(example) {
                println!("{example:?} ==> {cap:?}");
            }
        }
    }

    #[test]
    fn test_binary_correct() {
        let passes = [
            "#b101", "#e#b101", "#b#e101", "#i#b101", "#b#i101", "#b11/100",
        ];
        println!("!!! {REGEX_BINARY_STR:?}");
        for example in passes {
            assert!(is_valid_binary(example));
            for cap in REGEX_BINARY.captures_iter(example) {
                println!("{example:?} ==> {cap:?}");
            }
        }
    }

    #[test]
    fn test_binary_incorrect() {
        let fails = [
            "#b102", "#e#b121", "#b#e121", "#i#b121", "#b#i121", "#b11/120",
        ];
        for example in fails {
            println!("{example:?}");
            assert!(!is_valid_binary(example));
        }
    }

    #[test]
    fn test_parse_integer() {
        let parts = parse("123").unwrap();

        assert_eq!(parts.results_in_exact(), None);
        assert_eq!(parts.results_in_inexact(), None);
        assert_eq!(parts.is_polar_complex(), None);

        assert_eq!(parts.real().radix(), 10);
        assert_eq!(parts.real().value(), "123");
        assert_eq!(parts.real().exponent_value(), None);
        assert!(parts.real().is_integer());
        assert!(!parts.real().is_rational());
        assert!(!parts.real().is_float());
        assert!(!parts.real().is_infinity());
        assert!(!parts.real().is_nan());

        assert!(parts.imaginary().is_none());
    }

    #[test]
    fn test_parse_rational() {
        let parts = parse("1/23").unwrap();

        assert_eq!(parts.results_in_exact(), None);
        assert_eq!(parts.results_in_inexact(), None);
        assert_eq!(parts.is_polar_complex(), None);

        assert_eq!(parts.real().radix(), 10);
        assert_eq!(parts.real().value(), "1/23");
        assert_eq!(parts.real().exponent_value(), None);
        assert!(!parts.real().is_integer());
        assert!(parts.real().is_rational());
        assert!(!parts.real().is_float());
        assert!(!parts.real().is_infinity());
        assert!(!parts.real().is_nan());

        assert!(parts.imaginary().is_none());
    }

    #[test]
    fn test_parse_float() {
        let parts = parse("12.3").unwrap();

        assert_eq!(parts.results_in_exact(), None);
        assert_eq!(parts.results_in_inexact(), None);
        assert_eq!(parts.is_polar_complex(), None);

        assert_eq!(parts.real().radix(), 10);
        assert_eq!(parts.real().value(), "12.3");
        assert_eq!(parts.real().exponent_value(), None);
        assert!(!parts.real().is_integer());
        assert!(!parts.real().is_rational());
        assert!(parts.real().is_float());
        assert!(!parts.real().is_infinity());
        assert!(!parts.real().is_nan());

        assert!(parts.imaginary().is_none());
    }

    #[test]
    fn test_parse_float_exp() {
        let parts = parse("12.3e-4").unwrap();

        assert_eq!(parts.results_in_exact(), None);
        assert_eq!(parts.results_in_inexact(), None);
        assert_eq!(parts.is_polar_complex(), None);

        assert_eq!(parts.real().radix(), 10);
        assert_eq!(parts.real().value(), "12.3e-4");
        assert_eq!(parts.real().exponent_value(), Some(Cow::Borrowed("-4")));
        assert!(!parts.real().is_integer());
        assert!(!parts.real().is_rational());
        assert!(parts.real().is_float());
        assert!(!parts.real().is_infinity());
        assert!(!parts.real().is_nan());

        assert!(parts.imaginary().is_none());
    }

    #[test]
    fn test_parse_complex() {
        let parts = parse("3.0+4.0i").unwrap();

        assert_eq!(parts.results_in_exact(), None);
        assert_eq!(parts.results_in_inexact(), None);
        assert_eq!(parts.is_polar_complex(), Some(false));

        assert_eq!(parts.real().radix(), 10);
        assert_eq!(parts.real().value(), "3.0");
        assert_eq!(parts.real().exponent_value(), None);
        assert!(!parts.real().is_integer());
        assert!(!parts.real().is_rational());
        assert!(parts.real().is_float());
        assert!(!parts.real().is_infinity());
        assert!(!parts.real().is_nan());

        let imaginary = parts.imaginary().expect("There's no imaginary part!");

        assert_eq!(imaginary.radix(), 10);
        assert_eq!(imaginary.value(), "+4.0");
        assert_eq!(imaginary.exponent_value(), None);
        assert!(!imaginary.is_integer());
        assert!(!imaginary.is_rational());
        assert!(imaginary.is_float());
        assert!(!imaginary.is_infinity());
        assert!(!imaginary.is_nan());
    }
}
