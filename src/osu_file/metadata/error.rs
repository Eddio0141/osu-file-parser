use thiserror::Error;

use std::num::ParseIntError;

use crate::helper::macros::unreachable_err_impl;

#[derive(Debug, Error)]
#[non_exhaustive]
/// Error used when there was a problem parsing the `Metadata` section.
pub enum ParseError {
    /// There is a duplicate field in the `Metadata` section.
    #[error("There is a duplicate field in the `Metadata` section")]
    DuplicateField,
    /// A Field in `Metadata` failed to parse as a `Integer`.
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    /// When the line isn't in a `key: value` format.
    #[error("Invalid colon set, expected format of `key: value`")]
    InvalidColonSet,
    /// Invalid key name was used.
    #[error("The key doesn't exist in `General`")]
    InvalidKey,
}

unreachable_err_impl!(ParseError);
