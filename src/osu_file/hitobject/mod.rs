pub mod error;
mod hitobject_parser;
pub mod types;

use std::fmt::Display;
use std::str::FromStr;

use nom::character::streaming::char;
use nom::error::VerboseErrorKind;
use nom::Finish;
use rust_decimal::Decimal;
use strum_macros::Display;
use thiserror::Error;

use self::error::*;
use self::hitobject_parser::hitobject;
use self::hitobject_parser::Context;
use self::types::*;
use super::Integer;
use super::Position;
use crate::helper::*;

type ComboSkipCount = u8;

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct HitObjects(pub Vec<HitObject>);

impl Display for HitObjects {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|h| h.to_string())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl FromStr for HitObjects {
    type Err = HitObjectsParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut hitobjects = Vec::new();

        for s in s.lines() {
            hitobjects.push(s.parse()?);
        }

        Ok(HitObjects(hitobjects))
    }
}

#[derive(Debug, Error)]
#[error(transparent)]
pub struct HitObjectsParseError(#[from] HitObjectParseError);

/// An interface that represents a hitobject.
///
/// All hitobjects will have the properties: `x`, `y`, `time`, `type`, `hitsound`, `hitsample`.
///
/// The `type` property is a `u8` integer with each bit flags containing some information, which are split into the functions:
/// [hitobject_type][Self::obj_type], [new_combo][Self::new_combo], [combo_skip_count][Self::combo_skip_count]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[non_exhaustive]
pub struct HitObject {
    /// The position of the hitobject.
    pub position: Position,

    /// The time when the object is to be hit, in milliseconds from the beginning of the beatmap's audio.
    pub time: Integer,

    /// The hitobject parameters.
    /// Each hitobject contains different parameters.
    /// Also is used to know which hitobject type this is.
    pub obj_params: HitObjectParams,

    /// If the hitobject is a new combo.
    pub new_combo: bool,

    /// A 3-bit integer specifying how many combo colours to skip, if this object starts a new combo.
    // TODO limit this to 3 bits input
    pub combo_skip_count: ComboSkipCount,

    /// The [hitsound][HitSound] property of the hitobject.
    pub hitsound: HitSound,

    /// The [hitsample][HitSample] property of the hitobject.
    pub hitsample: HitSample,
}

impl HitObject {
    fn type_to_string(&self) -> String {
        let mut bit_flag: u8 = 0;

        bit_flag |= match self.obj_params {
            HitObjectParams::HitCircle => 1,
            HitObjectParams::Slider { .. } => 2,
            HitObjectParams::Spinner { .. } => 8,
            HitObjectParams::OsuManiaHold { .. } => 128,
        };

        if self.new_combo {
            bit_flag |= 4;
        }

        // 3 bit value from 4th ~ 6th bits
        bit_flag |= self.combo_skip_count << 4;

        bit_flag.to_string()
    }

    pub fn hitcircle_default() -> Self {
        Self {
            position: Default::default(),
            time: Default::default(),
            obj_params: HitObjectParams::HitCircle,
            new_combo: Default::default(),
            combo_skip_count: Default::default(),
            hitsound: Default::default(),
            hitsample: Default::default(),
        }
    }

    pub fn spinner_default() -> Self {
        Self {
            position: Default::default(),
            time: Default::default(),
            obj_params: HitObjectParams::Spinner {
                end_time: Default::default(),
            },
            new_combo: Default::default(),
            combo_skip_count: Default::default(),
            hitsound: Default::default(),
            hitsample: Default::default(),
        }
    }

    pub fn osu_mania_hold_default() -> Self {
        Self {
            position: Position {
                x: 0,
                ..Default::default()
            },
            time: Default::default(),
            obj_params: HitObjectParams::OsuManiaHold {
                end_time: Default::default(),
            },
            new_combo: Default::default(),
            combo_skip_count: Default::default(),
            hitsound: Default::default(),
            hitsample: Default::default(),
        }
    }
}

impl FromStr for HitObject {
    type Err = HitObjectParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match hitobject(s).finish() {
            Ok((_, hitobject)) => Ok(hitobject),
            Err(err) => {
                let mut context = None;
                let mut input = None;

                for err in &err.errors {
                    input = Some(err.0);

                    if let VerboseErrorKind::Context(c) = &err.1 {
                        context = Some(c);
                        break;
                    }
                }

                let context = context.unwrap();
                let input = input.unwrap();

                let err = match Context::from_str(context).unwrap() {
                    Context::InvalidX
                    | Context::InvalidY
                    | Context::InvalidTime
                    | Context::InvalidObjType
                    | Context::InvalidEndTime => {
                        HitObjectParseError::ParseIntError(input.to_string())
                    }
                    Context::InvalidCurveType => {
                        HitObjectParseError::ParseCurveTypeError(input.to_string())
                    }
                    Context::InvalidCurvePoints => {
                        HitObjectParseError::ParseCurvePointsError(input.to_string())
                    }
                    Context::InvalidSlides => {
                        HitObjectParseError::ParseSlidesError(input.to_string())
                    }
                    Context::InvalidLength => {
                        HitObjectParseError::ParseDecimalError(input.to_string())
                    }
                    Context::InvalidHitsound => {
                        HitObjectParseError::ParseHitSoundError(input.to_string())
                    }
                    Context::InvalidHitsample => {
                        HitObjectParseError::ParseHitsampleError(input.to_string())
                    }
                    Context::InvalidEdgeSounds => {
                        HitObjectParseError::ParseEdgeSoundsError(input.to_string())
                    }
                    Context::InvalidEdgeSets => {
                        HitObjectParseError::ParseEdgeSetsError(input.to_string())
                    }
                    Context::MissingY => HitObjectParseError::MissingField(FieldName::Y),
                    Context::MissingTime => HitObjectParseError::MissingField(FieldName::Time),
                    Context::MissingObjType => {
                        HitObjectParseError::MissingField(FieldName::ObjType)
                    }
                    Context::MissingCurveType => {
                        HitObjectParseError::MissingField(FieldName::CurveType)
                    }
                    Context::MissingCurvePoints => {
                        HitObjectParseError::MissingField(FieldName::CurvePoints)
                    }
                    Context::MissingSlides => HitObjectParseError::MissingField(FieldName::Slides),
                    Context::MissingLength => HitObjectParseError::MissingField(FieldName::Length),
                    Context::MissingEndTime => {
                        HitObjectParseError::MissingField(FieldName::EndTime)
                    }
                    Context::MissingHitsound => {
                        HitObjectParseError::MissingField(FieldName::Hitsound)
                    }
                    Context::MissingHitsample => {
                        HitObjectParseError::MissingField(FieldName::Hitsample)
                    }
                    Context::MissingEdgeSounds => {
                        HitObjectParseError::MissingField(FieldName::EdgeSounds)
                    }
                    Context::MissingEdgeSets => {
                        HitObjectParseError::MissingField(FieldName::EdgeSets)
                    }
                    Context::MissingObjParams => HitObjectParseError::MissingObjParams,
                    Context::UnknownObjType => HitObjectParseError::UnknownObjType,
                };

                Err(err)
            }
        }
    }
}

