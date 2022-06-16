use std::{error::Error, num::ParseIntError, str::FromStr};

use strum_macros::{EnumString, IntoStaticStr};
use thiserror::Error;

use crate::helper::macros::verbose_error_to_error;

#[derive(Debug, Error)]
#[error(transparent)]
pub struct ParseError(#[from] TimingPointParseError);

/// Error used when there was a problem parsing the [`TimingPoint`].
#[derive(Debug, Error, EnumString, IntoStaticStr)]
#[non_exhaustive]
pub enum TimingPointParseError {
    /// Invalid `time` value.
    #[error("Invalid `time` value")]
    InvalidTime,
    /// Missing `beat_length` field.
    #[error("Missing `beat_length` field")]
    MissingBeatLength,
    /// Invalid `beat_length` value.
    #[error("Invalid `beat_length` value")]
    InvalidBeatLength,
    /// Missing `meter` field.
    #[error("Missing `meter` field")]
    MissingMeter,
    /// Invalid `meter` value.
    #[error("Invalid `meter` value")]
    InvalidMeter,
    /// Missing `sample_set` field.
    #[error("Missing `sample_set` field")]
    MissingSampleSet,
    /// Invalid `sample_set` value.
    #[error("Invalid `sample_set` value")]
    InvalidSampleSet,
    /// Missing `sample_index` field.
    #[error("Missing `sample_index` field")]
    MissingSampleIndex,
    /// Invalid `sample_index` value.
    #[error("Invalid `sample_index` value")]
    InvalidSampleIndex,
    /// Missing `volume` field.
    #[error("Missing `volume` field")]
    MissingVolume,
    /// Invalid `volume` value.
    #[error("Invalid `volume` value")]
    InvalidVolume,
    /// Missing `effects` field.
    #[error("Missing `effects` field")]
    MissingEffects,
    /// Invalid `effects` value.
    #[error("Invalid `effects` value")]
    InvalidEffects,
    /// Missing `uninherited` field.
    #[error("Missing `uninherited` field")]
    MissingUninherited,
    /// Invalid `uninherited` value.
    #[error("Invalid `uninherited` value")]
    InvalidUninherited,
}

verbose_error_to_error!(TimingPointParseError);

/// There was some problem parsing the [`SampleSet`].
#[derive(Debug, Error, PartialEq)]
#[non_exhaustive]
pub enum SampleSetParseError {
    /// The value failed to parse from a `str`.
    #[error("There was a problem parsing {value} as an `Integer`")]
    ValueParseError {
        #[source]
        source: ParseIntError,
        value: String,
    },
    /// The `SampleSet` type is invalid.
    #[error("Expected `SampleSet` to have a value of 0 ~ 3, got {0}")]
    UnknownSampleSet(usize),
}

/// There was a problem parsing `str` as [`Effects`].
#[derive(Debug, Error)]
#[error("There was a problem parsing {value} as an `Integer`")]
pub struct EffectsParseError {
    #[source]
    pub source: ParseIntError,
    pub value: String,
}

/// Error used when `str` failed to parse as [`SampleIndex`].
#[derive(Debug, Error)]
#[error("There was a problem parsing {value} as an usize for `SampleIndex`")]
pub struct SampleIndexParseError {
    #[source]
    pub source: Box<dyn Error>,
    pub value: String,
}

/// Error for when there was a problem setting / parsing the volume.
#[derive(Debug, Error, PartialEq)]
#[non_exhaustive]
pub enum VolumeError {
    /// Error when volume is out of range of the 0 ~ 100 range.
    #[error("The volume was too high, expected 0 ~ 100, got {0}")]
    VolumeTooHigh(u8),
    /// There was a problem parsing the `str` as [`Volume`].
    #[error("There was a problem parsing a `str` as [`Volume`]")]
    VolumeParseError {
        #[source]
        source: ParseIntError,
        value: String,
    },
}
