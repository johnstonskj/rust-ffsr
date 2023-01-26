use tracing::{subscriber::DefaultGuard, Level};

#[inline(always)]
pub fn init_tracing() -> DefaultGuard {
    let subscriber = tracing_subscriber::fmt()
        .pretty()
        .with_test_writer()
        .with_max_level(Level::TRACE)
        .with_target(false)
        .with_level(true)
        .with_file(true)
        .with_line_number(true)
        .finish();
    tracing::subscriber::set_default(subscriber)
}

pub mod datum;
pub mod input;
pub mod lexer;
pub mod reader;
