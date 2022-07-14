use std::num::{ParseIntError, TryFromIntError};

use strum_macros::{EnumString, IntoStaticStr};
use thiserror::Error;

use crate::helper::macros::verbose_error_to_error;

#[derive(Debug, Error)]
#[error(transparent)]
pub struct ParseError(#[from] TimingPointParseError);

/// Error used when there was a problem parsing the [`TimingPoint`][super::TimingPoint].
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

/// There was some problem parsing the [`SampleSet`][super::SampleSet].
#[derive(Debug, Error, PartialEq)]
#[non_exhaustive]
pub enum SampleSetParseError {
    /// The value failed to parse from a `str`.
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    /// Unknown `SampleSet` variant.
    #[error("Unknown `SampleSet` variant")]
    UnknownSampleSet,
}

/// There was a problem parsing `str` as [`Effects`][super::Effects].
#[derive(Debug, Error)]
#[error(transparent)]
pub struct EffectsParseError(#[from] ParseIntError);

#[derive(Debug, Error)]
pub enum SampleIndexParseError {
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error(transparent)]
    TryFromIntError(#[from] TryFromIntError),
}

/// Error for when there was a problem setting / parsing the volume.
#[derive(Debug, Error, PartialEq)]
#[non_exhaustive]
pub enum VolumeError {
    /// Error when volume is out of range of the 0 ~ 100 range.
    #[error("The volume was too high, expected 0 ~ 100, got {0}")]
    VolumeTooHigh(u8),
    /// There was a problem parsing the `str` as [`Volume`][super::Volume].
    #[error(transparent)]
    VolumeParseError(#[from] ParseIntError),
}
