use ffsr::Sourced;
use paste::paste;

// ------------------------------------------------------------------------------------------------
// Single-valued success cases
// ------------------------------------------------------------------------------------------------

success_case!(empty, "\"\"" => string);
success_case!(hello, "\"hello\"" => string);
success_case!(hel_mnemonic_escape_lo, "\"hel\\\"lo\"" => string);
success_case!(hel_hex_escape_lo, "\"hel\\x00fd;lo\"" => string);

// ------------------------------------------------------------------------------------------------
// Multi-valued success cases
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Failure cases
// ------------------------------------------------------------------------------------------------

failure_case!(
    incomplete_string,
    "\" #t #f",
    "Token 'string' not closed, span: 0..6"
);
failure_case!(
    bad_mnemonic_escape,
    "\"hel\\zlo\"",
    "Invalid, or badly formed, character escape string; span: 0..5"
);
failure_case!(
    bad_hex_escape,
    "\"hel\\x00fdlo\"",
    "Invalid, or badly formed, character escape string; span: 0..10"
);
