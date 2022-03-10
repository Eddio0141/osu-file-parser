pub mod error;
mod helper;
pub mod types;

use std::fmt::Display;

use rust_decimal::Decimal;

use self::error::*;
use self::helper::*;
use self::types::*;

use super::Integer;
use super::Position;

// TODO look into some crate that allows for a 3 bit value
type ComboSkipCount = u8;

/// An interface that represents a hitobject.
///
/// All hitobjects will have the properties: `x`, `y`, `time`, `type`, `hitsound`, `hitsample`.
///
/// The `type` property is a `u8` integer with each bit flags containing some information, which are split into the functions:
/// [hitobject_type][Self::obj_type], [new_combo][Self::new_combo], [combo_skip_count][Self::combo_skip_count]
pub trait HitObject: Display {
    /// Returns the position of the hitobject.
    fn position(&self) -> &Position;
    /// Sets the position of the hitobject.
    fn position_mut(&mut self) -> &mut Position;

    /// Returns the time when the object is to be hit, in milliseconds from the beginning of the beatmap's audio.
    fn time(&self) -> Integer;
    /// Sets the time when the object is to be hit, in milliseconds from the beginning of the beatmap's audio.
    fn set_time(&mut self, time: Integer);
    /// Returns the hitobject type.
    fn obj_type(&self) -> &HitObjectType;

    /// Returns if the hitobject is a new combo.
    fn new_combo(&self) -> bool;
    /// Sets if the hitobject is a new combo.;
    fn set_new_combo(&mut self, value: bool);

    /// Returns a 3-bit integer specifying how many combo colours to skip, if this object starts a new combo.
    fn combo_skip_count(&self) -> ComboSkipCount;
    /// Sets a 3-bit integer specifying how many combo colours to skip, if this object starts a new combo.
    fn set_combo_skip_count(&mut self, value: ComboSkipCount);

    /// Returns the [hitsound][HitSound] property of the hitobject.
    fn hitsound(&self) -> &HitSound;
    /// Sets the [hitsound][HitSound] property of the hitobject.
    fn set_hitsound(&mut self, hitsound: HitSound);

    /// Returns the [hitsample][HitSample] property of the hitobject.
    fn hitsample(&self) -> &HitSample;
    /// Mutably borrows the [hitsample][HitSample] property of the hitobject.
    fn hitsample_mut(&mut self) -> &mut HitSample;

