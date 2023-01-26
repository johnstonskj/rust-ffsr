use ffsr::reader::datum::{Datum, SChar, SIdentifier, SString, SVector};
use paste::paste;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Single-valued success cases
// ------------------------------------------------------------------------------------------------

success_case!(empty, "#()" => Vector, SVector::default());

success_case!(
    identifiers,
    "#(a b c)" =>
    Vector,
    SVector::from(vec![
        Datum::Identifier(SIdentifier::from_str("a").unwrap()),
        Datum::Identifier(SIdentifier::from_str("b").unwrap()),
        Datum::Identifier(SIdentifier::from_str("c").unwrap()),
    ])
);

success_case!(
    heterogeneous,
    "#(a #\\b \"c\")" =>
    Vector,
    SVector::from(vec![
        Datum::Identifier(SIdentifier::from_str("a").unwrap()),
        Datum::Char(SChar::from('b')),
        Datum::String(SString::from_str("c").unwrap()),
    ])
);

success_case!(
    heterogeneous_nested,
    "#(a #() \"c\")" =>
    Vector,
    SVector::from(vec![
        Datum::Identifier(SIdentifier::from_str("a").unwrap()),
        Datum::Vector(SVector::default()),
        Datum::String(SString::from_str("c").unwrap()),
    ])
);

// ------------------------------------------------------------------------------------------------
// Failure cases
// ------------------------------------------------------------------------------------------------
