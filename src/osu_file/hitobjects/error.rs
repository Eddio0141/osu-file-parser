//! Module defining `error` types that's used for the `hitobject` related modules.

use std::{
    error::Error,
    num::{ParseIntError, TryFromIntError},
    str::FromStr,
};

use strum_macros::{EnumString, IntoStaticStr};
use thiserror::Error;

use crate::helper::macros::verbose_error_to_error;

#[derive(Debug, Error)]
#[error(transparent)]
pub struct ParseError(#[from] HitObjectParseError);

#[derive(Debug, Error)]
#[error("Expected combo skip count to be 3 bits, got {0}")]
pub struct ComboSkipCountTooHigh(pub u8);

#[derive(Debug, Error)]
#[non_exhaustive]
/// Error used when there was a problem parsing a `str` having a `F:S` format.
pub enum ColonSetParseError {
    /// When the first item is missing.
    #[error("Missing the first item in the colon set. Colon set requires to have the format `first:second`")]
    MissingFirstItem,
    /// When the second item is missing.
    #[error("Missing the second item in the colon set. Colon set requires to have the format `first:second`")]
    MissingSecondItem,
    /// There are more than 2 items defined.
    #[error("There is more than 2 items in the colon set: {0}. Colon set only has two items: `first:second`")]
    MoreThanTwoItems(String),
    /// There was some problem parsing the value.
    #[error("There was a problem parsing the `str` \"{value}\" to a colon set item")]
    ValueParseError {
        #[source]
        source: Box<dyn Error>,
        value: String,
    },
}

#[derive(Debug, Error, EnumString, IntoStaticStr)]
#[non_exhaustive]
/// Error used when there was a problem parsing a `str` into a [`HitObject`][super::HitObject].
pub enum HitObjectParseError {
    /// Invalid `hit_sound` value.
    #[error("Invalid `hit_sound` value")]
    InvalidHitSound,
    /// Missing `hit_sound` field.
    #[error("Missing `hit_sound` field")]
    MissingHitSound,
    /// Missing `hit_sample` field.
    #[error("Missing `hit_sample` field")]
    MissingHitSample,
    /// Invalid `hit_sample` value.
    #[error("Invalid `hit_sample` value")]
    InvalidHitSample,
    /// Invalid `x` value.
    #[error("Invalid `x` value")]
    InvalidX,
    /// Missing `x` field.
    #[error("Missing `x` field")]
    MissingX,
    /// Invalid `y` value.
    #[error("Invalid `y` value")]
    InvalidY,
    /// Missing `y` field.
    #[error("Missing `y` field")]
    MissingY,
    /// Missing `obj_type` field.
    #[error("Missing `obj_type` field")]
    MissingObjType,
    /// Invalid `obj_type` value.
    #[error("Invalid `obj_type` value")]
    InvalidObjType,
    /// Missing `time` field.
    #[error("Missing `time` field")]
    MissingTime,
    /// Invalid `time` value.
    #[error("Invalid `time` value")]
    InvalidTime,
    /// Invalid `curve_type` value.
    #[error("Invalid `curve_type` value")]
    InvalidCurveType,
    /// Missing `curve_type` field.
    #[error("Missing `curve_type` field")]
    MissingCurveType,
    /// Invalid `curve_point` value.
    #[error("Invalid `curve_point` value")]
    InvalidCurvePoint,
    /// Missing `curve_point` field.
    #[error("Missing `curve_point` field")]
    MissingCurvePoint,
    /// Invalid `edge_sound` value.
    #[error("Invalid `edge_sound` value")]
    InvalidEdgeSound,
    /// Missing `edge_sound` field.
    #[error("Missing `edge_sound` field")]
    MissingEdgeSound,
    /// Invalid `edge_set` value.
    #[error("Invalid `edge_set` value")]
    InvalidEdgeSet,
    /// Missing `edge_set` field.
    #[error("Missing `edge_set` field")]
    MissingEdgeSet,
    /// Invalid `slides_count` value.
    #[error("Invalid `slides_count` value")]
    InvalidSlidesCount,
    /// Missing `slides_count` field.
    #[error("Missing `slides_count` field")]
    MissingSlidesCount,
    /// Invalid `length` value.
    #[error("Invalid `length` value")]
    InvalidLength,
    /// Missing `length` field.
    #[error("Missing `length` field")]
    MissingLength,
    /// Invalid `end_time` value.
    #[error("Invalid `end_time` value")]
    InvalidEndTime,
    /// Missing `end_time` field.
    #[error("Missing `end_time` field")]
    MissingEndTime,
    /// Unknown object type.
    #[error("Unknown object type")]
    UnknownObjType,
}

verbose_error_to_error!(HitObjectParseError);

#[derive(Debug, Error, EnumString, IntoStaticStr)]
#[non_exhaustive]
/// Error used when there was a problem parsing a `str` into a [`hitsample`][super::types::HitSample].
pub enum HitSampleParseError {
    #[error("Invalid `sample_set` value")]
    InvalidSampleSet,
    #[error("Invalid `index` value")]
    InvalidIndex,
    #[error("Invalid `volume` value")]
    InvalidVolume,
    #[error("Missing field separator")]
    MissingSeparator,
}

verbose_error_to_error!(HitSampleParseError);

#[derive(Debug, Error)]
#[non_exhaustive]
/// Error used when there was a problem parsing a `str` into a [`sampleset`][super::types::SampleSet].
pub enum SampleSetParseError {
    /// The `str` had a value higher than 3.
    #[error("Invalid `SampleSet` type: {0}")]
    UnknownType(usize),
    /// There was a problem parsing a `str` as an integer first.
    #[error("There was a problem parsing the `str` into an integer first")]
    ValueParseError(#[from] ParseIntError),
}

#[derive(Debug, Error)]
#[non_exhaustive]
/// Error used when the user tried to set [`volume`][super::types::Volume]'s field as something invalid.
pub enum VolumeSetError {
    /// The volume was too high, being higher than `100`.
    #[error("The volume was too high. Expected 1 ~ 100, got {0}")]
    VolumeTooHigh(u8),
    /// The volume has a value `0`, which is "invalid".
    /// In the osu file documentation, the volume of 0 means the `timingpoint`'s volume is used instead.
    /// I handle that special case differently to make it more clear to the user what's going on.
    #[error("The volume was attempted to set to 0. Expects a value from 1 ~ 100")]
    VolumeTooLow,
}

#[derive(Debug, Error)]
#[non_exhaustive]
/// Error used when there was a problem parsing a `volume` from a `str`.
pub enum VolumeParseError {
    /// The volume was too high, being higher than `100`.
    #[error("The volume was too high. Expected 1 ~ 100, got {0}")]
    VolumeTooHigh(u8),
    /// The volume has a value `0`, which is "invalid".
    /// In the osu file documentation, the volume of 0 means the `timingpoint`'s volume is used instead.
    /// I handle that special case differently to make it more clear to the user what's going on.
    #[error("The volume was attempted to set to 0. Expects a value from 1 ~ 100")]
    VolumeTooLow,
    /// An invalid `str` was attempted to be parsed as an `Integer`.
    #[error("There was a problem parsing \"{0}\" to an `Integer`")]
    InvalidString(#[from] ParseIntError),
}

#[derive(Debug, Error)]
#[error("Unknown `CurveType` value was tried to be parsed: {0}")]
/// Error used when an unknown [`CurveType`][super::types::CurveType] was tried to be parsed from a `str`.
pub struct CurveTypeParseError(pub String);

#[derive(Debug, Error)]
/// Error used when there was a problem parsing one of the items from a `str` to another type.
#[error("There was a problem parsing one of the items from a `str` to another type")]
pub struct PipeVecParseErr {
    #[source]
    pub source: Box<dyn Error>,
    pub value: String,
}

#[derive(Debug, Error)]
#[error(transparent)]
pub struct HitSoundParseError(#[from] Box<dyn Error>);

impl From<ParseIntError> for HitSoundParseError {
    fn from(err: ParseIntError) -> Self {
        HitSoundParseError(Box::new(err))
    }
}

impl From<TryFromIntError> for HitSoundParseError {
    fn from(err: TryFromIntError) -> Self {
        HitSoundParseError(Box::new(err))
    }
}
