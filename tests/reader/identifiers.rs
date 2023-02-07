use ffsr::reader::datum::SIdentifier;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Single-valued success cases
// ------------------------------------------------------------------------------------------------

success_case!(vbar_empty, "||" => Identifier, SIdentifier::from_str("||").unwrap());

success_case!(single_a, "a" => Identifier, SIdentifier::from_str("a").unwrap());

success_case!(plus, "+" => Identifier, SIdentifier::from_str("+").unwrap());

success_case!(three_dots, "..." => Identifier, SIdentifier::from_str("...").unwrap());

success_case!(plus_soup_plus, "+soup+" => Identifier, SIdentifier::from_str("+soup+").unwrap());

success_case!(right_arrow_string, "->string" => Identifier, SIdentifier::from_str("->string").unwrap());

success_case!(less_than_or_equal_question, "<=?" => Identifier, SIdentifier::from_str("<=?").unwrap());

success_case!(spec_example_random, "a34kTMNs" => Identifier, SIdentifier::from_str("a34kTMNs").unwrap());

success_case!(long_example, "the-word-recursion-has-many-meanings" => Identifier, SIdentifier::from_str("the-word-recursion-has-many-meanings").unwrap());

success_case!(greek_latter_lambda, "λ" => Identifier, SIdentifier::from_str("λ").unwrap());

success_case!(vbar_a, "|a|" => Identifier, SIdentifier::from_str("a").unwrap());

success_case!(vbar_spaces, "|a b\tc|" => Identifier, SIdentifier::from_str("|a b\tc|").unwrap());

success_case!(single_emoji, "☺️️" => Identifier, SIdentifier::from_str( "☺️️").unwrap());

success_case!(fancy, "〜foo〜" => Identifier, SIdentifier::from_str("〜foo〜").unwrap());

success_case!(at_here, "@here" => Identifier, SIdentifier::from_str("@here").unwrap());

success_case!(one_world, "1world" => Identifier, SIdentifier::from_str("1world").unwrap());

success_case!(line_break, "|hel\nlo|" => Identifier, SIdentifier::from_str("|hel\nlo|").unwrap());

success_case!(unicode_complex, "ཨོཾ་མ་ཎི་པདྨེ་ཧཱུྃ" => Identifier, SIdentifier::from_str("ཨོཾ་མ་ཎི་པདྨེ་ཧཱུྃ").unwrap());
// ------------------------------------------------------------------------------------------------
// Failure cases
// ------------------------------------------------------------------------------------------------
