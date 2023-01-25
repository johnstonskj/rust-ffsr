use paste::paste;

// ------------------------------------------------------------------------------------------------
// Single-valued success cases
// ------------------------------------------------------------------------------------------------

success_case!(single_one, "1" => numeric);
success_case!(integer, "123" => numeric);
success_case!(exact_integer, "#e123" => numeric);
success_case!(inexact_integer, "#i123" => numeric);
success_case!(binary_integer, "#b1111011" => numeric);
success_case!(binary_exact_integer, "#b#e1111011" => numeric);
success_case!(binary_inexact_integer, "#b#i1111011" => numeric);
success_case!(octal_integer, "#o173" => numeric);
success_case!(octal_exact_integer, "#o#e173" => numeric);
success_case!(octal_inexact_integer, "#o#i173" => numeric);
success_case!(hex_integer, "#x7b" => numeric);
success_case!(hex_exact_integer, "#x#e7b" => numeric);
success_case!(hex_inexact_integer, "#x#i7b" => numeric);

// ------------------------------------------------------------------------------------------------
// Multi-valued success cases
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Failure cases
// ------------------------------------------------------------------------------------------------
