use thiserror::Error;

use std::error::Error;

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

#[derive(Debug, Error)]
pub enum EventParamsParseError {
    #[error("The field {0} is missing from the event parameter")]
    MissingField(&'static str),
    #[error("The field {field_name} failed to parse from a `str`")]
    ParseFieldError {
        #[source]
        source: Box<dyn Error>,
        value: String,
        field_name: &'static str,
    },
    #[error("Unknown field `{0}`")]
    UnknownParamType(String),
}
