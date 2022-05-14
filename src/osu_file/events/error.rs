use thiserror::Error;

use std::error::Error;

use super::storyboard::error::*;

/// Errors used when there was a problem parsing an [`Event`] from a `str`.
#[derive(Debug, Error)]
pub enum ParseError {
    /// A field is missing from the [`Event`].
    #[error("The field {0} is missing")]
    MissingField(&'static str),
    /// There was an error attempting to parse `str` as a field type.
    #[error("There was a problem parsing the {field_name} field from a `str`")]
    FieldParseError {
        #[source]
        source: Box<dyn Error>,
        value: String,
        field_name: &'static str,
    },
    /// The input has nothing or whitespace.
    #[error("The input is empty")]
    EmptyString,
    /// A `storyboard` `command` was used without defined sprite or animation sprite.
    #[error("A storyboard command was used without defined sprite or animation sprite")]
    StoryboardCmdWithNoSprite,
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
