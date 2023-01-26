use ffsr::reader::datum::{Datum, SChar, SIdentifier, SList, SString};
use paste::paste;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Single-valued success cases
// ------------------------------------------------------------------------------------------------

success_case!(empty, "()" => List, SList::default());

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
        Datum::List(SList::default()),
        Datum::String(SString::from_str("c").unwrap()),
    ])
);

// ------------------------------------------------------------------------------------------------
// Failure cases
// ------------------------------------------------------------------------------------------------