    /// Returns a `String` for the `type` property of the hitobject, which is a `u8` integer containing various bit flag infomation.
    fn type_to_string(&self) -> String {
        let mut bit_flag: u8 = 0;

        bit_flag |= match self.obj_type() {
            HitObjectType::HitCircle => 1,
            HitObjectType::Slider => 2,
            HitObjectType::Spinner => 8,
            HitObjectType::OsuManiaHold => 128,
        };

        if self.new_combo() {
            bit_flag |= 4;
        }

        // 3 bit value from 4th ~ 6th bits
        bit_flag |= self.combo_skip_count() << 4;

        bit_flag.to_string()
    }
}

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
pub fn try_parse_hitobject(hitobject: &str) -> Result<HitObjectWrapper, HitObjectParseError> {
    let mut obj_properties = hitobject.trim().split(',');

    let (x, y, time, obj_type, hitsound) = {
        let properties = (&mut obj_properties).take(5).collect::<Vec<_>>();

        if properties.len() < 5 {
            let missing_name = match properties.len() {
                0 => "x",
                1 => "y",
                2 => "Time",
                3 => "ObjType",
                4 => "HitSound",
                _ => unreachable!(),
            };
            return Err(HitObjectParseError::MissingProperty(
                missing_name.to_string(),
            ));
        }

        let properties_parse = properties
            .iter()
            .map(|property| property.parse::<Integer>())
            .collect::<Vec<_>>();
        let first_err = properties_parse.iter().position(|result| result.is_err());

        if let Some(first_err) = first_err {
            // since the first_err is a valid index here
            let err = properties_parse
                .get(first_err)
                .unwrap()
                .clone()
                .unwrap_err();

            return Err(HitObjectParseError::ValueParseError {
                source: Box::new(err),
                value: properties[first_err].to_string(),
            });
        }

        let properties = properties_parse
            .iter()
            .cloned()
            .map(|property| property.ok().unwrap())
            .collect::<Vec<_>>();

        (
            properties[0],
            properties[1],
            properties[2],
            properties[3],
            properties[4],
        )
    };

    let position = Position { x, y };

    let hitsound = hitsound
        .try_into()
        .map_err(|err| HitObjectParseError::ValueParseError {
            source: Box::new(err),
            value: hitsound.to_string(),
        })?;

    // type bit definition
    // 0: hitcircle, 1: slider, 2: newcombo, 3: spinner, 4 ~ 6: how many combo colours to skip, 7: osumania hold
    let (obj_type, new_combo, combo_skip_count) = {
        let new_combo = nth_bit_state_i64(obj_type as i64, 2);

        let combo_skip_count = (obj_type >> 4) & 0b111;

        let obj_type = if nth_bit_state_i64(obj_type as i64, 0) {
            HitObjectType::HitCircle
        } else if nth_bit_state_i64(obj_type as i64, 1) {
            HitObjectType::Slider
        } else if nth_bit_state_i64(obj_type as i64, 3) {
            HitObjectType::Spinner
        } else if nth_bit_state_i64(obj_type as i64, 7) {
            HitObjectType::OsuManiaHold
        } else {
            HitObjectType::HitCircle
        };

        (
            obj_type,
            new_combo,
            // this is fine since I remove the bits above the 2nd
            combo_skip_count as u8,
        )
    };

    let hitsample = |obj_properties: &mut dyn Iterator<Item = &str>| {
        let property = obj_properties
            .next()
            .ok_or(HitObjectParseError::MissingProperty(
                "HitSample".to_string(),
            ))?;

        property
            .parse()
            .map_err(|err| HitObjectParseError::ValueParseError {
                value: property.to_string(),
                source: Box::new(err),
            })
    };

    Ok(match obj_type {
        HitObjectType::HitCircle => {
            let hitsample = hitsample(&mut obj_properties)?;

            HitObjectWrapper::HitCircle(HitCircle {
                position,
                time,
                obj_type,
                hitsound,
                hitsample,
                new_combo,
                combo_skip_count,
            })
        }
        HitObjectType::Slider => {
            // idk why ppy decided to just put in curve type without the usual , splitter
            let (curve_type, curve_points) = obj_properties
                .next()
                .ok_or(HitObjectParseError::MissingProperty(
                    "CurveType".to_string(),
                ))?
                .split_once('|')
                .ok_or(HitObjectParseError::MissingProperty(
                    "CurvePoints".to_string(),
                ))?;

            let curve_type =
                curve_type
                    .parse()
                    .map_err(|err| HitObjectParseError::ValueParseError {
                        value: curve_type.to_string(),
                        source: Box::new(err),
                    })?;

            let curve_points = str_to_pipe_vec(curve_points).map_err(|err| {
                HitObjectParseError::ValueParseError {
                    value: curve_points.to_string(),
                    source: Box::new(err),
                }
            })?;

            let slides = obj_properties
                .next()
                .ok_or(HitObjectParseError::MissingProperty("Slides".to_string()))?;
            let slides = slides
                .parse()
                .map_err(|err| HitObjectParseError::ValueParseError {
                    value: slides.to_string(),
                    source: Box::new(err),
                })?;

            let length = obj_properties
                .next()
                .ok_or(HitObjectParseError::MissingProperty("Length".to_string()))?;
            let length = length
                .parse()
                .map_err(|err| HitObjectParseError::ValueParseError {
                    value: length.to_string(),
                    source: Box::new(err),
                })?;

            let edge_sounds = obj_properties
                .next()
                .ok_or(HitObjectParseError::MissingProperty(
                    "EdgeSounds".to_string(),
                ))?;
            let edge_sounds = str_to_pipe_vec(edge_sounds).map_err(|err| {
                HitObjectParseError::ValueParseError {
                    value: edge_sounds.to_string(),
                    source: Box::new(err),
                }
            })?;

            let edge_sets = obj_properties
                .next()
                .ok_or(HitObjectParseError::MissingProperty("EdgeSets".to_string()))?;
            let edge_sets =
                str_to_pipe_vec(edge_sets).map_err(|err| HitObjectParseError::ValueParseError {
                    value: edge_sets.to_string(),
                    source: Box::new(err),
                })?;

            let hitsample = hitsample(&mut obj_properties)?;

            HitObjectWrapper::Slider(Slider {
                position,
                time,
                obj_type,
                hitsound,
                hitsample,
                new_combo,
                combo_skip_count,
                curve_type,
                curve_points,
                slides,
                length,
                edge_sounds,
                edge_sets,
            })
        }
        HitObjectType::Spinner => {
            let end_time = obj_properties
                .next()
                .ok_or(HitObjectParseError::MissingProperty("EndTime".to_string()))?;
            let end_time =
                end_time
                    .parse()
                    .map_err(|err| HitObjectParseError::ValueParseError {
                        value: end_time.to_string(),
                        source: Box::new(err),
                    })?;

            let hitsample = hitsample(&mut obj_properties)?;

            HitObjectWrapper::Spinner(Spinner {
                position,
                time,
                obj_type,
                hitsound,
                hitsample,
                new_combo,
                combo_skip_count,
                end_time,
            })
        }
        HitObjectType::OsuManiaHold => {
            // ppy has done it once again
            let (end_time, hitsample) = obj_properties
                .next()
                .ok_or(HitObjectParseError::MissingProperty("EndTime".to_string()))?
                .split_once(':')
                .ok_or(HitObjectParseError::MissingProperty(
                    "HitSample".to_string(),
                ))?;

            let end_time =
                end_time
                    .parse()
                    .map_err(|err| HitObjectParseError::ValueParseError {
                        value: end_time.to_string(),
                        source: Box::new(err),
                    })?;

            let hitsample =
                hitsample
                    .parse()
                    .map_err(|err| HitObjectParseError::ValueParseError {
                        value: hitsample.to_string(),
                        source: Box::new(err),
                    })?;

            HitObjectWrapper::OsuManiaHold(OsuManiaHold {
                position,
                time,
                obj_type,
                hitsound,
                hitsample,
                new_combo,
                combo_skip_count,
                end_time,
            })
        }
    })
}

