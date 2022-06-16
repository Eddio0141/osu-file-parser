pub mod error;
pub mod types;

use nom::branch::alt;
use nom::bytes::complete::*;
use nom::character::streaming::char;
use nom::combinator::*;
use nom::error::context;
use nom::sequence::*;
use nom::*;
use rust_decimal::Decimal;

use self::types::*;
use crate::helper::trait_ext::MapStringNewLineVersion;
use crate::helper::*;
use crate::parsers::*;

pub use self::error::*;

use super::Error;
use super::Integer;
use super::Position;
use super::VersionedDefault;
use super::VersionedFromString;
use super::VersionedToString;

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct HitObjects(pub Vec<HitObject>);

impl VersionedFromString for HitObjects {
    type ParseError = Error<ParseError>;

    // TODO different versions
    fn from_str(s: &str, version: usize) -> std::result::Result<Option<Self>, Self::ParseError> {
        let mut hitobjects = Vec::new();

        for (line_index, s) in s.lines().enumerate() {
            hitobjects.push(Error::new_from_result_into(
                HitObject::from_str(s, version).map(|v| v.unwrap()),
                line_index,
            )?);
        }

        Ok(Some(HitObjects(hitobjects)))
    }
}

impl VersionedToString for HitObjects {
    fn to_string(&self, version: usize) -> Option<String> {
        self.0.iter().map_string_new_line(version)
    }
}

impl VersionedDefault for HitObjects {
    fn default(_: usize) -> Option<Self> {
        Some(HitObjects(Vec::new()))
    }
}

/// A struct that represents a hitobject.
///
/// All hitobjects will have the properties: `x`, `y`, `time`, `type`, `hitsound`, `hitsample`.
///
/// The `type` property is a `u8` integer with each bit flags containing some information, which are split into the functions and enums:
/// [hitobject_type][Self::obj_params], [new_combo][Self::new_combo], [combo_skip_count][Self::combo_skip_count]
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
    pub hitsample: Option<HitSample>,
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

const OLD_VERSION_TIME_OFFSET: Integer = 24;

impl VersionedFromString for HitObject {
    type ParseError = HitObjectParseError;

    fn from_str(s: &str, version: usize) -> std::result::Result<Option<Self>, Self::ParseError> {
        let hitsound = context(
            HitObjectParseError::InvalidHitSound.into(),
            comma_field_type(),
        );
        let mut hitsample = alt((
            nothing().map(|_| None),
            preceded(
                context(HitObjectParseError::MissingHitSample.into(), comma()),
                context(
                    HitObjectParseError::InvalidHitSample.into(),
                    map_res(rest, |s| {
                        HitSample::from_str(s, version).map(|v| v.unwrap())
                    }),
                ),
            )
            .map(Some),
        ));

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
            )
            // version 3 has a slight time delay of 24ms
            .map(|t| {
                if (3..=4).contains(&version) {
                    t + OLD_VERSION_TIME_OFFSET
                } else {
                    t
                }
            }),
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
            let (_, hitsample) = hitsample(s)?;

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

            let (
                _,
                (
                    curve_type,
                    curve_points,
                    slides,
                    length,
                    (
                        edge_sounds,
                        edge_sets,
                        hitsample,
                        edge_sounds_short_hand,
                        edge_sets_shorthand,
                    ),
                ),
            ) = tuple((
                preceded(
                    context(HitObjectParseError::MissingCurveType.into(), comma()),
                    context(
                        HitObjectParseError::InvalidCurveType.into(),
                        map_res(is_not("|"), |f: &str| f.parse()),
                    ),
                ),
                preceded(
                    context(HitObjectParseError::MissingCurvePoint.into(), pipe),
                    context(
                        HitObjectParseError::InvalidCurvePoint.into(),
                        pipe_vec_map().map(|mut v| {
                            if version == 3 && !v.is_empty() {
                                v.remove(0);
                            }
                            v
                        }),
                    ),
                ),
                preceded(
                    context(HitObjectParseError::MissingSlidesCount.into(), comma()),
                    context(
                        HitObjectParseError::InvalidSlidesCount.into(),
                        comma_field_type(),
                    ),
                ),
                preceded(
                    context(HitObjectParseError::MissingLength.into(), comma()),
                    context(
                        HitObjectParseError::InvalidLength.into(),
                        comma_field_type(),
                    ),
                ),
                alt((
                    nothing().map(|_| (Vec::new(), Vec::new(), None, true, true)),
                    tuple((
                        preceded(
                            context(HitObjectParseError::MissingEdgeSound.into(), comma()),
                            context(HitObjectParseError::InvalidEdgeSound.into(), pipe_vec_map()),
                        ),
                        alt((
                            nothing().map(|_| (Vec::new(), None, true)),
                            tuple((
                                preceded(
                                    context(HitObjectParseError::MissingEdgeSet.into(), comma()),
                                    context(
                                        HitObjectParseError::InvalidEdgeSet.into(),
                                        pipe_vec_map(),
                                    ),
                                ),
                                hitsample,
                            ))
                            .map(|(edge_sets, hitsample)| (edge_sets, hitsample, false)),
                        )),
                    ))
                    .map(
                        |(edge_sounds, (edge_sets, hitsample, edge_sets_shorthand))| {
                            (
                                edge_sounds,
                                edge_sets,
                                hitsample,
                                false,
                                edge_sets_shorthand,
                            )
                        },
                    ),
                )),
            ))(s)?;

