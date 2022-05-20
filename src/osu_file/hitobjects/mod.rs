pub mod error;
mod parser;
pub mod types;

use std::fmt::Display;
use std::str::FromStr;

use nom::character::streaming::char;
use nom::error::VerboseErrorKind;
use nom::Finish;
use rust_decimal::Decimal;
use strum_macros::Display;

use self::parser::hitobject;
use self::parser::HitObjectContext;
use self::types::*;
use super::Error;
use super::Integer;
use super::Position;
use super::Version;
use crate::helper::*;
use crate::parsers::comma_field;

pub use self::error::*;

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct HitObjects(pub Vec<HitObject>);

impl Version for HitObjects {
    type ParseError = Error<ParseError>;

    // TODO different versions
    fn from_str_v3(s: &str) -> std::result::Result<Option<Self>, Self::ParseError>
    where
        Self: Sized,
    {
        let mut hitobjects = Vec::new();

        for (line_index, s) in s.lines().enumerate() {
            hitobjects.push(Error::new_from_result_into(s.parse(), line_index)?);
        }

        Ok(Some(HitObjects(hitobjects)))
    }

    fn to_string_v3(&self) -> String {
        self.0
            .iter()
            .map(|h| h.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

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
        bit_flag |= self.combo_skip_count.get() << 4;

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
                let input_field = || {
                    comma_field::<nom::error::Error<_>>()(input)
                        .unwrap()
                        .1
                        .to_string()
                };

                let err = match HitObjectContext::from_str(context).unwrap() {
                    HitObjectContext::InvalidX
                    | HitObjectContext::InvalidY
                    | HitObjectContext::InvalidTime
                    | HitObjectContext::InvalidObjType
                    | HitObjectContext::InvalidEndTime => HitObjectParseError::ParseIntError(input_field()),
                    HitObjectContext::InvalidCurveType => {
                        HitObjectParseError::ParseCurveTypeError(input.to_string())
                    }
                    HitObjectContext::InvalidCurvePoints => {
                        HitObjectParseError::ParseCurvePointsError(input.to_string())
                    }
                    HitObjectContext::InvalidSlides => {
                        HitObjectParseError::ParseSlidesError(input.to_string())
                    }
                    HitObjectContext::InvalidLength => {
                        HitObjectParseError::ParseDecimalError(input.to_string())
                    }
                    HitObjectContext::InvalidHitsound => {
                        HitObjectParseError::ParseHitSoundError(input.to_string())
                    }
                    HitObjectContext::InvalidHitsample => {
                        HitObjectParseError::ParseHitsampleError(input.to_string())
                    }
                    HitObjectContext::InvalidEdgeSounds => {
                        HitObjectParseError::ParseEdgeSoundsError(input.to_string())
                    }
                    HitObjectContext::InvalidEdgeSets => {
                        HitObjectParseError::ParseEdgeSetsError(input.to_string())
                    }
                    HitObjectContext::MissingY => HitObjectParseError::MissingField(FieldName::Y),
                    HitObjectContext::MissingTime => HitObjectParseError::MissingField(FieldName::Time),
                    HitObjectContext::MissingObjType => {
                        HitObjectParseError::MissingField(FieldName::ObjType)
                    }
                    HitObjectContext::MissingCurveType => {
                        HitObjectParseError::MissingField(FieldName::CurveType)
                    }
                    HitObjectContext::MissingCurvePoints => {
                        HitObjectParseError::MissingField(FieldName::CurvePoints)
                    }
                    HitObjectContext::MissingSlides => HitObjectParseError::MissingField(FieldName::Slides),
                    HitObjectContext::MissingLength => HitObjectParseError::MissingField(FieldName::Length),
                    HitObjectContext::MissingEndTime => {
                        HitObjectParseError::MissingField(FieldName::EndTime)
                    }
                    HitObjectContext::MissingHitsound => {
                        HitObjectParseError::MissingField(FieldName::Hitsound)
                    }
                    HitObjectContext::MissingHitsample => {
                        HitObjectParseError::MissingField(FieldName::Hitsample)
                    }
                    HitObjectContext::MissingEdgeSounds => {
                        HitObjectParseError::MissingField(FieldName::EdgeSounds)
                    }
                    HitObjectContext::MissingEdgeSets => {
                        HitObjectParseError::MissingField(FieldName::EdgeSets)
                    }
                    HitObjectContext::MissingObjParams => HitObjectParseError::MissingObjParams,
                    HitObjectContext::UnknownObjType => HitObjectParseError::UnknownObjType,
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
