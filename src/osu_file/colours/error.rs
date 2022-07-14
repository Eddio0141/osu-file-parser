use strum_macros::{EnumString, IntoStaticStr};
use thiserror::Error;

use crate::helper::macros::verbose_error_to_error;

#[derive(Debug, Error)]
#[error(transparent)]
pub struct ParseError(#[from] ColourParseError);

/// Error used when there was a problem parsing a `str` as a `Colour`.
#[derive(Debug, Error, EnumString, IntoStaticStr)]
#[non_exhaustive]
pub enum ColourParseError {
    /// Invalid additive combo count.
    #[error("Invalid additive combo count")]
    InvalidComboCount,
    /// Invalid colon separator.
    #[error("Invalid colon separator")]
    InvalidColonSeparator,
    #[error(transparent)]
    #[strum(disabled)]
    RgbParseError(#[from] RgbParseError),
    /// Unknown colour type.
    #[error("Unknown colour type")]
    UnknownColourType,
}

verbose_error_to_error!(ColourParseError);

#[derive(Debug, Error, EnumString, IntoStaticStr)]
pub enum RgbParseError {
    /// Invalid red value.
    #[error("Invalid red value")]
    InvalidRed,
    /// Invalid green value.
    #[error("Invalid green value")]
    InvalidGreen,
    /// Invalid blue value.
    #[error("Invalid blue value")]
    InvalidBlue,
    /// Missing green value.
    #[error("Missing green value")]
    MissingGreen,
    /// Missing blue value.
    #[error("Missing blue value")]
    MissingBlue,
}

verbose_error_to_error!(RgbParseError);
