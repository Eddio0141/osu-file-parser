use std::{error::Error, num::ParseIntError};

use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub struct ParseError(#[from] TimingPointParseError);

/// Error used when there was a problem parsing the [`TimingPoint`].
#[derive(Debug, Error)]
pub enum TimingPointParseError {
    // TODO replace all static strings with a enum instead
    /// A field is missing.
    #[error("The field {0} is missing")]
    MissingField(&'static str),
    #[error("There was a problem parsing the value {value}")]
    /// The field failed to parse to some type from a `str`.
    FieldParseError {
        #[source]
        source: Box<dyn Error>,
        value: String,
        field_name: &'static str,
    },
}

/// There was some problem parsing the [`SampleSet`].
#[derive(Debug, Error, PartialEq)]
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
