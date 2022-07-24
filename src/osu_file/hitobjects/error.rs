//! Module defining `error` types that's used for the `hitobject` related modules.

use std::num::ParseIntError;

use strum_macros::{EnumString, IntoStaticStr};
use thiserror::Error;

use crate::helper::macros::verbose_error_to_error;

#[derive(Debug, Error)]
#[error(transparent)]
pub struct ParseError(#[from] ParseHitObjectError);

#[derive(Debug, Error)]
#[error("Expected combo skip count to be 3 bits")]
pub struct ComboSkipCountTooHigh;

#[derive(Debug, Error, IntoStaticStr, EnumString)]
#[non_exhaustive]
/// Error used when there was a problem parsing a `str` into a `ColonSet`.
pub enum ParseColonSetError {
    /// When the first item failed to parse.
    #[error("Failed to parse the first item")]
    InvalidFirstItem,
    /// The colon separator is missing.
    #[error("The colon separator is missing")]
    MissingSeparator,
    #[error("Failed to parse the second item")]
    InvalidSecondItem,
}

verbose_error_to_error!(ParseColonSetError);

#[derive(Debug, Error, EnumString, IntoStaticStr)]
#[non_exhaustive]
/// Error used when there was a problem parsing a `str` into a [`HitObject`][super::HitObject].
pub enum ParseHitObjectError {
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

verbose_error_to_error!(ParseHitObjectError);

#[derive(Debug, Error, EnumString, IntoStaticStr)]
#[non_exhaustive]
/// Error used when there was a problem parsing a `str` into a [`hitsample`][super::types::HitSample].
pub enum ParseHitSampleError {
    #[error("Invalid `sample_set` value")]
    InvalidSampleSet,
    #[error("Invalid `index` value")]
    InvalidIndex,
    #[error("Invalid `volume` value")]
    InvalidVolume,
    #[error("Missing field separator")]
    MissingSeparator,
}

verbose_error_to_error!(ParseHitSampleError);

#[derive(Debug, Error)]
#[non_exhaustive]
/// Error used when there was a problem parsing a `str` into a [`sampleset`][super::types::SampleSet].
pub enum ParseSampleSetError {
    #[error("Invalid `SampleSet` variant")]
    UnknownVariant,
    /// There was a problem parsing a `str` as an integer first.
    #[error("There was a problem parsing the `str` into an integer first")]
    ParseValueError(#[from] ParseIntError),
}

#[derive(Debug, Error)]
#[non_exhaustive]
/// Error used when the user tried to set [`volume`][super::types::Volume]'s field as something invalid.
pub enum VolumeSetError {
    #[error("The volume was too high, expected to be in range 1 ~ 100")]
    VolumeTooHigh,
    /// The volume has a value `0`, which is "invalid".
    /// In the osu file documentation, the volume of 0 means the `timingpoint`'s volume is used instead.
    /// I handle that special case differently to make it more clear to the user what's going on.
    #[error("The volume was attempted to set to 0, expected to be in range 1 ~ 100")]
    VolumeTooLow,
}

#[derive(Debug, Error)]
#[non_exhaustive]
/// Error used when there was a problem parsing a `volume` from a `str`.
pub enum ParseVolumeError {
    #[error(transparent)]
    VolumeSetError(#[from] VolumeSetError),
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ParseCurveTypeError {
    #[error("Unknown `CurveType` variant")]
    UnknownVariant,
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ParseHitSoundError {
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
}
