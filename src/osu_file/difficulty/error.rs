use thiserror::Error;

use crate::helper::macros::unreachable_err_impl;

#[derive(Debug, Error)]
#[non_exhaustive]
/// Error used when there was a problem parsing the `Difficulty` section.
pub enum ParseError {
    /// When the line isn't in a `key: value` format.
    #[error("Invalid colon set, expected format of `key: value`")]
    InvalidColonSet,
    /// Invalid key name was used.
    #[error("The key doesn't exist in `Difficulty`")]
    InvalidKey,
    /// There is a duplicate field in `Difficulty`.
    #[error("Duplicate field in `Difficulty`")]
    DuplicateField,
}

unreachable_err_impl!(ParseError);
