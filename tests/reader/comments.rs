use ffsr::reader::datum::SBoolean;
use paste::paste;

// ------------------------------------------------------------------------------------------------
// Single-valued success cases
// ------------------------------------------------------------------------------------------------

success_case!(
    ignore_boolean,
    "#; #f #t" =>
    Boolean,
    SBoolean::from(true)
);

success_case!(
    ignore_list,
    "#; (1 2 3) #f" =>
    Boolean,
    SBoolean::from(false)
);

// ------------------------------------------------------------------------------------------------
// Failure cases
// ------------------------------------------------------------------------------------------------

failure_case!(incomplete, "#;");
