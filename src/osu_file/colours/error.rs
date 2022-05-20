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
