use ffsr::input::{indices::CharIndex, Input};
use pretty_assertions::assert_eq;

#[test]
fn empty() {
    let input = Input::from("");
    assert!(input.char_indices().next().is_none());
}

#[test]
fn identifier() {
    let input = Input::from("hello");
    let mut char_indices = input.char_indices();
    assert_eq!(char_indices.next(), Some(CharIndex::new(0, 0, 'h')));
    assert_eq!(char_indices.next(), Some(CharIndex::new(1, 1, 'e')));
    assert_eq!(char_indices.next(), Some(CharIndex::new(2, 2, 'l')));
    assert_eq!(char_indices.next(), Some(CharIndex::new(3, 3, 'l')));
    assert_eq!(char_indices.next(), Some(CharIndex::new(4, 4, 'o')));
    assert!(char_indices.next().is_none());
}
