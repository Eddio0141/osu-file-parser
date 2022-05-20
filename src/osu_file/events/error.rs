use std::str::FromStr;

use nom::error::VerboseErrorKind;
use strum_macros::{EnumString, IntoStaticStr};
use thiserror::Error;

use super::storyboard::error::*;

/// Errors used when there was a problem parsing an [`Event`] from a `str`.
#[derive(Debug, Error)]
pub enum ParseError {
    /// When the line isn't in a `key: value` format.
    #[error("Invalid colon set, expected format of `key: value`")]
    InvalidColonSet,
    /// Invalid key name was used.
    #[error("The key doesn't exist in `General`")]
    InvalidKey,
    /// Missing the `start_time` field.
    #[error("Missing the `start_time` field")]
    MissingStartTime,
    /// Failed to parse the `start_time` field.
    #[error("Failed to parse the `start_time` field")]
    ParseStartTime,
    /// A `storyboard` `command` was used without defined sprite or animation sprite.
    #[error("A storyboard command was used without defined sprite or animation sprite")]
    StoryboardCmdWithNoSprite,
    #[error(transparent)]
    EventParamsParseError(#[from] EventParamsParseError),
    #[error(transparent)]
    CommandPushError(#[from] CommandPushError),
    #[error(transparent)]
    CommandParseError(#[from] CommandParseError),
    /// There was a problem parsing some `storyboard` element.
    #[error(transparent)]
    StoryboardObjectParseError(#[from] ObjectParseError),
}

#[derive(Debug, Error, EnumString, IntoStaticStr)]
pub enum EventParamsParseError {
    #[error("Missing the `start_time` field")]
    MissingStartTime,
    #[error("Unknown event type")]
    UnknownEventType,
    #[error("Missing the `file_name` field")]
    MissingFileName,
    #[error("Missing the `x_offset` field")]
    MissingXOffset,
    #[error("Invalid `x_offset` field")]
    InvalidXOffset,
    #[error("Missing the `y_offset` field")]
    MissingYOffset,
    #[error("Invalid `y_offset` field")]
    InvalidYOffset,
    #[error("Missing the `end_time` field")]
    MissingEndTime,
    #[error("Invalid `end_time` field")]
    InvalidEndTime,
    #[error("Missing the `red` field")]
    MissingRed,
    #[error("Invalid `red` field")]
    InvalidRed,
    #[error("Missing the `green` field")]
    MissingGreen,
    #[error("Invalid `green` field")]
    InvalidGreen,
    #[error("Missing the `blue` field")]
    MissingBlue,
    #[error("Invalid `blue` field")]
    InvalidBlue,
}

impl From<nom::Err<nom::error::VerboseError<&str>>> for EventParamsParseError {
    fn from(err: nom::Err<nom::error::VerboseError<&str>>) -> Self {
        match err {
            nom::Err::Error(err) | nom::Err::Failure(err) => {
                for (_, err) in err.errors {
                    if let VerboseErrorKind::Context(context) = err {
                        return EventParamsParseError::from_str(context).unwrap();
                    }
                }

                unreachable!()
            }
            // should never happen
            nom::Err::Incomplete(_) => unreachable!(),
        }
    }
}
