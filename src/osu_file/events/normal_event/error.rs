use std::num::ParseIntError;

use strum_macros::{EnumString, IntoStaticStr};
use thiserror::Error;

use crate::{helper::macros::verbose_error_to_error, InvalidRepr};

#[derive(Debug, Error, IntoStaticStr, EnumString)]
#[non_exhaustive]
pub enum ParseBackgroundError {
    #[error("Wrong event type")]
    WrongEventType,
    #[error("Missing `start_time` field")]
    MissingStartTime,
    #[error("Invalid `start_time` value")]
    InvalidStartTime,
    #[error("Missing `file_name` field")]
    MissingFileName,
    #[error("Missing `x` field")]
    MissingX,
    #[error("Invalid `x` value")]
    InvalidX,
    #[error("Missing `y` field")]
    MissingY,
    #[error("Invalid `y` value")]
    InvalidY,
}

verbose_error_to_error!(ParseBackgroundError);

#[derive(Debug, Error, IntoStaticStr, EnumString)]
#[non_exhaustive]
pub enum ParseVideoError {
    #[error("Wrong event type")]
    WrongEventType,
    #[error("Missing `start_time` field")]
    MissingStartTime,
    #[error("Invalid `start_time` value")]
    InvalidStartTime,
    #[error("Missing `file_name` field")]
    MissingFileName,
    #[error("Missing `x` field")]
    MissingX,
    #[error("Invalid `x` value")]
    InvalidX,
    #[error("Missing `y` field")]
    MissingY,
    #[error("Invalid `y` value")]
    InvalidY,
}

verbose_error_to_error!(ParseVideoError);

#[derive(Debug, Error, IntoStaticStr, EnumString)]
#[non_exhaustive]
pub enum ParseBreakError {
    #[error("Wrong event type")]
    WrongEventType,
    #[error("Missing `start_time` field")]
    MissingStartTime,
    #[error("Invalid `start_time` value")]
    InvalidStartTime,
    #[error("Missing `end_time` field")]
    MissingEndTime,
    #[error("Invalid `end_time` value")]
    InvalidEndTime,
}

verbose_error_to_error!(ParseBreakError);

#[derive(Debug, Error, IntoStaticStr, EnumString)]
#[non_exhaustive]
pub enum ParseColourTransformationError {
    #[error("Wrong event type")]
    WrongEventType,
    #[error("Missing `start_time` field")]
    MissingStartTime,
    #[error("Invalid `start_time` value")]
    InvalidStartTime,
    #[error("Missing `end_time` field")]
    MissingEndTime,
    #[error("Invalid `end_time` value")]
    InvalidEndTime,
    #[error("Missing `red` field")]
    MissingRed,
    #[error("Invalid `red` value")]
    InvalidRed,
    #[error("Missing `green` field")]
    MissingGreen,
    #[error("Invalid `green` value")]
    InvalidGreen,
    #[error("Missing `blue` field")]
    MissingBlue,
    #[error("Invalid `blue` value")]
    InvalidBlue,
}

verbose_error_to_error!(ParseColourTransformationError);

#[derive(Debug, Error, IntoStaticStr, EnumString)]
#[non_exhaustive]
pub enum ParseSpriteLegacyError {
    #[error("Wrong event type")]
    WrongEventType,
    #[error("Missing `layer` field")]
    MissingLayer,
    #[error("Invalid `layer` value")]
    InvalidLayer,
    #[error("Missing `origin` field")]
    MissingOrigin,
    #[error("Invalid `origin` value")]
    InvalidOrigin,
    #[error("Missing `file_name` field")]
    MissingFileName,
    #[error("Missing `x` field")]
    MissingX,
    #[error("Invalid `x` value")]
    InvalidX,
    #[error("Missing `y` field")]
    MissingY,
    #[error("Invalid `y` value")]
    InvalidY,
}

verbose_error_to_error!(ParseSpriteLegacyError);

#[derive(Debug, Error, IntoStaticStr, EnumString)]
#[non_exhaustive]
pub enum ParseAnimationLegacyError {
    #[error("Wrong event type")]
    WrongEventType,
    #[error("Missing `layer` field")]
    MissingLayer,
    #[error("Invalid `layer` value")]
    InvalidLayer,
    #[error("Missing `origin` field")]
    MissingOrigin,
    #[error("Invalid `origin` value")]
    InvalidOrigin,
    #[error("Missing `file_name` field")]
    MissingFileName,
    #[error("Missing `x` field")]
    MissingX,
    #[error("Invalid `x` value")]
    InvalidX,
    #[error("Missing `y` field")]
    MissingY,
    #[error("Invalid `y` value")]
    InvalidY,
}

verbose_error_to_error!(ParseAnimationLegacyError);

#[derive(Debug, Error, IntoStaticStr, EnumString)]
#[non_exhaustive]
pub enum ParseSampleLegacyError {
    #[error("Missing `time` field")]
    MissingTime,
    #[error("Invalid `time` value")]
    InvalidTime,
    #[error("Wrong event type")]
    WrongEventType,
    #[error("Missing `file_name` field")]
    MissingFileName,
    #[error("Missing `volume` field")]
    MissingVolume,
    #[error("Invalid `volume` value")]
    InvalidVolume,
    #[error("Missing `layer` field")]
    MissingLayer,
    #[error("Invalid `layer` value")]
    InvalidLayer,
}

verbose_error_to_error!(ParseSampleLegacyError);

#[derive(Debug, Error, IntoStaticStr)]
#[non_exhaustive]
pub enum ParseOriginTypeLegacyError {
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error(transparent)]
    InvalidRepr(#[from] InvalidRepr),
}

#[derive(Debug, Error, IntoStaticStr)]
#[non_exhaustive]
pub enum ParseLayerLegacyError {
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error(transparent)]
    InvalidRepr(#[from] InvalidRepr),
}