/// Type contanining one of the hitobject types.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum HitObjectWrapper {
    /// Contains a [`HitCircle`].
    HitCircle(HitCircle),
    /// Contains a [`Slider`].
    Slider(Slider),
    /// Contains a [`Spinner`].
    Spinner(Spinner),
    /// Contains a [`OsuManiaHold`].
    OsuManiaHold(OsuManiaHold),
}

impl Display for HitObjectWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hitobject = match self {
            HitObjectWrapper::HitCircle(circle) => circle.to_string(),
            HitObjectWrapper::Slider(slider) => slider.to_string(),
            HitObjectWrapper::Spinner(spinner) => spinner.to_string(),
            HitObjectWrapper::OsuManiaHold(hold) => hold.to_string(),
        };

        write!(f, "{hitobject}")
    }
}

impl HitObjectWrapper {
    /// Turns itself into a boxed [HitObject].
    pub fn boxed_hitobject(&self) -> Box<dyn HitObject> {
        match self {
            HitObjectWrapper::HitCircle(circle) => Box::new(circle.clone()),
            HitObjectWrapper::Slider(slider) => Box::new(slider.clone()),
            HitObjectWrapper::Spinner(spinner) => Box::new(spinner.clone()),
            HitObjectWrapper::OsuManiaHold(hold) => Box::new(hold.clone()),
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
/// All possible `HitObject` types.
pub enum HitObjectType {
    /// A hitcircle.
    HitCircle,
    /// A slider.
    Slider,
    /// A spinner.
    Spinner,
    /// An osu!mania hold
    OsuManiaHold,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
/// A `HitCircle` object.
pub struct HitCircle {
    position: Position,
    time: Integer,
    obj_type: HitObjectType,
    hitsound: HitSound,
    hitsample: HitSample,

    new_combo: bool,
    combo_skip_count: ComboSkipCount,
}

impl Default for HitCircle {
    fn default() -> Self {
        Self {
            position: Default::default(),
            time: Default::default(),
            obj_type: HitObjectType::HitCircle,
            hitsound: Default::default(),
            hitsample: Default::default(),
            new_combo: Default::default(),
            combo_skip_count: Default::default(),
        }
    }
}

impl Display for HitCircle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let properties: Vec<String> = vec![
            self.position.x.to_string(),
            self.position.y.to_string(),
            self.time.to_string(),
            self.type_to_string(),
            self.hitsound.to_string(),
            self.hitsample.to_string(),
        ];

        write!(f, "{}", properties.join(","))
    }
}

impl HitObject for HitCircle {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }

    fn time(&self) -> Integer {
        self.time
    }

    fn set_time(&mut self, time: Integer) {
        self.time = time;
    }

    fn obj_type(&self) -> &HitObjectType {
        &self.obj_type
    }

    fn new_combo(&self) -> bool {
        self.new_combo
    }

    fn set_new_combo(&mut self, value: bool) {
        self.new_combo = value;
    }

    fn combo_skip_count(&self) -> ComboSkipCount {
        self.combo_skip_count
    }

    fn set_combo_skip_count(&mut self, value: ComboSkipCount) {
        self.combo_skip_count = value;
    }

    fn hitsound(&self) -> &HitSound {
        &self.hitsound
    }

    fn set_hitsound(&mut self, hitsound: HitSound) {
        self.hitsound = hitsound;
    }

    fn hitsample(&self) -> &HitSample {
        &self.hitsample
    }

    fn hitsample_mut(&mut self) -> &mut HitSample {
        &mut self.hitsample
    }
}

impl HitCircle {
    /// Creates a new `HitCircle` instance.
    pub fn new(
        pos: Position,
        time: Integer,
        hitsound: HitSound,
        hitsample: HitSample,
        new_combo: bool,
        combo_skip_count: ComboSkipCount,
    ) -> Self {
        Self {
            position: pos,
            time,
            obj_type: HitObjectType::HitCircle,
            hitsound,
            hitsample,
            new_combo,
            combo_skip_count,
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
/// A `Slider` object.
pub struct Slider {
    position: Position,
    time: Integer,
    obj_type: HitObjectType,
    hitsound: HitSound,
    hitsample: HitSample,

    new_combo: bool,
    combo_skip_count: ComboSkipCount,

    curve_type: CurveType,
    curve_points: Vec<CurvePoint>,
    slides: Integer,
    length: Decimal,
    edge_sounds: Vec<HitSound>,
    edge_sets: Vec<EdgeSet>,
}

impl Slider {
    /// Returns the `CurveType` of the slider.
    pub fn curve_type(&self) -> CurveType {
        self.curve_type
    }

    /// Sets the `CurveType` of the slider.
    pub fn set_curve_type(&mut self, curve_type: CurveType) {
        self.curve_type = curve_type;
    }

    /// Returns a reference to the `CurvePoints` of the slider.
    pub fn curve_points(&self) -> &[CurvePoint] {
        &self.curve_points
    }

    /// Returns a mutable reference to the `CurvePoints` of the slider.
    pub fn curve_points_mut(&mut self) -> &mut Vec<CurvePoint> {
        &mut self.curve_points
    }

    /// Returns a reference to the `EdgeSets` of the slider.
    pub fn edge_sets(&self) -> &[EdgeSet] {
        &self.edge_sets
    }

    /// Returns a mutable reference to the `EdgeSets` of the slider.
    pub fn edge_sets_mut(&mut self) -> &mut Vec<EdgeSet> {
        &mut self.edge_sets
    }
}

impl Display for Slider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let properties = vec![
            self.position.x.to_string(),
            self.position.y.to_string(),
            self.time.to_string(),
            self.type_to_string(),
            self.hitsound.to_string(),
            self.curve_type.to_string(),
        ];

        let properties_2 = vec![
            pipe_vec_to_string(&self.curve_points),
            self.slides.to_string(),
            self.length.to_string(),
            pipe_vec_to_string(&self.edge_sounds),
            pipe_vec_to_string(&self.edge_sets),
            self.hitsample.to_string(),
        ];

        write!(f, "{}|{}", properties.join(","), properties_2.join(","))
    }
}

impl HitObject for Slider {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }

