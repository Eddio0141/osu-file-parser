use std::num::ParseIntError;

use thiserror::Error;

use crate::helper::{macros::unreachable_err_impl, ParseZeroOneBoolError};

#[derive(Debug, Error)]
#[non_exhaustive]
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
    ParseCountdownSpeedError(#[from] ParseCountdownSpeedError),
    /// A field in `General` failed to parse as an enum string.
    #[error(transparent)]
    ParseStrumError(#[from] strum::ParseError),
    /// A field in `General` failed to parse as a `GameMode`.
    #[error(transparent)]
    ParseGameModeError(#[from] ParseGameModeError),
    #[error(transparent)]
    ParseSampleSetError(#[from] ParseSampleSetError),
    #[error(transparent)]
    ParseOverlayPositionError(#[from] ParseOverlayPositionError),
    /// When the line isn't in a `key: value` format.
    #[error("Invalid colon set, expected format of `key: value`")]
    InvalidColonSet,
    /// Invalid key name was used.
    #[error("The key doesn't exist in `General`")]
    InvalidKey,
    /// Duplicate field in `General` were found.
    #[error("Duplicate field were found in `General`")]
    DuplicateField,
    /// Field doesn't exist in version.
    #[error("Field doesn't exist in version")]
    InvalidVersion,
}

unreachable_err_impl!(ParseError);

/// Error used when there's an error parsing the string as enum.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ParseGameModeError {
    /// Error when the `GameMode` is not a valid enum.
    #[error("Unknown `GameMode` variant")]
    UnknownVariant,
    /// Error trying to parse the `str` into an `Integer`.
    #[error(transparent)]
    ParseError(#[from] ParseIntError),
}

/// Error used when there's an error parsing the string as enum
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ParseCountdownSpeedError {
    /// The integer value is an unknown `CountdownSpeed` type.
    #[error("Unknown `CountdownSpeed` variant")]
    UnknownVariant,
    /// There was a problem converting from `str` to an `Integer`.
    #[error("There was a problem parsing the `str` as an `Integer`")]
    ParseError(#[from] ParseIntError),
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ParseOverlayPositionError {
    #[error("Unknown `OverlayPosition` variant")]
    UnknownVariant,
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ParseSampleSetError {
    #[error("Unknown `SampleSet` variant")]
    UnknownVariant,
}
