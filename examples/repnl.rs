use ffsr::input::input_from_stdin;
use ffsr::lexer::Lexer;
use ffsr::reader::Reader;
use ffsr::Sourced;
use ffsr::{error::Error, input::input_from_file};
use std::{fmt::Display, path::PathBuf};
use structopt::StructOpt;
use tracing::info;
use tracing::subscriber::SetGlobalDefaultError;
use tracing_subscriber::filter::{EnvFilter, LevelFilter, ParseError};

// ------------------------------------------------------------------------------------------------
// Command-Line Structure
// ------------------------------------------------------------------------------------------------

const TOOL_NAME: &str = "ffsr-repnl";

#[derive(Debug, StructOpt)]
#[structopt(name = TOOL_NAME)]
struct Cli {
    /// The level of logging to perform, from off to trace
    #[structopt(long, short = "v", parse(from_occurrences))]
    verbose: i8,

    #[structopt(long, short)]
    file: Option<PathBuf>,
}

// ------------------------------------------------------------------------------------------------
// Command-Line Errors
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
enum TracingSource {
    EnvFilter(ParseError),
    SetGlobal(SetGlobalDefaultError),
}

impl Display for TracingSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::EnvFilter(e) => e.to_string(),
                Self::SetGlobal(e) => e.to_string(),
            }
        )
    }
}

impl From<ParseError> for TracingSource {
    fn from(e: ParseError) -> Self {
        Self::EnvFilter(e)
    }
}

impl From<SetGlobalDefaultError> for TracingSource {
    fn from(e: SetGlobalDefaultError) -> Self {
        Self::SetGlobal(e)
    }
}

impl std::error::Error for TracingSource {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::EnvFilter(source) => Some(source),
            Self::SetGlobal(source) => Some(source),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Main Function
// ------------------------------------------------------------------------------------------------

fn main() -> Result<(), ToolError> {
    let args = Cli::from_args();

    init_tracing(args.verbose)?;

    let input = match args.file {
        None => {
            println!("Type some input, put ^D on a blank line to execute");
            input_from_stdin()?
        }
        Some(file) => input_from_file(file)?,
    };

    let reader = Reader::from(Lexer::from(input));

    for datum in reader.iter() {
        match datum {
            Ok(datum) => {
                println!("{}: {}", datum.type_string(), datum);
                println!("{}: {:#?}", datum.type_string(), datum);
            }
            Err(e) => {
                e.print(reader.source_str());
            }
        }
    }

    Ok(())
}

// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
enum ToolError {
    TracingInitFailed(TracingSource),
    ReaderError(Error),
}

impl Display for ToolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::TracingInitFailed(e) => e.to_string(),
                Self::ReaderError(e) => e.to_string(),
            }
        )
    }
}

impl From<TracingSource> for ToolError {
    fn from(e: TracingSource) -> Self {
        Self::TracingInitFailed(e)
    }
}

impl From<Error> for ToolError {
    fn from(e: Error) -> Self {
        Self::ReaderError(e)
    }
}

impl std::error::Error for ToolError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::TracingInitFailed(source) => Some(source),
            Self::ReaderError(source) => Some(source),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn init_tracing(verbosity: i8) -> Result<(), ToolError> {
    let log_level = match verbosity {
        0 => LevelFilter::OFF,
        1 => LevelFilter::ERROR,
        2 => LevelFilter::WARN,
        3 => LevelFilter::INFO,
        4 => LevelFilter::DEBUG,
        _ => LevelFilter::TRACE,
    };

    let filter = EnvFilter::from_default_env()
        .add_directive(
            format!("{}={}", module_path!(), log_level)
                .parse()
                .map_err(|e| ToolError::TracingInitFailed(TracingSource::EnvFilter(e)))?,
        )
        .add_directive(
            format!("{}={}", "ffsr", log_level)
                .parse()
                .map_err(|e| ToolError::TracingInitFailed(TracingSource::EnvFilter(e)))?,
        );

    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .pretty()
        .with_test_writer()
        .with_target(false)
        .with_level(true)
        .with_file(true)
        .with_line_number(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .map_err(|e| ToolError::TracingInitFailed(TracingSource::SetGlobal(e)))?;
    info!("Log level set to `LevelFilter::{:?}`", log_level);

    Ok(())
}
