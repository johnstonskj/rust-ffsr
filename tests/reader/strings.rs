use ffsr::reader::datum::SString;
use paste::paste;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Single-valued success cases
// ------------------------------------------------------------------------------------------------

success_case!(empty, "\"\"" => String, SString::default());

success_case!(hello, "\"hello\"" => String, SString::from_str("hello").unwrap());

success_case!(hello_with_mnemonic_escape, "\"hel\\\"lo\"" => String, SString::from_str("hel\"lo").unwrap());

success_case!(hello_with_hex_escape, "\"hel\\x00fd;lo\"" => String, SString::from_str("helýlo").unwrap());

// ------------------------------------------------------------------------------------------------
// Failure cases
// ------------------------------------------------------------------------------------------------

failure_case!(incomplete_string, "\"hello #t");

failure_case!(bad_mnemonic_escape, "\"hel\\zlo\"");

failure_case!(bad_hex_escape, "\"hel\\x00fdlo\"");
