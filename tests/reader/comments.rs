use ffsr::reader::datum::SBoolean;

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

success_case!(
    line_comment_before,
    "; you can ignore this #f\n#t" =>
    Boolean,
    SBoolean::from(true)
);

success_case!(
    line_comment_after,
    "#t ; you can ignore this #f" =>
    Boolean,
    SBoolean::from(true)
);

success_case!(
    block_comment_before,
    "#| you can ignore this #f|##t" =>
    Boolean,
    SBoolean::from(true)
);

success_case!(
    block_comment_after,
    "#t #| you can ignore this #f|#" =>
    Boolean,
    SBoolean::from(true)
);

// ------------------------------------------------------------------------------------------------
// Failure cases
// ------------------------------------------------------------------------------------------------

failure_case!(incomplete, "#;");
