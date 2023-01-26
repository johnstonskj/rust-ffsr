use ffsr::input::{indices::CharIndex, Input};

#[test]
fn push_back() {
    let _guard = crate::init_tracing();

    let input = Input::from("abcdef");
    let mut char_indices = input.char_indices();

    assert_next_char!(char_indices, 0, 0, 'a');
    assert_next_char!(char_indices, 1, 1, 'b');
    char_indices.push_back(CharIndex::new(1, 1, 'b'));
    assert_next_char!(char_indices, 1, 1, 'b');
    assert_next_char!(char_indices, 2, 2, 'c');
    char_indices.push_back(CharIndex::new(2, 2, 'c'));
    char_indices.push_back(CharIndex::new(1, 1, 'b'));
    assert_next_char!(char_indices, 1, 1, 'b');
    assert_next_char!(char_indices, 2, 2, 'c');
    assert_next_char!(char_indices, 3, 3, 'd');
    assert_next_char!(char_indices, 4, 4, 'e');
    assert_next_char!(char_indices, 5, 5, 'f');
    assert_complete!(char_indices);
}

#[test]
fn peekable_push_back() {
    let _guard = crate::init_tracing();

    let input = Input::from("abcdef");
    let mut char_indices = input.char_indices();

    assert_next_char!(?char_indices, 0, 0, 'a');
    assert_next_char!(char_indices, 0, 0, 'a');
    assert_next_char!(char_indices, 1, 1, 'b');
    assert_next_char!(?char_indices, 2, 2, 'c');
    assert_next_char!(char_indices, 2, 2, 'c');
    assert_next_char!(char_indices, 3, 3, 'd');
    char_indices.push_back(CharIndex::new(3, 3, 'd'));
    assert_next_char!(?char_indices, 3, 3, 'd');
    assert_next_char!(char_indices, 3, 3, 'd');
    assert_next_char!(char_indices, 4, 4, 'e');
    assert_next_char!(char_indices, 5, 5, 'f');
    assert_complete!(?char_indices);
    assert_complete!(char_indices);
}
