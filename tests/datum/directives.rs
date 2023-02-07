use ffsr::reader::datum::SDirective;

// ------------------------------------------------------------------------------------------------
// Single-valued success cases
// ------------------------------------------------------------------------------------------------

success_case!(! fold_case, "#!fold-case" => SDirective, SDirective::FoldCase(true));

success_case!(! no_fold_case, "#!no-fold-case" => SDirective, SDirective::FoldCase(false));

// ------------------------------------------------------------------------------------------------
// Multi-valued success cases
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Failure cases
// ------------------------------------------------------------------------------------------------

// FIX: IGNORE FOR NOW failure_case!(empty, "", SDirective);

failure_case!(no_name, "#!", SDirective);

failure_case!(unknown_name, "#!unknown", SDirective);
