use thiserror::Error;

use std::num::ParseIntError;

#[derive(Debug, Error)]
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

impl From<()> for ParseError {
    fn from(_: ()) -> Self {
        unreachable!()
    }
}
