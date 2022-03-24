pub mod error;
pub mod types;

use std::fmt::Display;
use std::str::FromStr;

use nom::bytes::complete::is_not;
use nom::character::streaming::char;
use nom::combinator::map_res;
use nom::sequence::tuple;
use rust_decimal::Decimal;
use thiserror::Error;

use self::error::*;
use self::types::*;
use super::helper::*;

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
        let colon = char::<_, nom::error::Error<_>>(',');
        let int = map_res(is_not(","), |f: &str| f.parse::<Integer>());

        let (s, (x, _, y, _)) = tuple((int, colon, int, colon))(s).unwrap();
        // let (s, (x, _)) = tuple((int, colon))(s).unwrap();
        // let (s, (y, _)) = tuple((int, colon))(s).unwrap();

        // let (s, (x, _, y)) = tuple((test, colon, test))(s).unwrap();

        todo!();
        // let mut obj_properties = s.trim().split(',');

        // let (x, y, time, obj_type, hitsound) = {
        //     let properties = (&mut obj_properties).take(5).collect::<Vec<_>>();

        //     if properties.len() < 5 {
        //         let missing_name = match properties.len() {
        //             0 => "x",
        //             1 => "y",
        //             2 => "Time",
        //             3 => "ObjType",
        //             4 => "HitSound",
        //             _ => unreachable!(),
        //         };
        //         return Err(HitObjectParseError::MissingProperty(
        //             missing_name.to_string(),
        //         ));
        //     }

        //     let properties_parse = properties
        //         .iter()
        //         .map(|property| property.parse::<Integer>())
        //         .collect::<Vec<_>>();
        //     let first_err = properties_parse.iter().position(|result| result.is_err());

        //     if let Some(first_err) = first_err {
        //         // since the first_err is a valid index here
        //         let err = properties_parse
        //             .get(first_err)
        //             .unwrap()
        //             .clone()
        //             .unwrap_err();

        //         return Err(HitObjectParseError::ValueParseError {
        //             source: Box::new(err),
        //             value: properties[first_err].to_string(),
        //         });
        //     }

        //     let properties = properties_parse
        //         .iter()
        //         .cloned()
        //         .map(|property| property.ok().unwrap())
        //         .collect::<Vec<_>>();

        //     (
        //         properties[0],
        //         properties[1],
        //         properties[2],
        //         properties[3],
        //         properties[4],
        //     )
        // };

        // let position = Position { x, y };

        // let hitsound = hitsound
        //     .try_into()
        //     .map_err(|err| HitObjectParseError::ValueParseError {
        //         source: Box::new(err),
        //         value: hitsound.to_string(),
        //     })?;

        // // type bit definition
        // // 0: hitcircle, 1: slider, 2: newcombo, 3: spinner, 4 ~ 6: how many combo colours to skip, 7: osumania hold
        // let (obj_type, new_combo, combo_skip_count) = {
        //     let new_combo = nth_bit_state_i64(obj_type as i64, 2);

        //     let combo_skip_count = (obj_type >> 4) & 0b111;

        //     let obj_type = if nth_bit_state_i64(obj_type as i64, 0) {
        //         HitObjectType::HitCircle
        //     } else if nth_bit_state_i64(obj_type as i64, 1) {
        //         HitObjectType::Slider
        //     } else if nth_bit_state_i64(obj_type as i64, 3) {
        //         HitObjectType::Spinner
        //     } else if nth_bit_state_i64(obj_type as i64, 7) {
        //         HitObjectType::OsuManiaHold
        //     } else {
        //         HitObjectType::HitCircle
        //     };

        //     (
        //         obj_type,
        //         new_combo,
        //         // this is fine since I remove the bits above the 2nd
        //         combo_skip_count as u8,
        //     )
        // };

        // let hitsample = |obj_properties: &mut dyn Iterator<Item = &str>| {
        //     let property = obj_properties
        //         .next()
        //         .ok_or_else(|| HitObjectParseError::MissingProperty("HitSample".to_string()))?;

        //     property
        //         .parse()
        //         .map_err(|err| HitObjectParseError::ValueParseError {
        //             value: property.to_string(),
        //             source: Box::new(err),
        //         })
        // };

        // Ok(match obj_type {
        //     HitObjectType::HitCircle => {
        //         let hitsample = hitsample(&mut obj_properties)?;

        //         HitObjectWrapper::HitCircle(HitCircle {
        //             position,
        //             time,
        //             obj_type,
        //             hitsound,
        //             hitsample,
        //             new_combo,
        //             combo_skip_count,
        //         })
        //     }
        //     HitObjectType::Slider => {
        //         // idk why ppy decided to just put in curve type without the usual , splitter
        //         let (curve_type, curve_points) = obj_properties
        //             .next()
        //             .ok_or_else(|| HitObjectParseError::MissingProperty("CurveType".to_string()))?
        //             .split_once('|')
        //             .ok_or_else(|| {
        //                 HitObjectParseError::MissingProperty("CurvePoints".to_string())
        //             })?;

        //         let curve_type =
        //             curve_type
        //                 .parse()
        //                 .map_err(|err| HitObjectParseError::ValueParseError {
        //                     value: curve_type.to_string(),
        //                     source: Box::new(err),
        //                 })?;

        //         let curve_points = str_to_pipe_vec(curve_points).map_err(|err| {
        //             HitObjectParseError::ValueParseError {
        //                 value: curve_points.to_string(),
        //                 source: Box::new(err),
        //             }
        //         })?;

        //         let slides = obj_properties
        //             .next()
        //             .ok_or_else(|| HitObjectParseError::MissingProperty("Slides".to_string()))?;
        //         let slides =
        //             slides
        //                 .parse()
        //                 .map_err(|err| HitObjectParseError::ValueParseError {
        //                     value: slides.to_string(),
        //                     source: Box::new(err),
        //                 })?;

        //         let length = obj_properties
        //             .next()
        //             .ok_or_else(|| HitObjectParseError::MissingProperty("Length".to_string()))?;
        //         let length =
        //             length
        //                 .parse()
        //                 .map_err(|err| HitObjectParseError::ValueParseError {
        //                     value: length.to_string(),
        //                     source: Box::new(err),
        //                 })?;

        //         let edge_sounds = obj_properties.next().ok_or_else(|| {
        //             HitObjectParseError::MissingProperty("EdgeSounds".to_string())
        //         })?;
        //         let edge_sounds = str_to_pipe_vec(edge_sounds).map_err(|err| {
        //             HitObjectParseError::ValueParseError {
        //                 value: edge_sounds.to_string(),
        //                 source: Box::new(err),
        //             }
        //         })?;

        //         let edge_sets = obj_properties
        //             .next()
        //             .ok_or_else(|| HitObjectParseError::MissingProperty("EdgeSets".to_string()))?;
        //         let edge_sets = str_to_pipe_vec(edge_sets).map_err(|err| {
        //             HitObjectParseError::ValueParseError {
        //                 value: edge_sets.to_string(),
        //                 source: Box::new(err),
        //             }
        //         })?;

        //         let hitsample = hitsample(&mut obj_properties)?;

        //         HitObjectWrapper::Slider(Slider {
        //             position,
        //             time,
        //             obj_type,
        //             hitsound,
        //             hitsample,
        //             new_combo,
        //             combo_skip_count,
        //             curve_type,
        //             curve_points,
        //             slides,
        //             length,
        //             edge_sounds,
        //             edge_sets,
        //         })
        //     }
        //     HitObjectType::Spinner => {
        //         let end_time = obj_properties
        //             .next()
        //             .ok_or_else(|| HitObjectParseError::MissingProperty("EndTime".to_string()))?;
        //         let end_time =
        //             end_time
        //                 .parse()
        //                 .map_err(|err| HitObjectParseError::ValueParseError {
        //                     value: end_time.to_string(),
        //                     source: Box::new(err),
        //                 })?;

        //         let hitsample = hitsample(&mut obj_properties)?;

        //         HitObjectWrapper::Spinner(Spinner {
        //             position,
        //             time,
        //             obj_type,
        //             hitsound,
        //             hitsample,
        //             new_combo,
        //             combo_skip_count,
        //             end_time,
        //         })
        //     }
        //     HitObjectType::OsuManiaHold => {
        //         // ppy has done it once again
        //         let (end_time, hitsample) = obj_properties
        //             .next()
        //             .ok_or_else(|| HitObjectParseError::MissingProperty("EndTime".to_string()))?
        //             .split_once(':')
        //             .ok_or_else(|| HitObjectParseError::MissingProperty("HitSample".to_string()))?;

        //         let end_time =
        //             end_time
        //                 .parse()
        //                 .map_err(|err| HitObjectParseError::ValueParseError {
        //                     value: end_time.to_string(),
        //                     source: Box::new(err),
        //                 })?;

        //         let hitsample =
        //             hitsample
        //                 .parse()
        //                 .map_err(|err| HitObjectParseError::ValueParseError {
        //                     value: hitsample.to_string(),
        //                     source: Box::new(err),
        //                 })?;

        //         HitObjectWrapper::OsuManiaHold(OsuManiaHold {
        //             position,
        //             time,
        //             obj_type,
        //             hitsound,
        //             hitsample,
        //             new_combo,
        //             combo_skip_count,
        //             end_time,
        //         })
        //     }
        // })
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
