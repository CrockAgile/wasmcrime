#![doc = include_str!("../README.md")]
#![warn(missing_docs, clippy::all, clippy::pedantic, clippy::cargo)]

/// Binary format of wasm modules
pub mod binary;

/// WIP
/// # Panics
/// always panics because this is a work in progress
pub fn init() {
    panic!("WIP")
}

/// Crate wide error type
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Incomplete bytes when decoding
    #[error("incomplete bytes when decoding {0}")]
    Incomplete(String),
}

/// Crate wide result type
pub type Result<T> = core::result::Result<T, Error>;
