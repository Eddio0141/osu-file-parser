pub mod error;
pub mod types;

use std::fmt::Display;
use std::str::FromStr;

use nom::bytes::complete::is_not;
use nom::bytes::complete::take_till;
use nom::character::streaming::char;
use nom::combinator::map_res;
use nom::sequence::tuple;
use rust_decimal::Decimal;
use thiserror::Error;

use self::error::*;
use self::types::*;
use super::helper::*;

use super::parsers::pipe_vec;
use super::Integer;
use super::Position;

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

// TODO re-write this
/// An interface that represents a hitobject.
///
/// All hitobjects will have the properties: `x`, `y`, `time`, `type`, `hitsound`, `hitsample`.
///
/// The `type` property is a `u8` integer with each bit flags containing some information, which are split into the functions:
/// [hitobject_type][Self::obj_type], [new_combo][Self::new_combo], [combo_skip_count][Self::combo_skip_count]
/// TODO unmerge this
/// Attempts to parse a `&str` into a [HitObjectWrapper].
///
/// # Example
/// ```
/// use osu_file_parser::osu_file::hitobject::try_parse_hitobject;
///
/// let hitcircle_str = "221,350,9780,1,0,0:0:0:0:";
/// let slider_str = "31,85,3049,2,0,B|129:55|123:136|228:86,1,172.51,2|0,3:2|0:2,0:2:0:0:";
/// let spinner_str = "256,192,33598,12,0,431279,0:0:0:0:";
/// let osu_mania_hold_str = "51,192,350,128,2,849:0:0:0:0:";
///
/// let hitcircle = try_parse_hitobject(hitcircle_str).unwrap();
/// let slider = try_parse_hitobject(slider_str).unwrap();
/// let spinner = try_parse_hitobject(spinner_str).unwrap();
/// let osu_mania_hold = try_parse_hitobject(osu_mania_hold_str).unwrap();
///
/// assert_eq!(hitcircle_str, hitcircle.to_string());
/// assert_eq!(slider_str, slider.to_string());
/// assert_eq!(spinner_str, spinner.to_string());
/// assert_eq!(osu_mania_hold_str, osu_mania_hold.to_string());
/// ```
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
        // TODO error handling
        let comma = || char::<_, nom::error::Error<_>>(',');
        let int = || map_res(is_not(","), |f: &str| f.parse::<Integer>());
        let hitsound = map_res(is_not(","), |f: &str| f.parse::<HitSound>());
        let mut hitsample = map_res(take_till(|_| false), |f: &str| f.parse::<HitSample>());

        let (s, (x, _, y, _, time, _, obj_type, _, hitsound, _)) = tuple((
            int(),
            comma(),
            int(),
            comma(),
            int(),
            comma(),
            int(),
            comma(),
            hitsound,
            comma(),
        ))(s)
        .unwrap();

        let position = Position { x, y };

        let new_combo = nth_bit_state_i64(obj_type as i64, 2);
        let combo_skip_count = (obj_type >> 4 & 0b111) as u8;

        if nth_bit_state_i64(obj_type as i64, 0) {
            let (s, hitsample) = hitsample(s).unwrap();

            // hitcircle
            Ok(Self {
                position,
                time,
                obj_params: HitObjectParams::HitCircle,
                new_combo,
                combo_skip_count,
                hitsound,
                hitsample,
            })
        } else if nth_bit_state_i64(obj_type as i64, 1) {
            // slider
            let pipe = char('|');
            let curve_type = map_res(is_not("|"), |f: &str| f.parse::<CurveType>());

            let decimal = map_res(is_not(","), |f: &str| f.parse::<Decimal>());
            let curve_points = pipe_vec(|s: &str| s.parse::<CurvePoint>());
            let edge_sounds = pipe_vec(|s: &str| s.parse::<HitSound>());
            let edge_sets = pipe_vec(|s: &str| s.parse::<EdgeSet>());

            // TODO error handling
            let (
                s,
                (
                    curve_type,
                    _,
                    curve_points,
                    _,
                    slides,
                    _,
                    length,
                    _,
                    edge_sounds,
                    _,
                    edge_sets,
                    _,
                    hitsample,
                ),
            ) = tuple((
                curve_type,
                pipe,
                curve_points,
                comma(),
                int(),
                comma(),
                decimal,
                comma(),
                edge_sounds,
                comma(),
                edge_sets,
                comma(),
                hitsample,
            ))(s)
            .unwrap();

            Ok(Self {
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
            })
        } else if nth_bit_state_i64(obj_type as i64, 3) {
            // spinner
            // TODO error handling
            let (s, (end_time, _, hitsample)) = tuple((int(), comma(), hitsample))(s).unwrap();

            Ok(Self {
                position,
                time,
                obj_params: HitObjectParams::Spinner { end_time },
                new_combo,
                combo_skip_count,
                hitsound,
                hitsample,
            })
        } else if nth_bit_state_i64(obj_type as i64, 7) {
            // osu!mania hold
            // TODO error handling
            // ppy has done it once again
            let (s, (end_time, _, hitsample)) = tuple((int(), char('|'), hitsample))(s).unwrap();

            Ok(Self {
                position,
                time,
                obj_params: HitObjectParams::OsuManiaHold { end_time },
                new_combo,
                combo_skip_count,
                hitsound,
                hitsample,
            })
        } else {
            // osu file format didn't specify what to do with no bit flags set
            Err(HitObjectParseError::UnknownObjType)
        }

        /*
        // object properties split by ,
        let obj_type = obj_properties
            .next()
            .ok_or(HitObjectParseError::MissingObjType)?;
        let obj_type =
            obj_type
                .parse::<Integer>()
                .map_err(|err| HitObjectParseError::ObjTypeParseError {
                    source: err,
                    value: obj_type.to_string(),
                })?;
        let hitsound = obj_properties
            .next()
            .ok_or(HitObjectParseError::MissingHitSound)?;
        let hitsound = hitsound
            .parse()
            .map_err(|err| HitObjectParseError::HitSoundParseError {
                source: err,
                value: hitsound.to_string(),
            })?;

        let position = Position { x, y };

        let new_combo = nth_bit_state_i64(obj_type as i64, 2);
        let combo_skip_count = (obj_type >> 4 & 0b111) as u8;

        let hitsample = |s: &mut Split<&str>| {
            let hitsample = s.next().ok_or(HitObjectParseError::MissingHitsample)?;

            hitsample
                .parse()
                .map_err(|err| HitObjectParseError::HitsampleParseError {
                    source: err,
                    value: hitsample.to_string(),
                })
        };
        let too_many_parameters_check = |s: &mut Split<&str>| {
            if s.next().is_some() {
                Err(HitObjectParseError::TooManyParameters)
            } else {
                Ok(())
            }
        };*/
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
