use thiserror::Error;

use std::num::ParseIntError;

#[derive(Debug, Error)]
#[error(transparent)]
pub struct ColoursParseError(#[from] ColourParseError);

/// Error used when there was a problem parsing a `str` as a `Colour`.
#[derive(Debug, Error)]
pub enum ColourParseError {
    /// The colour key was invalid.
    #[error("The colour key was an invalid key, got {0}")]
    InvalidKey(String),
    /// Attempted to parse a `str` as an `Integer` for the `ComboCount`.
    #[error("Attempted to parse {value} as an `Integer` for `ComboCount`")]
    ComboCountParseError {
        #[source]
        source: ParseIntError,
        value: String,
    },
    /// There is no value. Expected `key : value` pair.
    #[error("There is no value, expected `key : value` pairs, got {0}")]
    NoValue(String),
    #[error(transparent)]
    RGBParseError {
        #[from]
        source: RGBParseError,
    },
}

/// Error when there was a problem parsing `str` as a `RGB` struct.
#[derive(Debug, Error)]
pub enum RGBParseError {
    /// No red field defined.
    #[error("There is no red field defined")]
    NoRedDefined,
    /// No green field defined.
    #[error("There is no green field defined")]
    NoGreenDefined,
    /// No blue field defined.
    #[error("There is no blue field defined")]
    NoBlueDefined,
    /// More than 3 fields defined.
    #[error("There is more than 3 fields defined `{0}`, only needs the `red`,`green`,`blue`")]
    MoreThanThreeFields(String),
    /// There was a problem parsing the field as an `Integer`.
    #[error(
        "There was a problem parsing the {field_name} field with the value {value} as an `Integer`"
    )]
    ValueParseError {
        #[source]
        source: ParseIntError,
        value: String,
        field_name: &'static str,
    },
}