    fn time(&self) -> Integer {
        self.time
    }

    fn set_time(&mut self, time: Integer) {
        self.time = time;
    }

    fn obj_type(&self) -> &HitObjectType {
        &self.obj_type
    }

    fn new_combo(&self) -> bool {
        self.new_combo
    }

    fn set_new_combo(&mut self, value: bool) {
        self.new_combo = value;
    }

    fn combo_skip_count(&self) -> ComboSkipCount {
        self.combo_skip_count
    }

    fn set_combo_skip_count(&mut self, value: ComboSkipCount) {
        self.combo_skip_count = value;
    }

    fn hitsound(&self) -> &HitSound {
        &self.hitsound
    }

    fn set_hitsound(&mut self, hitsound: HitSound) {
        self.hitsound = hitsound;
    }

    fn hitsample(&self) -> &HitSample {
        &self.hitsample
    }

    fn hitsample_mut(&mut self) -> &mut HitSample {
        &mut self.hitsample
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
/// A `Spinner` object.
pub struct Spinner {
    position: Position,
    time: Integer,
    obj_type: HitObjectType,
    hitsound: HitSound,
    hitsample: HitSample,

    new_combo: bool,
    combo_skip_count: ComboSkipCount,

    end_time: Integer,
}

impl HitObject for Spinner {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }

    fn time(&self) -> Integer {
        self.time
    }

    fn set_time(&mut self, time: Integer) {
        self.time = time;
    }

    fn obj_type(&self) -> &HitObjectType {
        &self.obj_type
    }

    fn new_combo(&self) -> bool {
        self.new_combo
    }

    fn set_new_combo(&mut self, value: bool) {
        self.new_combo = value;
    }

    fn combo_skip_count(&self) -> ComboSkipCount {
        self.combo_skip_count
    }

    fn set_combo_skip_count(&mut self, value: ComboSkipCount) {
        self.combo_skip_count = value;
    }

    fn hitsound(&self) -> &HitSound {
        &self.hitsound
    }

    fn set_hitsound(&mut self, hitsound: HitSound) {
        self.hitsound = hitsound;
    }

    fn hitsample(&self) -> &HitSample {
        &self.hitsample
    }

    fn hitsample_mut(&mut self) -> &mut HitSample {
        &mut self.hitsample
    }
}

impl Display for Spinner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let properties: Vec<String> = vec![
            self.position.x.to_string(),
            self.position.y.to_string(),
            self.time.to_string(),
            self.type_to_string(),
            self.hitsound.to_string(),
            self.end_time.to_string(),
            self.hitsample.to_string(),
        ];

