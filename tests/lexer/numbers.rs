// ------------------------------------------------------------------------------------------------
// Single-valued success cases
// ------------------------------------------------------------------------------------------------

success_case!(single_one, "1" => number);
success_case!(integer, "123" => number);
success_case!(exact_integer, "#e123" => number);
success_case!(inexact_integer, "#i123" => number);
success_case!(binary_integer, "#b1111011" => number);
success_case!(binary_exact_integer, "#b#e1111011" => number);
success_case!(binary_inexact_integer, "#b#i1111011" => number);
success_case!(octal_integer, "#o173" => number);
success_case!(octal_exact_integer, "#o#e173" => number);
success_case!(octal_inexact_integer, "#o#i173" => number);
success_case!(hex_integer, "#x7b" => number);
success_case!(hex_exact_integer, "#x#e7b" => number);
success_case!(hex_inexact_integer, "#x#i7b" => number);

// ------------------------------------------------------------------------------------------------
// Multi-valued success cases
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Failure cases
// ------------------------------------------------------------------------------------------------
