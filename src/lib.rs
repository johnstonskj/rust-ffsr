/*!
One-line description.

More detailed description, with

# Example

YYYYY

# Features

*/

#![warn(
    unknown_lints,
    // ---------- Stylistic
    absolute_paths_not_starting_with_crate,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    macro_use_extern_crate,
    nonstandard_style, /* group */
    noop_method_call,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    // ---------- Future
    future_incompatible, /* group */
    rust_2021_compatibility, /* group */
    // ---------- Public
    missing_debug_implementations,
    // missing_docs,
    unreachable_pub,
    // ---------- Unsafe
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    // ---------- Unused
    unused, /* group */
)]
#![deny(
    // ---------- Public
    exported_private_dependencies,
    private_in_public,
    // ---------- Deprecated
    anonymous_parameters,
    bare_trait_objects,
    ellipsis_inclusive_range_patterns,
    // ---------- Unsafe
    deref_nullptr,
    drop_bounds,
    dyn_drop,
)]
#![forbid(unsafe_code)]

use std::{path::PathBuf, slice::SliceIndex};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SourceId {
    File(PathBuf),
    StdIn,
    String,
}

pub trait Sourced {
    fn source_id(&self) -> &SourceId;

    fn source_str(&self) -> &str;

    fn source_len(&self) -> usize {
        self.source_str().len()
    }

    fn get<I>(&self, index: I) -> Option<&I::Output>
    where
        I: SliceIndex<str>,
    {
        self.source_str().get(index)
    }
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[macro_use]
mod macros;

mod syntax;

pub mod error;

pub mod input;

pub mod lexer;

pub mod reader;
