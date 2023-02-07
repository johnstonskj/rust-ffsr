// ------------------------------------------------------------------------------------------------
// Single-valued success cases
// ------------------------------------------------------------------------------------------------

success_case!(single_letter_a, "a" => identifier);
success_case!(single_sign_plus, "+" => identifier);
success_case!(single_sign_divide, "÷" => identifier);
success_case!(three_dots, "..." => identifier);
success_case!(plus_soup_plus, "+soup+" => identifier);
success_case!(into_string, "->string" => identifier);
success_case!(arrow_into_string, "→string" => identifier);
success_case!(less_than_or_equal, "<=?" => identifier);
success_case!(
    kebab_case,
    "the-word-recursion-has-many-meanings" => identifier
);
success_case!(
    shouty_kebab_case,
    "THE-WORD-RECURSION-HAS-MANY-MEANINGS" => identifier
);
success_case!(
    snake_case,
    "the_word_recursion_has_many_meanings" => identifier
);
success_case!(camel_case, "theWordRecursionHasManyMeanings" => identifier);
success_case!(pascal_case, "TheWordRecursionHasManyMeanings" => identifier);

// ------------------------------------------------------------------------------------------------
// Multi-valued success cases
// ------------------------------------------------------------------------------------------------

success_case!(three_in_a_row, "a b c" => (identifier, "a"), (identifier, "b"), (identifier, "c"));

// ------------------------------------------------------------------------------------------------
// Failure cases
// ------------------------------------------------------------------------------------------------

failure_case!(
    incomplete_identifier,
    "|hello",
    "Incomplete identifier, expecting a terminating `#\\|`; span: 0..5"
);
