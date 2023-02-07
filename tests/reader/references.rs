use ffsr::reader::datum::{Datum, Fixnum, SBoolean, SIdentifier, SList};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Single-valued success cases
// ------------------------------------------------------------------------------------------------

success_case!(!
    top_level_assignment,
    "#1=#t" =>
    Boolean,
    SBoolean::from(true)
);

success_case!(!
    assignment_within_list,
    "(#1=a b c #1#)" =>
    List,
    SList::from(vec![
        Datum::Identifier(SIdentifier::from_str("a").unwrap()),
        Datum::Identifier(SIdentifier::from_str("b").unwrap()),
        Datum::Identifier(SIdentifier::from_str("c").unwrap()),
        Datum::Identifier(SIdentifier::from_str("a").unwrap()),
    ])
);

success_case!(!
    assign_to_reference,
    "(#1=99 77 88 #2=#1# 88 77 #2#)" =>
    List,
    SList::from(vec![
        Datum::Number(Fixnum::from(99).into()),
        Datum::Number(Fixnum::from(77).into()),
        Datum::Number(Fixnum::from(88).into()),
        Datum::Number(Fixnum::from(99).into()),
        Datum::Number(Fixnum::from(88).into()),
        Datum::Number(Fixnum::from(77).into()),
        Datum::Number(Fixnum::from(99).into()),
    ])
);

success_case!(!
    ignore_assignment,
    "(1 2 #;#1=99 3 4)" =>
    List,
    SList::from(vec![
        Datum::Number(Fixnum::from(1).into()),
        Datum::Number(Fixnum::from(2).into()),
        Datum::Number(Fixnum::from(3).into()),
        Datum::Number(Fixnum::from(4).into()),
    ])
);

// ------------------------------------------------------------------------------------------------
// Failure cases
// ------------------------------------------------------------------------------------------------
