use std::str::FromStr;

use nom::error::VerboseErrorKind;
use strum_macros::{EnumString, IntoStaticStr};
use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub struct ParseError(#[from] ColourParseError);

/// Error used when there was a problem parsing a `str` as a `Colour`.
#[derive(Debug, Error, EnumString, IntoStaticStr)]
pub enum ColourParseError {
    /// Invalid additive combo count.
    #[error("Invalid additive combo count")]
    InvalidComboCount,
    /// Invalid colon separator.
    #[error("Invalid colon separator")]
    InvalidColonSeparator,
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
    /// Unknown colour type.
    #[error("Unknown colour type")]
    UnknownColourType,
}

impl From<nom::Err<nom::error::VerboseError<&str>>> for ColourParseError {
    fn from(err: nom::Err<nom::error::VerboseError<&str>>) -> Self {
        match err {
            nom::Err::Error(err) | nom::Err::Failure(err) => {
                for (_, err) in err.errors {
                    if let VerboseErrorKind::Context(context) = err {
                        return ColourParseError::from_str(context).unwrap();
                    }
                }

                unreachable!()
            }
            nom::Err::Incomplete(_) => unreachable!(),
        }
    }
}
