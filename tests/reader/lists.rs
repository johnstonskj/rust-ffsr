use ffsr::reader::datum::{Datum, SChar, SIdentifier, SList, SPair, SString};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Single-valued success cases
// ------------------------------------------------------------------------------------------------

success_case!(empty, "()" => List, SList::empty());

success_case!(
    identifiers,
    "(a b c)" =>
    List,
    SList::from(vec![
        Datum::Identifier(SIdentifier::from_str("a").unwrap()),
        Datum::Identifier(SIdentifier::from_str("b").unwrap()),
        Datum::Identifier(SIdentifier::from_str("c").unwrap()),
    ])
);

success_case!(
    identifiers_nested,
    "(a . (b . (c)))" =>
    List,
    SList::from(vec![
        Datum::Identifier(SIdentifier::from_str("a").unwrap()),
        Datum::Identifier(SIdentifier::from_str("b").unwrap()),
        Datum::Identifier(SIdentifier::from_str("c").unwrap()),
    ])
);

success_case!(
    heterogeneous,
    "(a #\\b \"c\")" =>
    List,
    SList::from(vec![
        Datum::Identifier(SIdentifier::from_str("a").unwrap()),
        Datum::Char(SChar::from('b')),
        Datum::String(SString::from_str("c").unwrap()),
    ])
);

success_case!(
    heterogeneous_nested,
    "(a () \"c\")" =>
    List,
    SList::from(vec![
        Datum::Identifier(SIdentifier::from_str("a").unwrap()),
        Datum::List(SList::empty()),
        Datum::String(SString::from_str("c").unwrap()),
    ])
);

success_case!(
    improper_pair,
    "(a . x)" =>
    List,
    SList::from(SPair::cons(
        Datum::Identifier(SIdentifier::from_str("a").unwrap()).into(),
        Datum::Identifier(SIdentifier::from_str("x").unwrap()).into(),
    ))
);

success_case!(
    improper_list,
    "(a b c . x)" =>
    List,
    improper_list_value()
);

fn improper_list_value() -> SList {
    let mut list = SList::from(vec![
        Datum::Identifier(SIdentifier::from_str("a").unwrap()),
        Datum::Identifier(SIdentifier::from_str("b").unwrap()),
        Datum::Identifier(SIdentifier::from_str("c").unwrap()),
    ]);
    list.append_improper(
        Datum::Identifier(SIdentifier::from_str("x").unwrap()).into(),
        None,
    )
    .expect("Could not create improper list");
    list
}

// ------------------------------------------------------------------------------------------------
// Failure cases
// ------------------------------------------------------------------------------------------------

failure_case!(
    missing_car,
    "( . a)",
    "Dotted pair missing a car value; span: 2..3"
);

failure_case!(
    missing_cdr,
    "(a . )",
    "Dotted pair missing a cdr value; span: 3..4"
);

failure_case!(
    too_many_cdrs,
    "(a . b c)",
    "Dotted pair already has a cdr value; span: 3..4"
);

failure_case!(
    too_many_dots,
    "(a . . b)",
    "Dotted pair already has a cdr value; span: 3..4"
);
