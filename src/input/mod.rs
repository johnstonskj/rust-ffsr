/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::error::Error;
use crate::Sourced;
use crate::{input::iter::CharIndices, SourceId};
use std::borrow::{Borrow, Cow};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use tracing::trace;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Input<'a> {
    id: SourceId,
    source: Cow<'a, str>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn input_from_str(s: &str) -> Input<'_> {
    Input::from(s)
}

pub fn input_from_string<'a>(s: String) -> Input<'a> {
    Input::from(s)
}

pub fn input_from_stdin<'a>() -> Result<Input<'a>, Error> {
    input_from_reader(std::io::stdin(), SourceId::StdIn)
}

pub fn input_from_file<'a, P>(path: P) -> Result<Input<'a>, Error>
where
    P: Into<PathBuf>,
{
    let path = path.into();
    let file = File::open(&path)?;
    input_from_reader(file, SourceId::File(path))
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<'a> From<&'a str> for Input<'a> {
    fn from(s: &'a str) -> Self {
        Self {
            id: SourceId::String,
            source: Cow::Borrowed(s),
        }
    }
}

impl From<String> for Input<'_> {
    fn from(s: String) -> Self {
        trace!("input source {:?}; length: {}", s, s.len());
        Self {
            id: SourceId::String,
            source: Cow::Owned(s),
        }
    }
}

impl Sourced for Input<'_> {
    #[inline(always)]
    fn source_id(&self) -> &SourceId {
        &self.id
    }

    #[inline(always)]
    fn source_str(&self) -> &str {
        self.source.borrow()
    }
}

impl<'a> Input<'a> {
    #[inline(always)]
    pub fn char_indices(&'a self) -> CharIndices<'a> {
        CharIndices::new(&self.id, &self.source)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn input_from_reader<'a, R: Read>(mut reader: R, id: SourceId) -> Result<Input<'a>, Error> {
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    Ok(Input {
        id,
        source: Cow::Owned(buffer),
    })
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod indices;

pub mod iter;
