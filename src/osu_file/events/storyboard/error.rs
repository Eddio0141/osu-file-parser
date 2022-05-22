use nom::error::VerboseErrorKind;
use strum_macros::{EnumString, IntoStaticStr};
use thiserror::Error;

use std::{error::Error, num::ParseIntError, str::FromStr};

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

#[derive(Debug, Error, EnumString, IntoStaticStr)]
pub enum CommandParseError {
    /// Missing the `start_time` field.
    #[error("Missing the `start_time` field")]
    MissingStartTime,
    /// Invalid `start_time` field.
    #[error("Invalid `start_time` field")]
    InvalidStartTime,
    /// Missing the `loop_count` field.
    #[error("Missing the `loop_count` field")]
    MissingLoopCount,
    /// Invalid `loop_count` field.
    #[error("Invalid `loop_count` field")]
    InvalidLoopCount,
    /// Missing the "trigger_type" field.
    #[error("Missing the `trigger_type` field")]
    MissingTriggerType,
    /// Invalid `trigger_type` field.
    #[error("Invalid `trigger_type` field")]
    InvalidTriggerType,
    /// Invalid `group_number` field.
    #[error("Invalid `group_number` field")]
    InvalidGroupNumber,
    /// Missing `end_time` field.
    #[error("Missing `end_time` field")]
    MissingEndTime,
    /// Invalid `end_time` field.
    #[error("Invalid `end_time` field")]
    InvalidEndTime,
    /// Missing `easing` field.
    #[error("Missing `easing` field")]
    MissingEasing,
    /// Invalid `easing` field.
    #[error("Invalid `easing` field")]
    InvalidEasing,
    /// Missing colour's `red` field.
    #[error("Missing `red` field")]
    MissingRed,
    /// Missing colour's `green` field.
    #[error("Missing `green` field")]
    MissingGreen,
    /// Missing colour's `blue` field.
    #[error("Missing `blue` field")]
    MissingBlue,
    /// Invalid colour value, expected u8 integer.
    #[error("Invalid colour value, expected u8 integer")]
    InvalidColourValue,
    /// Invalid continuing colours, expected format of `,r,g,b,r,g,b,r,g...`
    #[error("Invalid continuing colours, expected format of `,r,g,b,r,g...`")]
    InvalidContinuingColours,
    /// Missing parameter's `parameter_type` field.
    #[error("Missing `parameter_type` field")]
    MissingParameterType,
    /// Invalid `parameter_type` field.
    #[error("Invalid `parameter_type` field")]
    InvalidParameterType,
    /// Invalid continuing parameters, expected format of `,param_type,param_type,param_type...`
    #[error("Invalid continuing parameters, expected format of `,param_type,param_type...`")]
    InvalidContinuingParameters,
}

impl From<nom::Err<nom::error::VerboseError<&str>>> for CommandParseError {
    fn from(err: nom::Err<nom::error::VerboseError<&str>>) -> Self {
        match err {
            nom::Err::Error(err) | nom::Err::Failure(err) => {
                for (_, err) in err.errors {
                    if let VerboseErrorKind::Context(context) = err {
                        return CommandParseError::from_str(context).unwrap();
                    }
                }

                unreachable!()
            }
            nom::Err::Incomplete(_) => unreachable!(),
        }
    }
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
