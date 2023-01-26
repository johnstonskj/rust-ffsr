use ffsr::input::{indices::CharIndex, Input};

#[test]
fn empty() {
    let _guard = crate::init_tracing();

    let input = Input::from("");
    let mut char_indices = input.char_indices();

    assert_complete!(char_indices);
}

#[test]
fn identifier() {
    let _guard = crate::init_tracing();

    let input = Input::from("hello");
    let mut char_indices = input.char_indices();

    assert_next_char!(char_indices, 0, 0, 'h');
    assert_next_char!(char_indices, 1, 1, 'e');
    assert_next_char!(char_indices, 2, 2, 'l');
    assert_next_char!(char_indices, 3, 3, 'l');
    assert_next_char!(char_indices, 4, 4, 'o');
    assert_complete!(char_indices);
}
