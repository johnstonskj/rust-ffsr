use ffsr::reader::datum::{Fixnum, Flonum, SNumber};
use pretty_assertions::assert_eq;

// ------------------------------------------------------------------------------------------------
// Single-valued success cases
// ------------------------------------------------------------------------------------------------

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

// ------------------------------------------------------------------------------------------------

success_case!(!fixnum_zero, "0" => SNumber, SNumber::Fixnum(Fixnum::from(0)));

success_case!(!fixnum, "9762457" => SNumber, SNumber::Fixnum(Fixnum::from(9762457)));

success_case!(!fixnum_exact, "#e9762457" => SNumber, SNumber::Fixnum(Fixnum::from(9762457)));

success_case!(!fixnum_pos, "+9762457" => SNumber, SNumber::Fixnum(Fixnum::from(9762457)));

success_case!(!fixnum_exact_pos, "#e+9762457" => SNumber, SNumber::Fixnum(Fixnum::from(9762457)));

success_case!(!fixnum_exact_neg, "#e-9762457" => SNumber, SNumber::Fixnum(Fixnum::from(-9762457)));

// ------------------------------------------------------------------------------------------------

success_case!(!fixnum_binary, "#b1010" => SNumber, SNumber::Fixnum(Fixnum::from(0b1010)));

success_case!(!fixnum_exact_binary, "#e#b1010" => SNumber, SNumber::Fixnum(Fixnum::from(0b1010)));

success_case!(!fixnum_binary_exact, "#b#e1010" => SNumber, SNumber::Fixnum(Fixnum::from(0b1010)));

// ------------------------------------------------------------------------------------------------

success_case!(!fixnum_octal, "#o12345670" => SNumber, SNumber::Fixnum(Fixnum::from(0o12345670)));

success_case!(!fixnum_exact_octal, "#e#o12345670" => SNumber, SNumber::Fixnum(Fixnum::from(0o12345670)));

success_case!(!fixnum_octal_exact, "#o#e12345670" => SNumber, SNumber::Fixnum(Fixnum::from(0o12345670)));

// ------------------------------------------------------------------------------------------------

success_case!(!fixnum_decimal, "#d1234567890" => SNumber, SNumber::Fixnum(Fixnum::from(1234567890)));

success_case!(!fixnum_exact_decimal, "#e#d1234567890" => SNumber, SNumber::Fixnum(Fixnum::from(1234567890)));

success_case!(!fixnum_decimal_exact, "#d#e1234567890" => SNumber, SNumber::Fixnum(Fixnum::from(1234567890)));

// ------------------------------------------------------------------------------------------------

success_case!(!fixnum_hex, "#x123456789abcdef0" => SNumber, SNumber::Fixnum(Fixnum::from(0x123456789abcdef0_i64)));

success_case!(!fixnum_exact_hex, "#e#x123456789abcdef0" => SNumber, SNumber::Fixnum(Fixnum::from(0x123456789abcdef0_i64)));

success_case!(!fixnum_hex_exact, "#x#e123456789abcdef0" => SNumber, SNumber::Fixnum(Fixnum::from(0x123456789abcdef0_i64)));

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

success_case!(!flonum_zero, "0.0" => SNumber, SNumber::Flonum(Flonum::from(0.0)));

success_case!(!flonum_dot_zero, ".0" => SNumber, SNumber::Flonum(Flonum::from(0.0)));

success_case!(!flonum_zero_dot, "0." => SNumber, SNumber::Flonum(Flonum::from(0.0)));

success_case!(!flonum, "123.45" => SNumber, SNumber::Flonum(Flonum::from(123.45)));

success_case!(!flonum_inexact, "#i123.45" => SNumber, SNumber::Flonum(Flonum::from(123.45)));

success_case!(!positive_infinity, "+inf.0" => SNumber, SNumber::Flonum(Flonum::from(f64::INFINITY)));

success_case!(!negative_infinity, "-inf.0" => SNumber, SNumber::Flonum(Flonum::from(f64::NEG_INFINITY)));

success_case!(positive_nan, "+nan.0" => SNumber, "+nan.0");

success_case!(negative_nan, "-nan.0" => SNumber, "-nan.0");

// ------------------------------------------------------------------------------------------------

success_case!(flonum_exp, "123e20" => SNumber, "1.23e22");

success_case!(flonum_positive_exp, "123e+20" => SNumber, "1.23e22");

success_case!(flonum_negative_exp, "123e-20" => SNumber, "1.23e-18");

success_case!(flonum_zero_exp, "0.0e20" => SNumber, "0.0");

success_case!(flonum_dot_one_exp, ".1e20" => SNumber, "1e19");

success_case!(flonum_one_dot_exp, "1.e20" => SNumber, "1e20");

success_case!(flonum_inexact_exp, "#i123.45e20" => SNumber, "1.2345e22");

// ------------------------------------------------------------------------------------------------
// Failure cases
// ------------------------------------------------------------------------------------------------

failure_case!(invalid_binary_digit, "#b02", SNumber);

failure_case!(invalid_octal_digit, "#o09", SNumber);

failure_case!(invalid_decimal_digit, "#d0a", SNumber);

failure_case!(invalid_hex_digit, "#x0g", SNumber);

failure_case!(fixnum_exact_inexact, "#e#i0.0", SNumber);

failure_case!(fixnum_inexact_exact, "#i#e0.0", SNumber);
