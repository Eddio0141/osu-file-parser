use thiserror::Error;

use std::{error::Error, num::ParseIntError};

#[derive(Debug, Error)]
pub enum CommandPushError {
    #[error("Invalid indentation, expected {0}, got {1}")]
    InvalidIndentation(usize, usize),
}

#[derive(Debug, Error)]
pub enum ObjectParseError {
    #[error("Unknown object type {0}")]
    UnknownObjectType(String),
    #[error("The object is missing the field {0}")]
    MissingField(&'static str),
    #[error("The field {field_name} failed to parse from a `str` to a type")]
    FieldParseError {
        #[source]
        source: Box<dyn Error>,
        field_name: &'static str,
        value: String,
    },
}

#[derive(Debug, Error)]
#[error("The filepath needs to be a path relative to where the .osu file is, not a full path such as `C:\\folder\\image.png`")]
pub struct FilePathNotRelative;

#[derive(Debug, Error)]
pub enum EasingParseError {
    #[error(transparent)]
    ValueParseError(#[from] ParseIntError),
    #[error("Unknown easing type {0}")]
    UnknownEasingType(usize),
}

#[derive(Debug, Error)]
pub enum CommandParseError {
    #[error("The field {0} is missing from the [`Command`]")]
    MissingField(&'static str),
    #[error("The event type {0} is unknown")]
    UnknownEvent(String),
    #[error("Attempted to parse {value} from a `str` as another type")]
    FieldParseError {
        #[source]
        source: Box<dyn Error>,
        value: String,
    },
    #[error("Invalid easing, {0}")]
    InvalidEasing(usize),
    #[error("Invalid field ending formatting")]
    InvalidFieldEnding,
}

#[derive(Debug, Error)]
pub enum TriggerTypeParseError {
    #[error("There are too many `HitSound` fields: {0}")]
    TooManyHitSoundFields(usize),
    #[error("There was a problem parsing a field")]
    FieldParseError {
        #[from]
        source: ParseIntError,
    },
    #[error("Unknown trigger type {0}")]
    UnknownTriggerType(String),
    #[error("Unknown `HitSound` type {0}")]
    UnknownHitSoundType(String),
}

#[derive(Debug, Error)]
pub enum ContinuingRGBSetError {
    #[error("continuing fields index out of bounds")]
    IndexOutOfBounds,
    #[error(transparent)]
    InvalidFieldOption(#[from] InvalidColourFieldOption),
}

#[derive(Debug, Error)]
pub enum InvalidColourFieldOption {
    #[error("continuing fields green field is none without it being the last item in the continuing fields")]
    Green,
    #[error("continuing fields blue field is none without it being the last item in the continuing fields")]
    Blue,
}

#[derive(Debug, Error)]
pub enum ContinuingSetError {
    #[error("continuing fields index out of bounds")]
    IndexOutOfBounds,
    #[error(
        "continuing fields 2nd field is none without it being the last item in the continuing fields")]
    InvalidSecondFieldOption,
}

#[derive(Debug, Error)]
#[error(
    "continuing fields 2nd field is none without it being the last item in the continuing fields"
)]
pub struct InvalidSecondFieldOption;