            HitObject {
                position,
                time,
                obj_params: HitObjectParams::Slider(SlideParams {
                    curve_type,
                    curve_points,
                    slides,
                    length,
                    edge_sounds,
                    edge_sets,
                    edge_sets_shorthand,
                    edge_sounds_short_hand,
                }),
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
                )
                .map(|t| {
                    if (3..=4).contains(&version) {
                        t + OLD_VERSION_TIME_OFFSET
                    } else {
                        t
                    }
                }),
                hitsample,
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
            let hitsample = alt((
                nothing().map(|_| None),
                preceded(
                    context(HitObjectParseError::MissingHitSample.into(), char(':')),
                    context(
                        HitObjectParseError::InvalidHitSample.into(),
                        map_res(rest, |s| {
                            HitSample::from_str(s, version).map(|v| v.unwrap())
                        }),
                    ),
                )
                .map(Some),
            ));
            let end_time = context(
                HitObjectParseError::InvalidEndTime.into(),
                map_res(take_until(":"), |s: &str| s.parse()),
            )
            .map(|v| {
                if version == 3 {
                    v + OLD_VERSION_TIME_OFFSET
                } else {
                    v
                }
            });
            let (_, (end_time, hitsample)) = tuple((
                preceded(
                    context(HitObjectParseError::MissingEndTime.into(), comma()),
                    end_time,
                ),
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

        Ok(Some(hitobject))
    }
}

impl VersionedToString for &HitObject {
    fn to_string(&self, version: usize) -> Option<String> {
        self.to_owned().to_string(version)
    }
}

impl VersionedToString for HitObject {
    fn to_string(&self, version: usize) -> Option<String> {
        let mut properties: Vec<String> = vec![
            self.position.x.to_string(),
            self.position.y.to_string(),
            if (3..=4).contains(&version) {
                self.time - OLD_VERSION_TIME_OFFSET
            } else {
                self.time
            }
            .to_string(),
            self.type_to_string(),
            self.hitsound.to_string(),
        ];

        match &self.obj_params {
            HitObjectParams::HitCircle => (),
            HitObjectParams::Slider(SlideParams {
                curve_type,
                curve_points,
                slides,
                length,
                edge_sounds,
                edge_sets,
                edge_sounds_short_hand,
                edge_sets_shorthand,
            }) => {
                properties.push(curve_type.to_string());

                let mut properties_2 = vec![
                    {
                        if version == 3 {
                            let mut curve_points = curve_points.clone();
                            curve_points.insert(0, CurvePoint(self.position));
                            pipe_vec_to_string(&curve_points)
                        } else {
                            pipe_vec_to_string(curve_points)
                        }
                    },
                    slides.to_string(),
                    length.to_string(),
                ];

                if !edge_sounds.is_empty()
                    || !*edge_sounds_short_hand
                    || !edge_sets.is_empty()
                    || !*edge_sets_shorthand
                    || self.hitsample.is_some()
                {
                    properties_2.push(pipe_vec_to_string(edge_sounds));
                }
                if !edge_sets.is_empty() || !*edge_sets_shorthand || self.hitsample.is_some() {
                    properties_2.push(pipe_vec_to_string(edge_sets));
                }
                if let Some(hitsample) = &self.hitsample {
                    if let Some(hitsample) = hitsample.to_string(version) {
                        properties_2.push(hitsample);
                    }
                }

                return Some(format!(
                    "{}|{}",
                    properties.join(","),
                    properties_2.join(",")
                ));
            }
            HitObjectParams::Spinner { end_time } => properties.push(
                if (3..=4).contains(&version) {
                    end_time - OLD_VERSION_TIME_OFFSET
                } else {
                    *end_time
                }
                .to_string(),
            ),
            HitObjectParams::OsuManiaHold { end_time } => {
                properties.push(
                    if (3..=4).contains(&version) {
                        end_time - OLD_VERSION_TIME_OFFSET
                    } else {
                        *end_time
                    }
                    .to_string(),
                );

                let hitsample = if let Some(hitsample) = &self.hitsample {
                    if let Some(hitsample) = hitsample.to_string(version) {
                        hitsample
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                };

                return Some(format!("{}:{hitsample}", properties.join(",")));
            }
        }

        if let Some(hitsample) = &self.hitsample {
            if let Some(hitsample) = hitsample.to_string(version) {
                properties.push(hitsample);
            }
        }

        let s = properties.join(",");

        // v3 for some reason has a trailing comma for hitcircles
        let s = if version == 3 && matches!(self.obj_params, HitObjectParams::HitCircle) {
            format!("{s},")
        } else {
            s
        };

        Some(s)
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[non_exhaustive]
pub enum HitObjectParams {
    HitCircle,
    Slider(SlideParams),
    Spinner { end_time: Integer },
    OsuManiaHold { end_time: Integer },
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct SlideParams {
    pub curve_type: CurveType,
    pub curve_points: Vec<CurvePoint>,
    pub slides: Integer,
    pub length: Decimal,
    pub edge_sounds: Vec<HitSound>,
    edge_sounds_short_hand: bool,
    pub edge_sets: Vec<EdgeSet>,
    edge_sets_shorthand: bool,
}

impl SlideParams {
    pub fn new(
        curve_type: CurveType,
        curve_points: Vec<CurvePoint>,
        slides: Integer,
        length: Decimal,
        edge_sounds: Vec<HitSound>,
        edge_sets: Vec<EdgeSet>,
    ) -> Self {
        Self {
            curve_type,
            curve_points,
            slides,
            length,
            edge_sounds,
            edge_sets,
            edge_sets_shorthand: true,
            edge_sounds_short_hand: true,
        }
    }
}
