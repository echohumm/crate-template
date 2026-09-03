//! {{project-name}}

#![allow(unknown_lints)]
#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::multiple_unsafe_ops_per_block,
    clippy::undocumented_unsafe_blocks,
    clippy::unsafe_op_in_unsafe_fn,
    missing_docs
)]
#![warn(clippy::missing_errors_doc)]
#![allow(
    clippy::inline_always,
    clippy::borrow_as_ptr,
    clippy::module_name_repetitions,
    clippy::use_self,
    clippy::question_mark,
    // the From trait is evil and i hate it
    clippy::cast_lossless,
    unused_unsafe
)]
#![warn(unknown_lints)]
#![no_implicit_prelude]

extern crate core;
