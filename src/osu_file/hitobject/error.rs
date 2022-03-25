//! Module defining `error` types that's used for the `hitobject` related modules.

use std::{
    error::Error,
    num::{ParseIntError, TryFromIntError},
};

use strum::ParseError;
use thiserror::Error;

use super::ComboSkipCount;

#[derive(Debug, Error)]
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

#[derive(Debug, Error)]
/// Error used when there was a problem parsing a `str` into a [`hitobject`][super::HitObjectWrapper].
pub enum HitObjectParseError {
    #[error("The hitobject is missing the `x` coordinate")]
    MissingX,
    #[error("The hitobject failed to parse the `x` coordinate from {value}")]
    XParseError {
        #[source]
        source: ParseIntError,
        value: String,
    },
    #[error("The hitobject is missing the `y` coordinate")]
    MissingY,
    #[error("The hitobject failed to parse the `y` coordinate from {value}")]
    YParseError {
        #[source]
        source: ParseIntError,
        value: String,
    },
    #[error("The hitobject is missing the `time`")]
    MissingTime,
    #[error("The hitobject failed to parse the `time` from {value}")]
    TimeParseError {
        #[source]
        source: ParseIntError,
        value: String,
    },
    #[error("The hitobject is missing the `obj_type`")]
    MissingObjType,
    #[error("The hitobject failed to parse the `obj_type` from {value}")]
    ObjTypeParseError {
        #[source]
        source: ParseIntError,
        value: String,
    },
    #[error("The hitobject is missing the `hit_sound`")]
    MissingHitSound,
    #[error("The hitobject failed to parse the `hit_sound` from {value}")]
    HitSoundParseError {
        #[source]
        source: HitSoundParseError,
        value: String,
    },
    #[error("The hitobject is missing the `curve_type`")]
    MissingCurveType,
    #[error("The hitobject failed to parse the `curve_type` from {value}")]
    CurveTypeParseError {
        #[source]
        source: ParseError,
        value: String,
    },
    #[error("The hitobject is missing the `curve_points`")]
    MissingCurvePoints,
    #[error("The hitobject failed to parse the `curve_points` from {value}")]
    CurvePointsParseError {
        #[source]
        source: PipeVecParseErr,
        value: String,
    },
    #[error("The hitobject is missing the `hit_sample`")]
    MissingHitsample,
    #[error("The hitobject failed to parse the `hit_sample` from {value}")]
    HitsampleParseError {
        #[source]
        source: HitSampleParseError,
        value: String,
    },
    #[error("The hitobject is missing the `end_time`")]
    MissingEndTime,
    #[error("The hitobject failed to parse the `end_time` from {value}")]
    EndTimeParseError {
        #[source]
        source: ParseIntError,
        value: String,
    },
    #[error("The hitobject is missing the `slides`")]
    MissingSlides,
    #[error("The hitobject failed to parse the `slides` from {value}")]
    SlidesParseError {
        #[source]
        source: ParseIntError,
        value: String,
    },
    #[error("The hitobject is missing the `length`")]
    MissingLength,
    #[error("The hitobject failed to parse the `length` from {value}")]
    LengthParseError {
        #[source]
        source: rust_decimal::Error,
        value: String,
    },
    #[error("The hitobject is missing the `edge_sounds`")]
    MissingEdgeSounds,
    #[error("The hitobject failed to parse the `edge_sounds` from {value}")]
    EdgeSoundsParseError {
        #[source]
        source: PipeVecParseErr,
        value: String,
    },
    #[error("The hitobject is missing the `edge_sets`")]
    MissingEdgeSets,
    #[error("The hitobject failed to parse the `edge_sets` from {value}")]
    EdgeSetsParseError {
        #[source]
        source: PipeVecParseErr,
        value: String,
    },
    #[error("Too many parameters were provided for the hitobject")]
    TooManyParameters,
    #[error("Unknown object type")]
    UnknownObjType,
}

#[derive(Debug, Error)]
/// Error used when there was a problem parsing a `str` into a [`hitsample`][super::types::HitSample].
pub enum HitSampleParseError {
    /// A property is missing.
    #[error("Property for the index {0} of the hitsample is missing")]
    MissingProperty(usize),
    /// There was a problem parsing the `str` into some type.
    #[error("There was a problem parsing the `str` \"{value}\" to a property type")]
    ParseError {
        #[source]
        source: Box<dyn Error>,
        value: String,
    },
}

#[derive(Debug, Error)]
/// Error used when there was a problem parsing a `str` into a [`sampleset`][super::types::SampleSet].
pub enum SampleSetParseError {
    /// The `str` had a value higher than 3.
    #[error("Expected value in between `0` ~ `3`, got {0}")]
    ValueHigherThanThree(i32),
    /// There was a problem parsing a `str` as an integer first.
    #[error("There was a problem parsing the `str` into an integer first")]
    ValueParseError(#[from] ParseIntError),
}

#[derive(Debug, Error)]
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

#[derive(Debug, Error)]
#[error("Attempted to set the combo skip count but the value was higher than the 3-bit limit: `7`, got {0}")]
/// Error used when the `ComboSkipCount` was tried to set higher than the limit of 3 bits: `7`.
pub struct ComboSkipCountSetError(pub ComboSkipCount);