        write!(f, "{}", properties.join(","))
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self {
            // TODO check if 0ms spinner is valid
            position: Default::default(),
            time: Default::default(),
            obj_type: HitObjectType::Spinner,
            hitsound: Default::default(),
            hitsample: Default::default(),
            new_combo: Default::default(),
            combo_skip_count: Default::default(),
            end_time: Default::default(),
        }
    }
}

impl Spinner {
    /// Creates a new `Spinner` instance.
    pub fn new(
        position: Position,
        time: Integer,
        hitsound: HitSound,
        hitsample: HitSample,
        new_combo: bool,
        combo_skip_count: ComboSkipCount,
        end_time: Integer,
    ) -> Self {
        Self {
            position,
            time,
            obj_type: HitObjectType::Spinner,
            hitsound,
            hitsample,
            new_combo,
            combo_skip_count,
            end_time,
        }
    }

    /// Returns the `end_time` of the Spinner.
    pub fn end_time(&self) -> i32 {
        self.end_time
    }

    // TODO is it valid if end_time is lower or equals to time
    /// Sets the `end_time` of the Spinner.
    pub fn set_end_time(&mut self, end_time: Integer) {
        self.end_time = end_time;
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
/// A `Osu!Mania Hold` object.
pub struct OsuManiaHold {
    position: Position,
    time: Integer,
    obj_type: HitObjectType,
    hitsound: HitSound,
    hitsample: HitSample,

    new_combo: bool,
    combo_skip_count: ComboSkipCount,

    end_time: Integer,
}

impl HitObject for OsuManiaHold {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }

    fn time(&self) -> Integer {
        self.time
    }

    fn set_time(&mut self, time: Integer) {
        self.time = time;
    }

    fn obj_type(&self) -> &HitObjectType {
        &self.obj_type
    }

    fn new_combo(&self) -> bool {
        self.new_combo
    }

    fn set_new_combo(&mut self, value: bool) {
        self.new_combo = value;
    }

    fn combo_skip_count(&self) -> ComboSkipCount {
        self.combo_skip_count
    }

    fn set_combo_skip_count(&mut self, value: ComboSkipCount) {
        self.combo_skip_count = value;
    }

    fn hitsound(&self) -> &HitSound {
        &self.hitsound
    }

    fn set_hitsound(&mut self, hitsound: HitSound) {
        self.hitsound = hitsound;
    }

    fn hitsample(&self) -> &HitSample {
        &self.hitsample
    }

    fn hitsample_mut(&mut self) -> &mut HitSample {
        &mut self.hitsample
    }
}

impl Display for OsuManiaHold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let properties: Vec<String> = vec![
            self.position.x.to_string(),
            self.position.y.to_string(),
            self.time.to_string(),
            self.type_to_string(),
            self.hitsound.to_string(),
            self.end_time.to_string(),
        ];

