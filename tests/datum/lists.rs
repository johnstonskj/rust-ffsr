use ffsr::reader::datum::{Datum, SChar, SList, SPair};

// ------------------------------------------------------------------------------------------------
// Single-valued success cases
// ------------------------------------------------------------------------------------------------

#[test]
fn empty() {
    let list = SList::empty();
    assert!(list.is_empty());
    assert!(list.is_list());
    assert!(!list.is_improper_list());
    assert_eq!(list.to_string(), "()".to_string());
}

#[test]
fn simple_pair() {
    let pair = SPair::cons(
        Datum::from(SChar::from('a')).into(),
        Datum::from(SChar::from('b')).into(),
    );
    assert!(!pair.is_proper());
    assert!(pair.is_improper());
    assert_eq!(pair.to_string(), "(#\\a . #\\b)".to_string());
}

// ------------------------------------------------------------------------------------------------
// Multi-valued success cases
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Failure cases
// ------------------------------------------------------------------------------------------------
