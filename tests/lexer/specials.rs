// ------------------------------------------------------------------------------------------------
// Single-valued success cases
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Multi-valued success cases
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Failure cases
// ------------------------------------------------------------------------------------------------

failure_case!(
    incomplete_special,
    "# ()",
    "Incomplete special form, span: 0..1"
);

failure_case!(unkown_special, "#*");
