use std::num::ParseIntError;

use thiserror::Error;

use crate::helper::ParseZeroOneBoolError;

#[derive(Debug, Error)]
/// Error used when there was a problem parsing the `General` section.
pub enum ParseError {
    /// A Field in `General` failed to parse as a `Integer`.
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    /// A Field in `General` failed to parse as a `Decimal`.
    #[error(transparent)]
    ParseDecimalError(#[from] rust_decimal::Error),
    /// A field in `General` failed to parse as a `bool` from an `Integer`.
    #[error(transparent)]
    ParseZeroOneBoolError(#[from] ParseZeroOneBoolError),
    /// A field in `General` failed to parse as a `CountdownSpeed`.
    #[error(transparent)]
    CountdownSpeedParseError(#[from] CountdownSpeedParseError),
    /// A field in `General` failed to parse as an enum string.
    #[error(transparent)]
    StrumParseError(#[from] strum::ParseError),
    /// A field in `General` failed to parse as a `GameMode`.
    #[error(transparent)]
    GameModeParseError(#[from] GameModeParseError),
    /// When the line isn't in a `key: value` format.
    #[error("Invalid colon set, expected format of `key: value`")]
    InvalidColonSet,
    /// Invalid key name was used.
    #[error("The key doesn't exist in `General`")]
    InvalidKey,
}

/// Error used when there's an error parsing the string as enum.
#[derive(Debug, Error)]
pub enum GameModeParseError {
    /// Error when the `GameMode` is not a valid enum.
    #[error("Unknown `GameMode`")]
    UnknownType,
    /// Error trying to parse the `str` into an `Integer`.
    #[error(transparent)]
    ParseError(#[from] ParseIntError),
}

/// Error used when there's an error parsing the string as enum
#[derive(Debug, Error)]
pub enum CountdownSpeedParseError {
    /// The integer value is an unknown `CountdownSpeed` type.
    #[error("The integer value is an unknown `CountdownSpeed` type")]
    UnknownType,
    /// There was a problem converting from `str` to an `Integer`.
    #[error("There was a problem parsing the `str` as an `Integer`")]
    ParseError(#[from] ParseIntError),
}
