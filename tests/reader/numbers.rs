use ffsr::reader::datum::{Fixnum, SNumber};

// ------------------------------------------------------------------------------------------------
// Single-valued success cases
// ------------------------------------------------------------------------------------------------

success_case!(single_one, "1" => Number, SNumber::Fixnum(Fixnum::from(1)));

success_case!(integer, "12345" => Number, SNumber::Fixnum(Fixnum::from(12345)));

success_case!(binary_integer, "#e#b1011" => Number, SNumber::Fixnum(Fixnum::from(11)));

// ------------------------------------------------------------------------------------------------
// Failure cases
// ------------------------------------------------------------------------------------------------
