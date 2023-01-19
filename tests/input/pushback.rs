use ffsr::input::{indices::CharIndex, Input};
use pretty_assertions::assert_eq;

#[test]
fn test_push_back() {
    let input = Input::from("abcdef");
    let mut char_indices = input.char_indices();
    assert_eq!(char_indices.next(), Some(CharIndex::new(0, 0, 'a')));
    assert_eq!(char_indices.next(), Some(CharIndex::new(1, 1, 'b')));
    char_indices.push_back(CharIndex::new(1, 1, 'b'));
    assert_eq!(char_indices.next(), Some(CharIndex::new(1, 1, 'b')));
    assert_eq!(char_indices.next(), Some(CharIndex::new(2, 2, 'c')));
    char_indices.push_back(CharIndex::new(2, 2, 'c'));
    char_indices.push_back(CharIndex::new(1, 1, 'b'));
    assert_eq!(char_indices.next(), Some(CharIndex::new(1, 1, 'b')));
    assert_eq!(char_indices.next(), Some(CharIndex::new(2, 2, 'c')));
    assert_eq!(char_indices.next(), Some(CharIndex::new(3, 3, 'd')));
    assert_eq!(char_indices.next(), Some(CharIndex::new(4, 4, 'e')));
    assert_eq!(char_indices.next(), Some(CharIndex::new(5, 5, 'f')));
    assert_eq!(char_indices.next(), None);
}

#[test]
fn test_peekable_push_back() {
    let input = Input::from("abcdef");
    let mut char_indices = input.char_indices();
    assert_eq!(char_indices.peek(), Some(&CharIndex::new(0, 0, 'a')));
    assert_eq!(char_indices.next(), Some(CharIndex::new(0, 0, 'a')));
    assert_eq!(char_indices.next(), Some(CharIndex::new(1, 1, 'b')));
    assert_eq!(char_indices.peek(), Some(&CharIndex::new(2, 2, 'c')));
    assert_eq!(char_indices.next(), Some(CharIndex::new(2, 2, 'c')));
    assert_eq!(char_indices.next(), Some(CharIndex::new(3, 3, 'd')));
    char_indices.push_back(CharIndex::new(3, 3, 'd'));
    assert_eq!(char_indices.peek(), Some(&CharIndex::new(3, 3, 'd')));
    assert_eq!(char_indices.next(), Some(CharIndex::new(3, 3, 'd')));
    assert_eq!(char_indices.next(), Some(CharIndex::new(4, 4, 'e')));
    assert_eq!(char_indices.next(), Some(CharIndex::new(5, 5, 'f')));
    assert_eq!(char_indices.peek(), None);
    assert_eq!(char_indices.next(), None);
}
