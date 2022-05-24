pub mod error;
pub mod types;

use std::fmt::Display;
use std::str::FromStr;

use nom::bytes::complete::is_not;
use nom::bytes::complete::take_until;
use nom::character::streaming::char;
use nom::combinator::map_res;
use nom::error::context;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::Parser;
use rust_decimal::Decimal;

use self::types::*;
use super::Error;
use super::Integer;
use super::Position;
use super::Version;
use crate::helper::*;
use crate::parsers::*;

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

/*pub enum HitObjectParseError {
    InvalidX,
    InvalidY,
    InvalidTime,
    InvalidObjType,
    InvalidCurveType,
    InvalidCurvePoints,
    InvalidSlides,
    InvalidLength,
    InvalidEndTime,
    InvalidHitsound,
    InvalidHitsample,
    InvalidEdgeSounds,
    InvalidEdgeSets,
    MissingY,
    MissingTime,
    MissingObjType,
    MissingCurveType,
    MissingCurvePoints,
    MissingSlides,
    MissingLength,
    MissingEndTime,
    MissingHitsound,
    MissingHitsample,
    MissingEdgeSounds,
    MissingEdgeSets,
    MissingObjParams,
    UnknownObjType,
} */

impl FromStr for HitObject {
    type Err = HitObjectParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hitsound = context(
            HitObjectParseError::InvalidHitSound.into(),
            // TODO replace all map_res(is_not(",")) with comma_field_type
            comma_field_type(),
        );
        let hitsample = context(
            HitObjectParseError::InvalidHitSample.into(),
            consume_rest_type(),
        );

        let (s, (position, time, obj_type, hitsound)) = tuple((
            tuple((
                context(HitObjectParseError::InvalidX.into(), comma_field_type()),
                preceded(
                    context(HitObjectParseError::MissingY.into(), comma()),
                    context(HitObjectParseError::InvalidY.into(), comma_field_type()),
                ),
            ))
            .map(|(x, y)| (Position { x, y })),
            preceded(
                context(HitObjectParseError::MissingTime.into(), comma()),
                context(HitObjectParseError::InvalidTime.into(), comma_field_type()),
            ),
            preceded(
                context(HitObjectParseError::MissingObjType.into(), comma()),
                context(
                    HitObjectParseError::InvalidObjType.into(),
                    comma_field_type::<_, Integer>(),
                ),
            ),
            preceded(
                context(HitObjectParseError::MissingHitSound.into(), comma()),
                hitsound,
            ),
        ))(s)?;

        let new_combo = nth_bit_state_i64(obj_type as i64, 2);
        let combo_skip_count = ComboSkipCount::try_from((obj_type >> 4 & 0b111) as u8).unwrap();

        let hitobject = if nth_bit_state_i64(obj_type as i64, 0) {
            let (_, hitsample) = preceded(
                context(HitObjectParseError::MissingHitSample.into(), comma()),
                hitsample,
            )(s)?;

            // hitcircle
            HitObject {
                position,
                time,
                obj_params: HitObjectParams::HitCircle,
                new_combo,
                combo_skip_count,
                hitsound,
                hitsample,
            }
        } else if nth_bit_state_i64(obj_type as i64, 1) {
            // slider
            let pipe = char('|');
            let curve_type = context(
                HitObjectParseError::InvalidCurveType.into(),
                map_res(is_not("|"), |f: &str| f.parse()),
            );
            let curve_points = context(
                HitObjectParseError::InvalidCurvePoint.into(),
                pipe_vec(|s: &str| s.parse()),
            );
            let edge_sounds = context(
                HitObjectParseError::InvalidEdgeSound.into(),
                pipe_vec(|s: &str| s.parse()),
            );
            let edge_sets = context(
                HitObjectParseError::InvalidEdgeSet.into(),
                pipe_vec(|s: &str| s.parse()),
            );

            let (_, (curve_type, curve_points, slides, length, edge_sounds, edge_sets, hitsample)) =
                tuple((
                    preceded(
                        context(HitObjectParseError::MissingCurveType.into(), comma()),
                        curve_type,
                    ),
                    preceded(
                        context(HitObjectParseError::MissingCurvePoint.into(), pipe),
                        curve_points,
                    ),
                    context(
                        HitObjectParseError::InvalidSlidesCount.into(),
                        comma_field_type(),
                    ),
                    preceded(
                        context(HitObjectParseError::MissingLength.into(), comma()),
                        context(
                            HitObjectParseError::InvalidLength.into(),
                            comma_field_type(),
                        ),
                    ),
                    preceded(
                        context(HitObjectParseError::MissingEdgeSound.into(), comma()),
                        edge_sounds,
                    ),
                    edge_sets,
                    hitsample,
                ))(s)?;

            HitObject {
                position,
                time,
                obj_params: HitObjectParams::Slider {
                    curve_type,
                    curve_points,
                    slides,
                    length,
                    edge_sounds,
                    edge_sets,
                },
                new_combo,
                combo_skip_count,
                hitsound,
                hitsample,
            }
        } else if nth_bit_state_i64(obj_type as i64, 3) {
            // spinner
            let (_, (end_time, hitsample)) = tuple((
                preceded(
                    context(HitObjectParseError::MissingEndTime.into(), comma()),
                    context(
                        HitObjectParseError::InvalidEndTime.into(),
                        comma_field_type(),
                    ),
                ),
                preceded(
                    context(HitObjectParseError::MissingHitSample.into(), comma()),
                    hitsample,
                ),
            ))(s)?;

            HitObject {
                position,
                time,
                obj_params: HitObjectParams::Spinner { end_time },
                new_combo,
                combo_skip_count,
                hitsound,
                hitsample,
            }
        } else if nth_bit_state_i64(obj_type as i64, 7) {
            // osu!mania hold
            // ppy has done it once again
            let end_time = context(
                HitObjectParseError::InvalidEndTime.into(),
                map_res(take_until(":"), |s: &str| s.parse()),
            );
            let (_, (end_time, _, hitsample)) = tuple((
                preceded(
                    context(HitObjectParseError::MissingEndTime.into(), comma()),
                    end_time,
                ),
                context(HitObjectParseError::MissingHitSample.into(), char(':')),
                hitsample,
            ))(s)?;

            HitObject {
                position,
                time,
                obj_params: HitObjectParams::OsuManiaHold { end_time },
                new_combo,
                combo_skip_count,
                hitsound,
                hitsample,
            }
        } else {
            return Err(HitObjectParseError::UnknownObjType);
        };

        Ok(hitobject)
    }
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