        write!(f, "{}:{}", properties.join(","), self.hitsample)
    }
}

impl Default for OsuManiaHold {
    fn default() -> Self {
        Self {
            // TODO check if 0ms hold is valid
            position: Position {
                x: Default::default(),
                ..Default::default()
            },
            time: Default::default(),
            obj_type: HitObjectType::OsuManiaHold,
            hitsound: Default::default(),
            hitsample: Default::default(),
            new_combo: Default::default(),
            combo_skip_count: Default::default(),
            end_time: Default::default(),
        }
    }
}

impl OsuManiaHold {
    /// Returns a new `OsuManiaHold` instance.
    pub fn new(
        position: Position,
        time: Integer,
        hitsound: HitSound,
        hitsample: HitSample,
        new_combo: bool,
        combo_skip_count: ComboSkipCount,
        end_time: Integer,
    ) -> Self {
        Self {
            position,
            time,
            obj_type: HitObjectType::OsuManiaHold,
            hitsound,
            hitsample,
            new_combo,
            combo_skip_count,
            end_time,
        }
    }

    /// Returns the `end_time` for the `OsuManiaHold`.
    pub fn end_time(&self) -> i32 {
        self.end_time
    }

    // TODO is it valid if end_time is lower or equals to time
    /// Sets the `end_time` for the `OsuManiaHold`.
    pub fn set_end_time(&mut self, end_time: Integer) {
        self.end_time = end_time;
    }
}
