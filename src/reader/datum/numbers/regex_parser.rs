use crate::syntax::{
    NEGATIVE_INFINITY, NEGATIVE_NAN, NUMERIC_ALT_EXPONENT_MARK, NUMERIC_DECIMAL_POINT,
    NUMERIC_EXPONENT_MARK, NUMERIC_LONG_EXPONENT_MARK, NUMERIC_POLAR_SEPARATOR,
    NUMERIC_PREFIX_BINARY, NUMERIC_PREFIX_EXACT, NUMERIC_PREFIX_HEXADECIMAL,
    NUMERIC_PREFIX_INEXACT, NUMERIC_PREFIX_OCTAL, NUMERIC_RATIONAL_SEPARATOR, POSITIVE_INFINITY,
    POSITIVE_NAN,
};
use const_format::{concatcp, str_replace};
use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Cow;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Parts<'a> {
    prefix: Option<Cow<'a, str>>,
    real: Cow<'a, str>,
    imaginary: Option<Cow<'a, str>>,
    polar: Option<bool>,
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

pub fn parse(s: &str) -> Option<Parts<'_>> {
    let captures = if s.contains(NUMERIC_PREFIX_BINARY) {
        REGEX_BINARY.captures(s)
    } else if s.contains(NUMERIC_PREFIX_OCTAL) {
        REGEX_OCTAL.captures(s)
    } else if s.contains(NUMERIC_PREFIX_HEXADECIMAL) {
        REGEX_HEX.captures(s)
    } else {
        REGEX_DECIMAL.captures(s)
    };
    if let Some(captures) = captures {
        let prefix = if let Some(part) = captures.name("prefix") {
            Some(Cow::Borrowed(part.as_str()))
        } else {
            None
        };
        let (polar, imaginary) = if let Some(part) = captures.name("imaginary") {
            let part = part.as_str();
            let polar = part.starts_with(NUMERIC_POLAR_SEPARATOR);
            (
                Some(polar),
                Some(Cow::Borrowed(&part[usize::from(polar)..part.len() - 1])),
            )
        } else {
            (None, None)
        };
        Some(Parts {
            prefix,
            real: Cow::Borrowed(captures.name("real").unwrap().as_str()),
            imaginary,
            polar,
        })
    } else {
        None
    }
}