#[derive(Debug, Display)]
pub enum FieldName {
    X,
    Y,
    Time,
    ObjType,
    Hitsound,
    Hitsample,
    Length,
    EndTime,
    Slides,
    CurvePoints,
    EdgeSounds,
    EdgeSets,
    CurveType,
}

impl Display for HitObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut properties: Vec<String> = vec![
            self.position.x.to_string(),
            self.position.y.to_string(),
            self.time.to_string(),
            self.type_to_string(),
            self.hitsound.to_string(),
        ];

        match &self.obj_params {
            HitObjectParams::HitCircle => (),
            HitObjectParams::Slider {
                curve_type,
                curve_points,
                slides,
                length,
                edge_sounds,
                edge_sets,
            } => {
                properties.push(curve_type.to_string());

                let properties_2 = vec![
                    pipe_vec_to_string(curve_points),
                    slides.to_string(),
                    length.to_string(),
                    pipe_vec_to_string(edge_sounds),
                    pipe_vec_to_string(edge_sets),
                    self.hitsample.to_string(),
                ];

                return write!(f, "{}|{}", properties.join(","), properties_2.join(","));
            }
            HitObjectParams::Spinner { end_time } => properties.push(end_time.to_string()),
            HitObjectParams::OsuManiaHold { end_time } => {
                properties.push(end_time.to_string());

                return write!(f, "{}:{}", properties.join(","), self.hitsample);
            }
        }

        properties.push(self.hitsample.to_string());

        write!(f, "{}", properties.join(","))
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[non_exhaustive]
pub enum HitObjectParams {
    HitCircle,
    Slider {
        curve_type: CurveType,
        curve_points: Vec<CurvePoint>,
        slides: Integer,
        length: Decimal,
        edge_sounds: Vec<HitSound>,
        edge_sets: Vec<EdgeSet>,
    },
    Spinner {
        end_time: Integer,
    },
    OsuManiaHold {
        end_time: Integer,
    },
}
