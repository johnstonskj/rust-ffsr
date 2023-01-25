use paste::paste;

// ------------------------------------------------------------------------------------------------
// Single-valued success cases
// ------------------------------------------------------------------------------------------------

success_case!(true_literal, "#t" => boolean);
success_case!(false_literal, "#f" => boolean);

// ------------------------------------------------------------------------------------------------
// Multi-valued success cases
// ------------------------------------------------------------------------------------------------

success_case!(three_booleans, "#f #t#f" => (boolean, "#f"), (boolean, "#t"), (boolean, "#f"));

// ------------------------------------------------------------------------------------------------
// Failure cases
// ------------------------------------------------------------------------------------------------