// pub(crate) fn from_str_in_span(s: &str, span: Span) -> Result<SNumber, Error> {
//     let _span = ::tracing::trace_span!("from_str_in_span", s, ?span);
//     let _scope = _span.enter();
//
//     let parts = parse(s)?;
//
//     todo!()
// }

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

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
    pub fn prefix(&self) -> Option<&Cow<'a, str>> {
        self.prefix.as_ref()
    }

    pub fn radix(&self) -> u32 {
        if let Some(prefix) = &self.prefix {
            match prefix.as_ref() {
                NUMERIC_PREFIX_BINARY => 2,
                NUMERIC_PREFIX_OCTAL => 8,
                NUMERIC_PREFIX_HEXADECIMAL => 16,
                _ => 10,
            }
        } else {
            10
        }
    }

    pub fn results_in_exact(&self) -> Option<bool> {
        if let Some(prefix) = &self.prefix {
            match prefix.as_ref() {
                NUMERIC_PREFIX_EXACT => Some(true),
                _ => Some(false),
            }
        } else {
            None
        }
    }

    pub fn results_in_inexact(&self) -> Option<bool> {
        if let Some(prefix) = &self.prefix {
            match prefix.as_ref() {
                NUMERIC_PREFIX_INEXACT => Some(true),
                _ => Some(false),
            }
        } else {
            None
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn real(&self) -> &Cow<'a, str> {
        &self.real
    }

    pub fn real_exponent(&self) -> Option<Cow<'_, str>> {
        self.exponent(self.real())
    }

    pub fn is_real_integer(&self) -> bool {
        !self.is_real_rational() && !self.is_real_float()
    }

    pub fn is_real_rational(&self) -> bool {
        self.real.contains(NUMERIC_RATIONAL_SEPARATOR)
    }

    pub fn is_real_float(&self) -> bool {
        self.real.contains(NUMERIC_DECIMAL_POINT) || self.real_exponent().is_some()
    }

    pub fn is_real_infinity(&self) -> bool {
        self.real() == POSITIVE_INFINITY || self.real() == NEGATIVE_INFINITY
    }

    pub fn is_real_nan(&self) -> bool {
        self.real() == POSITIVE_NAN || self.real() == NEGATIVE_NAN
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_complex(&self) -> bool {
        self.imaginary.is_some()
    }

    pub fn is_polar_complex(&self) -> Option<bool> {
        self.polar
    }

    pub fn is_cartesian_complex(&self) -> Option<bool> {
        self.polar.map(|b| !b)
    }

    // --------------------------------------------------------------------------------------------

    pub fn imaginary(&self) -> Option<&Cow<'a, str>> {
        self.imaginary.as_ref()
    }

    pub fn imaginary_exponent(&self) -> Option<Cow<'_, str>> {
        if let Some(imaginary) = self.imaginary() {
            self.exponent(imaginary)
        } else {
            None
        }
    }

    pub fn is_imaginary_integer(&self) -> Option<bool> {
        if self.imaginary().is_some() {
            Some(
                !self.is_imaginary_rational().unwrap_or_default()
                    && !self.is_imaginary_float().unwrap_or_default(),
            )
        } else {
            None
        }
    }

    pub fn is_imaginary_rational(&self) -> Option<bool> {
        self.imaginary
            .as_ref()
            .map(|i| i.contains(NUMERIC_RATIONAL_SEPARATOR))
    }

    pub fn is_imaginary_float(&self) -> Option<bool> {
        if self.imaginary().is_some() {
            Some(
                self.imaginary
                    .as_ref()
                    .map(|i| i.contains(NUMERIC_DECIMAL_POINT))
                    .unwrap_or_default()
                    || self.imaginary_exponent().is_some(),
            )
        } else {
            None
        }
    }

    pub fn is_imaginary_infinity(&self) -> Option<bool> {
        self.imaginary
            .as_ref()
            .map(|i| i == POSITIVE_INFINITY || i == NEGATIVE_INFINITY)
    }

    pub fn is_imaginary_nan(&self) -> Option<bool> {
        self.imaginary
            .as_ref()
            .map(|i| i == POSITIVE_NAN || i == NEGATIVE_NAN)
    }

    // --------------------------------------------------------------------------------------------

    fn exponent(&self, s: &'a str) -> Option<Cow<'a, str>> {
        let marks = [
            if self.radix() == 16 {
                NUMERIC_LONG_EXPONENT_MARK
            } else {
                NUMERIC_EXPONENT_MARK
            },
            NUMERIC_ALT_EXPONENT_MARK,
        ];
        for mark in marks {
            if let Some(index) = s.find(mark) {
                return Some(Cow::Borrowed(&s[(index + 1)..]));
            }
        }
        None
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

        assert_eq!(parts.prefix(), None);
        assert_eq!(parts.radix(), 10);
        assert_eq!(parts.results_in_exact(), None);
        assert_eq!(parts.results_in_inexact(), None);

        assert_eq!(parts.real().as_ref(), "123");
        assert_eq!(parts.real_exponent(), None);
        assert!(parts.is_real_integer());
        assert!(!parts.is_real_rational());
        assert!(!parts.is_real_float());
        assert!(!parts.is_real_infinity());
        assert!(!parts.is_real_nan());

        assert_eq!(parts.is_polar_complex(), None);
        assert_eq!(parts.is_cartesian_complex(), None);

        assert_eq!(parts.imaginary(), None);
        assert_eq!(parts.imaginary_exponent(), None);
        assert_eq!(parts.is_imaginary_integer(), None);
        assert_eq!(parts.is_imaginary_rational(), None);
        assert_eq!(parts.is_imaginary_float(), None);
        assert_eq!(parts.is_imaginary_infinity(), None);
        assert_eq!(parts.is_imaginary_nan(), None);
    }

    #[test]
    fn test_parse_rational() {
        let parts = parse("1/23").unwrap();

        assert_eq!(parts.prefix(), None);
        assert_eq!(parts.radix(), 10);
        assert_eq!(parts.results_in_exact(), None);
        assert_eq!(parts.results_in_inexact(), None);

        assert_eq!(parts.real().as_ref(), "1/23");
        assert_eq!(parts.real_exponent(), None);
        assert!(!parts.is_real_integer());
        assert!(parts.is_real_rational());
        assert!(!parts.is_real_float());
        assert!(!parts.is_real_infinity());
        assert!(!parts.is_real_nan());

        assert_eq!(parts.is_polar_complex(), None);
        assert_eq!(parts.is_cartesian_complex(), None);

        assert_eq!(parts.imaginary(), None);
        assert_eq!(parts.imaginary_exponent(), None);
        assert_eq!(parts.is_imaginary_integer(), None);
        assert_eq!(parts.is_imaginary_rational(), None);
        assert_eq!(parts.is_imaginary_float(), None);
        assert_eq!(parts.is_imaginary_infinity(), None);
        assert_eq!(parts.is_imaginary_nan(), None);
    }

    #[test]
    fn test_parse_float() {
        let parts = parse("12.3").unwrap();

        assert_eq!(parts.prefix(), None);
        assert_eq!(parts.radix(), 10);
        assert_eq!(parts.results_in_exact(), None);
        assert_eq!(parts.results_in_inexact(), None);

        assert_eq!(parts.real().as_ref(), "12.3");
        assert_eq!(parts.real_exponent(), None);
        assert!(!parts.is_real_integer());
        assert!(!parts.is_real_rational());
        assert!(parts.is_real_float());
        assert!(!parts.is_real_infinity());
        assert!(!parts.is_real_nan());

        assert_eq!(parts.is_polar_complex(), None);
        assert_eq!(parts.is_cartesian_complex(), None);

        assert_eq!(parts.imaginary(), None);
        assert_eq!(parts.imaginary_exponent(), None);
        assert_eq!(parts.is_imaginary_integer(), None);
        assert_eq!(parts.is_imaginary_rational(), None);
        assert_eq!(parts.is_imaginary_float(), None);
        assert_eq!(parts.is_imaginary_infinity(), None);
        assert_eq!(parts.is_imaginary_nan(), None);
    }

    #[test]
    fn test_parse_float_exp() {
        let parts = parse("12.3e-4").unwrap();

        assert_eq!(parts.prefix(), None);
        assert_eq!(parts.radix(), 10);
        assert_eq!(parts.results_in_exact(), None);
        assert_eq!(parts.results_in_inexact(), None);

        assert_eq!(parts.real().as_ref(), "12.3e-4");
        assert_eq!(parts.real_exponent(), Some(Cow::Borrowed("-4")));
        assert!(!parts.is_real_integer());
        assert!(!parts.is_real_rational());
        assert!(parts.is_real_float());
        assert!(!parts.is_real_infinity());
        assert!(!parts.is_real_nan());

        assert_eq!(parts.is_polar_complex(), None);
        assert_eq!(parts.is_cartesian_complex(), None);

        assert_eq!(parts.imaginary(), None);
        assert_eq!(parts.imaginary_exponent(), None);
        assert_eq!(parts.is_imaginary_integer(), None);
        assert_eq!(parts.is_imaginary_rational(), None);
        assert_eq!(parts.is_imaginary_float(), None);
        assert_eq!(parts.is_imaginary_infinity(), None);
        assert_eq!(parts.is_imaginary_nan(), None);
    }

    #[test]
    fn test_parse_complex() {
        let parts = parse("3.0+4.0i").unwrap();

        assert_eq!(parts.prefix(), None);
        assert_eq!(parts.radix(), 10);
        assert_eq!(parts.results_in_exact(), None);
        assert_eq!(parts.results_in_inexact(), None);

        assert_eq!(parts.real().as_ref(), "3.0");
        assert_eq!(parts.real_exponent(), None);
        assert!(!parts.is_real_integer());
        assert!(!parts.is_real_rational());
        assert!(parts.is_real_float());
        assert!(!parts.is_real_infinity());
        assert!(!parts.is_real_nan());

        assert_eq!(parts.is_polar_complex(), Some(false));
        assert_eq!(parts.is_cartesian_complex(), Some(true));

        assert_eq!(parts.imaginary().unwrap().as_ref(), "+4.0");
        assert_eq!(parts.imaginary_exponent(), None);
        assert_eq!(parts.is_imaginary_integer(), Some(false));
        assert_eq!(parts.is_imaginary_rational(), Some(false));
        assert_eq!(parts.is_imaginary_float(), Some(true));
        assert_eq!(parts.is_imaginary_infinity(), Some(false));
        assert_eq!(parts.is_imaginary_nan(), Some(false));
    }
}
