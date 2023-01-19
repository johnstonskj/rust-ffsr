use ffsr::input::{indices::CharIndex, Input};
use pretty_assertions::assert_eq;

#[test]
fn test_unicode_string() {
    let input = Input::from("ཨོཾ་མ་ཎི་པདྨེ་ཧཱུྃ");
    let mut char_indices = input.char_indices();
    assert_eq!(char_indices.next(), Some(CharIndex::new(0, 0, '\u{0f68}')));
    assert_eq!(char_indices.next(), Some(CharIndex::new(3, 1, '\u{0f7c}')));
    assert_eq!(char_indices.next(), Some(CharIndex::new(6, 2, '\u{0f7e}')));
    assert_eq!(char_indices.next(), Some(CharIndex::new(9, 3, '\u{0f0b}')));
    assert_eq!(char_indices.next(), Some(CharIndex::new(12, 4, '\u{0f58}')));
    assert_eq!(char_indices.next(), Some(CharIndex::new(15, 5, '\u{0f0b}')));
    assert_eq!(char_indices.next(), Some(CharIndex::new(18, 6, '\u{0f4e}')));
    assert_eq!(char_indices.next(), Some(CharIndex::new(21, 7, '\u{0f72}')));
    assert_eq!(char_indices.next(), Some(CharIndex::new(24, 8, '\u{0f0b}')));
    assert_eq!(char_indices.next(), Some(CharIndex::new(27, 9, '\u{0f54}')));
    assert_eq!(
        char_indices.next(),
        Some(CharIndex::new(30, 10, '\u{0f51}'))
    );
    assert_eq!(
        char_indices.next(),
        Some(CharIndex::new(33, 11, '\u{0fa8}'))
    );
    assert_eq!(
        char_indices.next(),
        Some(CharIndex::new(36, 12, '\u{0f7a}'))
    );
    assert_eq!(
        char_indices.next(),
        Some(CharIndex::new(39, 13, '\u{0f0b}'))
    );
    assert_eq!(
        char_indices.next(),
        Some(CharIndex::new(42, 14, '\u{0f67}'))
    );
    assert_eq!(
        char_indices.next(),
        Some(CharIndex::new(45, 15, '\u{0f71}'))
    );
    assert_eq!(
        char_indices.next(),
        Some(CharIndex::new(48, 16, '\u{0f74}'))
    );
    assert_eq!(
        char_indices.next(),
        Some(CharIndex::new(51, 17, '\u{0f83}'))
    );
    assert_eq!(char_indices.next(), None);
}
