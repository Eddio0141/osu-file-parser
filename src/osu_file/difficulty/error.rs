use std::num::ParseIntError;

use thiserror::Error;

use crate::helper::macros::unreachable_err_impl;

#[derive(Debug, Error)]
#[non_exhaustive]
/// Error used when there was a problem parsing the `Difficulty` section.
pub enum ParseError {
    /// A Field in `Difficulty` failed to parse as a `Integer`.
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    /// A Field in `Difficulty` failed to parse as a `Decimal`.
    #[error(transparent)]
    ParseDecimalError(#[from] rust_decimal::Error),
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
