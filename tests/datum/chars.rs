use ffsr::reader::datum::SChar;

// ------------------------------------------------------------------------------------------------
// Single-valued success cases
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Multi-valued success cases
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Failure cases
// ------------------------------------------------------------------------------------------------

// FIX: IGNORE FOR NOW failure_case!(empty, "", SChar);

failure_case!(no_name, "#\\", SChar);

failure_case!(unknown_name, "#\\unknown", SChar);
