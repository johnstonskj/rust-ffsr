use ffsr::reader::datum::{Datum, SChar};

// ------------------------------------------------------------------------------------------------
// Single-valued success cases
// ------------------------------------------------------------------------------------------------

success_case!(quote, "'#\\a" => Quote, Box::from(Datum::from(SChar::from('a'))));

success_case!(quasi_quote, "`#\\a" => QuasiQuote, Box::from(Datum::from(SChar::from('a'))));

success_case!(unquote, ",#\\a" => Unquote, Box::from(Datum::from(SChar::from('a'))));

success_case!(unquote_splicing, ",@#\\a" => UnquoteSplicing, Box::from(Datum::from(SChar::from('a'))));

// ------------------------------------------------------------------------------------------------
// Multi-valued success cases
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Failure cases
// ------------------------------------------------------------------------------------------------

failure_case!(incomplete_quote, "'");

failure_case!(incomplete_quasi_quote, "`");

failure_case!(incomplete_unquote, ",");

failure_case!(incomplete_unquote_splicing, ",@");
