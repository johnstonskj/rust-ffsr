use ffsr::reader::datum::{Fixnum, Flonum, SNumber};

// ------------------------------------------------------------------------------------------------
// Single-valued success cases
// ------------------------------------------------------------------------------------------------

success_case!(!fixnum_zero, "0" => Number, SNumber::Fixnum(Fixnum::from(0)));

success_case!(!fixnum, "9762457" => Number, SNumber::Fixnum(Fixnum::from(9762457)));

success_case!(!fixnum_exact, "#e9762457" => Number, SNumber::Fixnum(Fixnum::from(9762457)));

success_case!(!fixnum_pos, "+9762457" => Number, SNumber::Fixnum(Fixnum::from(9762457)));

success_case!(!fixnum_exact_pos, "#e+9762457" => Number, SNumber::Fixnum(Fixnum::from(9762457)));

success_case!(!fixnum_exact_neg, "#e-9762457" => Number, SNumber::Fixnum(Fixnum::from(-9762457)));

// ------------------------------------------------------------------------------------------------

success_case!(!fixnum_binary, "#b1010" => Number, SNumber::Fixnum(Fixnum::from(0b1010)));

success_case!(!fixnum_exact_binary, "#e#b1010" => Number, SNumber::Fixnum(Fixnum::from(0b1010)));

success_case!(!fixnum_binary_exact, "#b#e1010" => Number, SNumber::Fixnum(Fixnum::from(0b1010)));

// ------------------------------------------------------------------------------------------------

success_case!(!fixnum_octal, "#o12345670" => Number, SNumber::Fixnum(Fixnum::from(0o12345670)));

success_case!(!fixnum_exact_octal, "#e#o12345670" => Number, SNumber::Fixnum(Fixnum::from(0o12345670)));

success_case!(!fixnum_octal_exact, "#o#e12345670" => Number, SNumber::Fixnum(Fixnum::from(0o12345670)));

// ------------------------------------------------------------------------------------------------

success_case!(!fixnum_decimal, "#d1234567890" => Number, SNumber::Fixnum(Fixnum::from(1234567890)));

success_case!(!fixnum_exact_decimal, "#e#d1234567890" => Number, SNumber::Fixnum(Fixnum::from(1234567890)));

success_case!(!fixnum_decimal_exact, "#d#e1234567890" => Number, SNumber::Fixnum(Fixnum::from(1234567890)));

// ------------------------------------------------------------------------------------------------

success_case!(!fixnum_hex, "#x123456789abcdef0" => Number, SNumber::Fixnum(Fixnum::from(0x123456789abcdef0_i64)));

success_case!(!fixnum_exact_hex, "#e#x123456789abcdef0" => Number, SNumber::Fixnum(Fixnum::from(0x123456789abcdef0_i64)));

success_case!(!fixnum_hex_exact, "#x#e123456789abcdef0" => Number, SNumber::Fixnum(Fixnum::from(0x123456789abcdef0_i64)));

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

success_case!(!flonum_zero, "0.0" => Number, SNumber::Flonum(Flonum::from(0.0)));

success_case!(!flonum_dot_zero, ".0" => Number, SNumber::Flonum(Flonum::from(0.0)));

success_case!(!flonum_zero_dot, "0." => Number, SNumber::Flonum(Flonum::from(0.0)));

success_case!(!flonum, "123.45" => Number, SNumber::Flonum(Flonum::from(123.45)));

success_case!(!flonum_inexact, "#i123.45" => Number, SNumber::Flonum(Flonum::from(123.45)));

success_case!(!positive_infinity, "+inf.0" => Number, SNumber::Flonum(Flonum::from(f64::INFINITY)));

success_case!(!negative_infinity, "-inf.0" => Number, SNumber::Flonum(Flonum::from(f64::NEG_INFINITY)));

//success_case!(positive_nan, "+nan.0" => Number, "+nan.0");

//success_case!(negative_nan, "-nan.0" => Number, "-nan.0");

// ------------------------------------------------------------------------------------------------

//success_case!(flonum_exp, "123e20" => Number, "1.23e22");

//success_case!(flonum_positive_exp, "123e+20" => Number, "1.23e22");

//success_case!(flonum_negative_exp, "123e-20" => Number, "1.23e-18");

//success_case!(flonum_zero_exp, "0.0e20" => Number, "0.0");

//success_case!(flonum_dot_one_exp, ".1e20" => Number, "1e19");

//success_case!(flonum_one_dot_exp, "1.e20" => Number, "1e20");

//success_case!(flonum_inexact_exp, "#i123.45e20" => Number, "1.2345e22");

// ------------------------------------------------------------------------------------------------
// Failure cases
// ------------------------------------------------------------------------------------------------
