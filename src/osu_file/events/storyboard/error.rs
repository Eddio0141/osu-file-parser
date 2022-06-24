use strum_macros::{EnumString, IntoStaticStr};
use thiserror::Error;

use std::num::ParseIntError;

use crate::helper::macros::verbose_error_to_error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum CommandPushError {
    #[error("Invalid indentation, expected {0}, got {1}")]
    InvalidIndentation(usize, usize),
}

#[derive(Debug, Error, IntoStaticStr, EnumString)]
#[non_exhaustive]
pub enum ObjectParseError {
    #[error("Unknown object type")]
    UnknownObjectType,
    #[error("Missing layer field")]
    MissingLayer,
    #[error("Invalid layer value")]
    InvalidLayer,
    #[error("Missing origin field")]
    MissingOrigin,
    #[error("Invalid origin value")]
    InvalidOrigin,
    #[error("Missing file path field")]
    MissingFilePath,
    #[error("Missing position x field")]
    MissingPositionX,
    #[error("Invalid position x value")]
    InvalidPositionX,
    #[error("Missing position y field")]
    MissingPositionY,
    #[error("Invalid position y value")]
    InvalidPositionY,
    #[error("Missing frame count field")]
    MissingFrameCount,
    #[error("Invalid frame count value")]
    InvalidFrameCount,
    #[error("Missing frame delay field")]
    MissingFrameDelay,
    #[error("Invalid frame delay value")]
    InvalidFrameDelay,
    #[error("Missing loop type field")]
    MissingLoopType,
    #[error("Invalid loop type value")]
    InvalidLoopType,
}

verbose_error_to_error!(ObjectParseError);

#[derive(Debug, Error)]
#[error("The filepath needs to be a path relative to where the .osu file is, not a full path such as `C:\\folder\\image.png`")]
pub struct FilePathNotRelative;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum EasingParseError {
    #[error(transparent)]
    ValueParseError(#[from] ParseIntError),
    #[error("Unknown easing type {0}")]
    UnknownEasingType(usize),
}

#[derive(Debug, Error, EnumString, IntoStaticStr)]
#[non_exhaustive]
pub enum CommandParseError {
    /// Unknown command type
    #[error("Unknown command type")]
    UnknownCommandType,
    /// Missing `start_time` field.
    #[error("Missing `start_time` field")]
    MissingStartTime,
    /// Invalid `start_time` value.
    #[error("Invalid `start_time` value")]
    InvalidStartTime,
    /// Missing `loop_count` field.
    #[error("Missing `loop_count` field")]
    MissingLoopCount,
    /// Invalid `loop_count` value.
    #[error("Invalid `loop_count` value")]
    InvalidLoopCount,
    /// Missing "trigger_type" field.
    #[error("Missing `trigger_type` field")]
    MissingTriggerType,
    /// Invalid `trigger_type` value.
    #[error("Invalid `trigger_type` value")]
    InvalidTriggerType,
    /// Invalid `group_number` value.
    #[error("Invalid `group_number` value")]
    InvalidGroupNumber,
    /// Missing `end_time` field.
    #[error("Missing `end_time` field")]
    MissingEndTime,
    /// Invalid `end_time` value.
    #[error("Invalid `end_time` value")]
    InvalidEndTime,
    /// Missing `easing` field.
    #[error("Missing `easing` field")]
    MissingEasing,
    /// Invalid `easing` value.
    #[error("Invalid `easing` value")]
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
    /// Invalid `red` value.
    #[error("Invalid `red` value")]
    InvalidRed,
    /// Invalid `green` value.
    #[error("Invalid `green` value")]
    InvalidGreen,
    /// Invalid `blue` value.
    #[error("Invalid `blue` value")]
    InvalidBlue,
    /// Invalid continuing colour value.
    #[error("Invalid continuing colour value")]
    InvalidContinuingColours,
    /// Missing parameter's `parameter_type` field.
    #[error("Missing `parameter_type` field")]
    MissingParameterType,
    /// Invalid `parameter_type` value.
    #[error("Invalid `parameter_type` value")]
    InvalidParameterType,
    /// Invalid continuing parameter value.
    #[error("Invalid continuing parameter value")]
    InvalidContinuingParameters,
    /// Missing `move_x` field.
    #[error("Missing `move_x` field")]
    MissingMoveX,
    /// Invalid `move_x` value.
    #[error("Invalid `move_x` value")]
    InvalidMoveX,
    /// Missing `move_y` field.
    #[error("Missing `move_y` field")]
    MissingMoveY,
    /// Invalid `move_y` value.
    #[error("Invalid `move_y` value")]
    InvalidMoveY,
    /// Invalid continuing move value.
    #[error("Invalid continuing move value")]
    InvalidContinuingMove,
    /// Missing `scale_x` field.
    #[error("Missing `scale_x` field")]
    MissingScaleX,
    /// Invalid `scale_x` value.
    #[error("Invalid `scale_x` value")]
    InvalidScaleX,
    /// Missing `scale_y` field.
    #[error("Missing `scale_y` field")]
    MissingScaleY,
    /// Invalid `scale_y` value.
    #[error("Invalid `scale_y` value")]
    InvalidScaleY,
    /// Invalid continuing scale value.
    #[error("Invalid continuing scale value")]
    InvalidContinuingScales,
    /// Missing `start_opacity` field.
    #[error("Missing `start_opacity` field")]
    MissingStartOpacity,
    /// Invalid `start_opacity` value.
    #[error("Invalid `start_opacity` value")]
    InvalidStartOpacity,
    /// Invalid continuing opacity value.
    #[error("Invalid continuing opacity value")]
    InvalidContinuingOpacities,
    /// Missing `start_scale` field.
    #[error("Missing `start_scale` field")]
    MissingStartScale,
    /// Invalid `start_scale` value.
    #[error("Invalid `start_scale` value")]
    InvalidStartScale,
    /// Invalid continuing scale value.
    #[error("Invalid continuing scale value")]
    InvalidContinuingScale,
    /// Missing `start_rotation` field.
    #[error("Missing `start_rotation` field")]
    MissingStartRotation,
    /// Invalid `start_rotation` value.
    #[error("Invalid `start_rotation` value")]
    InvalidStartRotation,
    /// Invalid continuing rotation value.
    #[error("Invalid continuing rotation value")]
    InvalidContinuingRotation,
}

verbose_error_to_error!(CommandParseError);

#[derive(Debug, Error)]
#[non_exhaustive]
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
#[non_exhaustive]
pub enum ContinuingRGBSetError {
    #[error("continuing fields index out of bounds")]
    IndexOutOfBounds,
    #[error(transparent)]
    InvalidFieldOption(#[from] InvalidColourFieldOption),
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum InvalidColourFieldOption {
    #[error("continuing fields green field is none without it being the last item in the continuing fields")]
    Green,
    #[error("continuing fields blue field is none without it being the last item in the continuing fields")]
    Blue,
}

#[derive(Debug, Error)]
#[non_exhaustive]
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
