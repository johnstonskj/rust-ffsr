use ffsr::reader::datum::SIdentifier;

// ------------------------------------------------------------------------------------------------
// Single-valued success cases
// ------------------------------------------------------------------------------------------------

success_case!(sinple_a, "a" => SIdentifier);

success_case!(single_plus, "+" => SIdentifier);

success_case!(single_divide, "÷" => SIdentifier);

success_case!(vbar_a, "|a|" => SIdentifier, "a");

success_case!(with_spaces, " a " => SIdentifier, "| a |");

success_case!(space, " " => SIdentifier, "| |");

success_case!(with_reserved_chars, "a[0].ba#" => SIdentifier, "|a[0].ba#|");

success_case!(
    with_mnemonic_escape,
    "hello \\\"scheme\\\" from rust" => SIdentifier,
    "|hello \"scheme\" from rust|"
);

success_case!(
    with_hex_escape,
    "\\x03B1; is named GREEK SMALL LETTER ALPHA." => SIdentifier,
    "|α is named GREEK SMALL LETTER ALPHA.|"
);

// ------------------------------------------------------------------------------------------------
// Failure cases
// ------------------------------------------------------------------------------------------------

failure_case!(empty, "", SIdentifier);

failure_case!(vbar_empty, "||", SIdentifier);

failure_case!(incomplete_ascii_escape, "str\\", SIdentifier);

failure_case!(incomplete_hex_escape, "str\\x20", SIdentifier);

failure_case!(number, "12", SIdentifier);

failure_case!(plus_number, "+12", SIdentifier);
