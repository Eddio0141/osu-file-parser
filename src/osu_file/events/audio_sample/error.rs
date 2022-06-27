use std::num::ParseIntError;

use strum_macros::{EnumString, IntoStaticStr};
use thiserror::Error;

use crate::helper::macros::verbose_error_to_error;

#[derive(Debug, Error, IntoStaticStr, EnumString)]
pub enum AudioSampleParseError {
    #[error("Missing `time` field")]
    MissingTime,
    #[error("Invalid `time` value")]
    InvalidTime,
    #[error("Missing `layer` field")]
    MissingLayer,
    #[error("Invalid `layer` value")]
    InvalidLayer,
    #[error("Missing `volume` field")]
    MissingVolume,
    #[error("Invalid `volume` value")]
    InvalidVolume,
    #[error("Wrong event type")]
    WrongEvent,
    #[error("Missing `filepath` field")]
    MissingFilepath,
}

verbose_error_to_error!(AudioSampleParseError);

#[derive(Debug, Error)]
pub enum VolumeSetError {
    /// The volume was too high, being higher than `100`.
    #[error("The volume was too high. Expected 1 ~ 100, got {0}")]
    VolumeTooHigh(u8),
    /// The volume was expected to be in the range `1` ~ `100`, got `0`.
    #[error("The volume was attempted to set to 0. Expects a value from 1 ~ 100")]
    VolumeTooLow,
}

#[derive(Debug, Error)]
pub enum VolumeParseError {
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error(transparent)]
    VolumeSetError(#[from] VolumeSetError),
}
