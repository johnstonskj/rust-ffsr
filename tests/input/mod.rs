use ffsr::input::Input;

#[test]
fn test_empty_string() {
    let input = Input::from("");
    assert!(input.char_indices().next().is_none());
}

pub mod pushback;
pub mod simple;
pub mod unicode;
