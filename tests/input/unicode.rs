use ffsr::input::{indices::CharIndex, Input};

#[test]
fn phrase() {
    let _guard = crate::init_tracing();

    let input = Input::from("ཨོཾ་མ་ཎི་པདྨེ་ཧཱུྃ");
    let mut char_indices = input.char_indices();

    assert_next_char!(char_indices, 0, 0, '\u{0f68}');
    assert_next_char!(char_indices, 3, 1, '\u{0f7c}');
    assert_next_char!(char_indices, 6, 2, '\u{0f7e}');
    assert_next_char!(char_indices, 9, 3, '\u{0f0b}');
    assert_next_char!(char_indices, 12, 4, '\u{0f58}');
    assert_next_char!(char_indices, 15, 5, '\u{0f0b}');
    assert_next_char!(char_indices, 18, 6, '\u{0f4e}');
    assert_next_char!(char_indices, 21, 7, '\u{0f72}');
    assert_next_char!(char_indices, 24, 8, '\u{0f0b}');
    assert_next_char!(char_indices, 27, 9, '\u{0f54}');
    assert_next_char!(char_indices, 30, 10, '\u{0f51}');
    assert_next_char!(char_indices, 33, 11, '\u{0fa8}');
    assert_next_char!(char_indices, 36, 12, '\u{0f7a}');
    assert_next_char!(char_indices, 39, 13, '\u{0f0b}');
    assert_next_char!(char_indices, 42, 14, '\u{0f67}');
    assert_next_char!(char_indices, 45, 15, '\u{0f71}');
    assert_next_char!(char_indices, 48, 16, '\u{0f74}');
    assert_next_char!(char_indices, 51, 17, '\u{0f83}');
    assert_complete!(char_indices);
}
