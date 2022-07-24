use std::num::ParseIntError;

use thiserror::Error;

use crate::helper::macros::unreachable_err_impl;

#[derive(Debug, Error)]
#[non_exhaustive]
/// Error used when there was a problem parsing the `Editor` section.
pub enum ParseError {
    /// A Field in `Editor` failed to parse as a `Integer`.
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    /// When the line isn't in a `key: value` format.
    #[error("Invalid colon set, expected format of `key: value`")]
    InvalidColonSet,
    /// Invalid key name was used.
    #[error("The key doesn't exist in `Editor`")]
    InvalidKey,
    /// Invalid comma list, expected format of `key: value, value, value, ...`
    #[error("Invalid comma list, expected format of `key: value, value, value, ...`")]
    InvalidCommaList,
    /// Duplicate field in `Editor`.
    #[error("Duplicate field in `Editor`")]
    DuplicateField,
}

unreachable_err_impl!(ParseError);
