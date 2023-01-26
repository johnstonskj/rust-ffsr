macro_rules! assert_next_char {
    ($iter:expr, $at_byte:expr, $at_char:expr, $is_char:expr) => {
        ::pretty_assertions::assert_eq!(
            $iter.next(),
            Some(CharIndex::new($at_byte, $at_char, $is_char))
        );
    };
    (? $iter:expr, $at_byte:expr, $at_char:expr, $is_char:expr) => {
        ::pretty_assertions::assert_eq!(
            $iter.peek(),
            Some(&CharIndex::new($at_byte, $at_char, $is_char))
        );
    };
}

macro_rules! assert_complete {
    ($iter:expr) => {
        assert!($iter.next().is_none());
    };
    (? $iter:expr) => {
        assert!($iter.peek().is_none());
    };
}

pub mod ascii;
pub mod pushback;
pub mod unicode;
