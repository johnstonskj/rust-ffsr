// ------------------------------------------------------------------------------------------------
// Single-valued success cases
// ------------------------------------------------------------------------------------------------

success_case!(single_one, "1" => number);
success_case!(integer, "123" => number);
success_case!(positive_integer, "+123" => number);
success_case!(negative_integer, "-123" => number);
success_case!(exact_integer, "#e123" => number);

success_case!(positive_exact_integer, "#e+123" => number);
success_case!(negative_exact_integer, "#e-123" => number);

success_case!(binary_integer, "#b1111011" => number);
success_case!(binary_exact_integer, "#b#e1111011" => number);

success_case!(octal_integer, "#o173" => number);
success_case!(octal_exact_integer, "#o#e173" => number);

success_case!(hex_integer, "#x7b" => number);
success_case!(hex_exact_integer, "#x#e7b" => number);

success_case!(inexact_integer, "#i123" => number);
success_case!(binary_inexact_integer, "#b#i1111011" => number);
success_case!(octal_inexact_integer, "#o#i173" => number);
success_case!(hex_inexact_integer, "#x#i7b" => number);

// ------------------------------------------------------------------------------------------------

success_case!(rational, "1/2" => number);
success_case!(negative_rational, "-1/2" => number);
success_case!(exact_rational, "#e1/2" => number);
success_case!(exact_negative_rational, "#e-1/2" => number);
success_case!(inexact_rational, "#i1/2" => number);
success_case!(inexact_negative_rational, "#i-1/2" => number);

success_case!(binary_exact_rational, "#b#e1/10" => number);
success_case!(binary_exact_negative_rational, "#b#e-1/10" => number);
success_case!(octal_exact_rational, "#o#e1/6" => number);
success_case!(octal_exact_negative_rational, "#o#e-1/6" => number);
success_case!(hex_exact_rational, "#x#e1/c" => number);
success_case!(hex_exact_negative_rational, "#x#e-1/c" => number);

// ------------------------------------------------------------------------------------------------

success_case!(inexact_flonum, "#i1.23" => number);
success_case!(positive_inexact_flonum, "#i+1.23" => number);
success_case!(negative_inexact_flonum, "#i-1.23" => number);
success_case!(inexact_flonum_exp, "#i1.23e20" => number);
success_case!(binary_inexact_flonum_exp, "#b#i100.01e10" => number);
success_case!(hex_flonum_exp, "#x#if0.01la" => number);
success_case!(binary_inexact_flonum_alt_exp, "#b#i100.01^10" => number);
success_case!(hex_flonum_alt_exp, "#x#if0.01^a" => number);

// ------------------------------------------------------------------------------------------------

success_case!(complex_fixnum, "3+4i" => number);
success_case!(complex_ratnum, "1/2+3/4i" => number);
success_case!(complex_flonum, "3.0+4.0i" => number);

// ------------------------------------------------------------------------------------------------

success_case!(polar_fixnum, "3@4i" => number);
success_case!(polar_ratnum, "1/2@+3/4i" => number);
success_case!(polar_flonum, "3.0@4.0i" => number);

// ------------------------------------------------------------------------------------------------

success_case!(!positive_infinity, "+inf.0" => number);
success_case!(!negative_infinity, "-inf.0" => number);

success_case!(positive_nan, "+nan.0" => number);
success_case!(negative_nan, "-nan.0" => number);

// ------------------------------------------------------------------------------------------------
// Multi-valued success cases
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Failure cases
// ------------------------------------------------------------------------------------------------
