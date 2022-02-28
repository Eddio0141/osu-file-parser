use std::{error::Error, fmt::Display, num::{ParseIntError, TryFromIntError}, str::FromStr};

use super::{Decimal, Integer, OsuFileParseError};

type ComboSkipCount = u8;

pub trait HitObject {
    fn x(&self) -> Integer;
    fn y(&self) -> Integer;
    fn set_x(&mut self, x: Integer);
    fn set_y(&mut self, y: Integer);

    fn time(&self) -> Integer;
    fn set_time(&mut self, time: Integer);

    fn obj_type(&self) -> &HitObjectType;

    fn new_combo(&self) -> bool;
    fn set_new_combo(&mut self, value: bool);

    fn combo_skip_count(&self) -> ComboSkipCount;
    fn set_combo_skip_count(&mut self, value: ComboSkipCount);

    fn hitsound(&self) -> &HitSound;
    fn set_hitsound(&mut self, hitsound: HitSound);

    fn hitsample(&self) -> &HitSample;
    fn hitsample_mut(&mut self) -> &mut HitSample;
}

#[derive(Debug)]
pub struct ComboSkipCountParseError;

impl Display for ComboSkipCountParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "There was a problem parsing a value to a 3 bit value")
    }
}

impl Error for ComboSkipCountParseError {}

pub fn parse_hitobject(hitobject: &str) -> Result<Box<dyn HitObject>, HitObjectParseError> {
    let obj_properties = hitobject.trim().split(',').collect::<Vec<_>>();

    let initial_properties_count = 5;

    let (x, y, time, obj_type, hitsound) = {
        let properties = obj_properties
            .iter()
            .take(initial_properties_count)
            .map(|property| property.parse::<Integer>())
            .collect::<Result<Vec<_>, _>>()?;

        if properties.len() < initial_properties_count {
            return Err(HitObjectParseError::MissingProperty);
        }

        (
            properties[0],
            properties[1],
            properties[2],
            properties[3],
            properties[4],
        )
    };
    
    let hitsound = HitSound::from(u8::try_from(hitsound)?);

    // type bit definition
    // 0: hitcircle, 1: slider, 2: newcombo, 3: spinner, 4 ~ 6: how many combo colours to skip, 7: osumania hold
    let (obj_type, new_combo, combo_skip_count) = {
        let new_combo = nth_bit_state_i32(obj_type, 2);

        let combo_skip_count = (obj_type >> 4) & 7;

        let obj_type = if nth_bit_state_i32(obj_type, 1) {
            HitObjectType::HitCircle
        } else if nth_bit_state_i32(obj_type, 2) {
            HitObjectType::Slider
        } else if nth_bit_state_i32(obj_type, 4) {
            HitObjectType::Spinner
        } else if nth_bit_state_i32(obj_type, 128) {
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

    let mut obj_properties = obj_properties
        .iter()
        .skip(initial_properties_count)
        .collect::<Vec<_>>();

    let hitsample = obj_properties
        .last()
        .ok_or(HitObjectParseError::MissingProperty)?
        .parse()?;
    obj_properties.remove(obj_properties.len() - 1);

    Ok(Box::new(match obj_type {
        HitObjectType::HitCircle => HitCircle {
            x,
            y,
            time,
            obj_type,
            hitsound,
            hitsample,
            new_combo,
            combo_skip_count,
        },
        HitObjectType::Slider => todo!(),
        HitObjectType::Spinner => todo!(),
        HitObjectType::OsuManiaHold => todo!(),
    }))
}

impl From<TryFromIntError> for HitObjectParseError {
    fn from(err: TryFromIntError) -> Self {
        Self::ValueParseError(Box::new(err))
    }
}

impl From<HitSampleParseError> for HitObjectParseError {
    fn from(err: HitSampleParseError) -> Self {
        Self::ValueParseError(Box::new(err))
    }
}

fn nth_bit_state_i32(value: i32, nth_bit: u32) -> bool {
    value & 2i32.pow(nth_bit) == 1
}

fn nth_bit_state_u8(value: u8, nth_bit: u32) -> bool {
    value & 2u8.pow(nth_bit) == 1
}

#[derive(Debug)]
pub enum HitObjectParseError {
    MissingProperty,
    ValueParseError(Box<dyn Error>),
}

impl Display for HitObjectParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err = match self {
            HitObjectParseError::MissingProperty => "A property of a hitobject is missing",
            HitObjectParseError::ValueParseError(_) => "There was a problem parsing a property",
        };

        write!(f, "{err}")
    }
}

impl Error for HitObjectParseError {}

impl From<HitObjectParseError> for OsuFileParseError {
    fn from(err: HitObjectParseError) -> Self {
        Self::SectionParseError {
            source: Box::new(err),
        }
    }
}

impl From<ParseIntError> for HitObjectParseError {
    fn from(err: ParseIntError) -> Self {
        HitObjectParseError::ValueParseError(Box::new(err))
    }
}

pub enum HitObjectType {
    HitCircle,
    Slider,
    Spinner,
    OsuManiaHold,
}

pub struct HitSound {
    normal: bool,
    whistle: bool,
    finish: bool,
    clap: bool,
}

impl Default for HitSound {
    fn default() -> Self {
        Self {
            normal: true,
            whistle: false,
            finish: false,
            clap: false,
        }
    }
}

impl HitSound {
    pub fn normal(&self) -> bool {
        self.normal
    }
    pub fn whistle(&self) -> bool {
        self.whistle
    }
    pub fn finish(&self) -> bool {
        self.finish
    }
    pub fn clap(&self) -> bool {
        self.clap
    }

    fn validate_normal(&mut self) {
        if !(self.normal && self.whistle && self.finish && self.clap) {
            self.normal = true;
        }
    }

    pub fn set_normal(&mut self, normal: bool) {
        self.normal = normal;
        self.validate_normal();
    }
    pub fn set_whistle(&mut self, whistle: bool) {
        self.whistle = whistle;
        self.validate_normal();
    }

    pub fn set_finish(&mut self, finish: bool) {
        self.finish = finish;
        self.validate_normal();
    }

    pub fn set_clap(&mut self, clap: bool) {
        self.clap = clap;
        self.validate_normal();
    }
}

impl From<u8> for HitSound {
    fn from(value: u8) -> Self {
        let normal = nth_bit_state_u8(value, 0);
        let whistle = nth_bit_state_u8(value, 1);
        let finish = nth_bit_state_u8(value, 2);
        let clap = nth_bit_state_u8(value, 3);

        let mut hitsound = Self {
            normal,
            whistle,
            finish,
            clap,
        };
        hitsound.validate_normal();

        hitsound
    }
}

#[derive(Default)]
pub struct HitSample {
    normal_set: SampleSet,
    addition_set: SampleSet,
    index: Option<usize>,
    volume: Volume,
    filename: String,
}

impl HitSample {
    pub fn normal_set(&self) -> SampleSet {
        self.normal_set
    }

    pub fn set_normal_set(&mut self, normal_set: SampleSet) {
        self.normal_set = normal_set;
    }

    pub fn addition_set(&self) -> SampleSet {
        self.addition_set
    }

    pub fn set_addition_set(&mut self, addition_set: SampleSet) {
        self.addition_set = addition_set;
    }

    pub fn index(&self) -> Option<usize> {
        self.index
    }

    pub fn set_index(&mut self, index: usize) {
        if index == 0 {
            self.index = None;
        } else {
            self.index = Some(index);
        }
    }

    pub fn use_timing_point_index(&self) -> bool {
        self.index.is_none()
    }

    pub fn set_use_timing_point_index(&mut self) {
        self.index = None;
    }

    pub fn volume(&self) -> &Volume {
        &self.volume
    }

    pub fn filename(&self) -> &str {
        self.filename.as_ref()
    }
}

impl FromStr for HitSample {
    type Err = HitSampleParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(':');

        let sample_set_count = 2;
        let (normal_set, addition_set) = {
            let mut sample_sets = Vec::new();

            for _ in 0..sample_set_count {
                sample_sets.push(SampleSet::new(
                    s.next()
                        .ok_or(HitSampleParseError::MissingProperty)?
                        .parse::<Integer>()?,
                )?);
            }

            (sample_sets[0], sample_sets[1])
        };

        let index = s
            .next()
            .ok_or(HitSampleParseError::MissingProperty)?
            .parse::<usize>()?;
        let index = if index == 0 { None } else { Some(index) };

        let volume = s
            .next()
            .ok_or(HitSampleParseError::MissingProperty)?
            .parse()?;

        // filename is empty if not specified
        let filename = s.next().unwrap_or_default();

        Ok(Self {
            normal_set,
            addition_set,
            index,
            volume,
            filename: filename.to_owned(),
        })
    }
}

impl From<VolumeParseError> for HitSampleParseError {
    fn from(err: VolumeParseError) -> Self {
        Self::ParseError(Box::new(err))
    }
}

impl From<SampleSetParseError> for HitSampleParseError {
    fn from(err: SampleSetParseError) -> Self {
        Self::ParseError(Box::new(err))
    }
}

#[derive(Debug)]
pub enum HitSampleParseError {
    MissingProperty,
    ParseError(Box<dyn Error>),
}

impl Error for HitSampleParseError {}

impl Display for HitSampleParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err = match self {
            HitSampleParseError::MissingProperty => "A property is missing",
            HitSampleParseError::ParseError(_) => "There was a problem parsing a value",
        };

        write!(f, "{err}")
    }
}

impl From<ParseIntError> for HitSampleParseError {
    fn from(err: ParseIntError) -> Self {
        Self::ParseError(Box::new(err))
    }
}

#[derive(Clone, Copy)]
pub enum SampleSet {
    NoCustomSampleSet,
    NormalSet,
    SoftSet,
    DrumSet,
}

impl Default for SampleSet {
    fn default() -> Self {
        Self::NoCustomSampleSet
    }
}

impl SampleSet {
    pub fn new(value: Integer) -> Result<SampleSet, SampleSetParseError> {
        match value {
            0 => Ok(SampleSet::NoCustomSampleSet),
            1 => Ok(SampleSet::NormalSet),
            2 => Ok(SampleSet::SoftSet),
            3 => Ok(SampleSet::DrumSet),
            _ => Err(SampleSetParseError),
        }
    }
}

#[derive(Debug)]
pub struct SampleSetParseError;

impl Display for SampleSetParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The value parsing is higher than 3")
    }
}

impl Error for SampleSetParseError {}

#[derive(Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Volume(Option<u8>);

impl Volume {
    pub fn new(volume: u8) -> Result<Volume, VolumeParseError> {
        match volume {
            0 => Ok(Self(None)),
            volume if volume <= 100 => Ok(Self(Some(volume))),
            _ => Err(VolumeParseError::VolumeTooHigh),
        }
    }

    pub fn volume(&self) -> Option<u8> {
        self.0
    }

    pub fn set_volume(&mut self, volume: u8) {
        self.0 = Some(volume);
    }

    pub fn use_timing_point_volume(&self) -> bool {
        self.0.is_none()
    }

    pub fn set_use_timing_point_volume(&mut self) {
        self.0 = None;
    }
}

impl FromStr for Volume {
    type Err = VolumeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let volume = s.parse::<u8>()?;

        Volume::new(volume)
    }
}

#[derive(Debug)]
pub enum VolumeParseError {
    VolumeTooHigh,
    InvalidString(Box<dyn Error>),
}

impl From<ParseIntError> for VolumeParseError {
    fn from(err: ParseIntError) -> Self {
        Self::InvalidString(Box::new(err))
    }
}

impl Display for VolumeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err = match self {
            VolumeParseError::VolumeTooHigh => {
                "Volume is too high. Requires to be in the range of 0 ~ 100"
            }
            VolumeParseError::InvalidString(_) => "Invalid string",
        };

        write!(f, "{err}")
    }
}

impl Error for VolumeParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if let VolumeParseError::InvalidString(err) = self {
            Some(err.as_ref())
        } else {
            None
        }
    }
}

pub struct HitCircle {
    x: Integer,
    y: Integer,
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
            x: Default::default(),
            y: Default::default(),
            time: Default::default(),
            obj_type: HitObjectType::HitCircle,
            hitsound: Default::default(),
            hitsample: Default::default(),
            new_combo: Default::default(),
            combo_skip_count: Default::default(),
        }
    }
}

impl HitObject for HitCircle {
    fn x(&self) -> Integer {
        self.x
    }

    fn y(&self) -> Integer {
        self.y
    }

    fn set_x(&mut self, x: Integer) {
        self.x = x;
    }

    fn set_y(&mut self, y: Integer) {
        self.y = y;
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
    pub fn new(
        x: Integer,
        y: Integer,
        time: Integer,
        hitsound: HitSound,
        hitsample: HitSample,
        new_combo: bool,
        combo_skip_count: ComboSkipCount,
    ) -> Self {
        Self {
            x,
            y,
            time,
            obj_type: HitObjectType::HitCircle,
            hitsound,
            hitsample,
            new_combo,
            combo_skip_count,
        }
    }
}

pub struct Slider {
    x: Integer,
    y: Integer,
    time: Integer,
    obj_type: HitObjectType,
    hitsound: HitSound,
    hitsample: HitSample,

    new_combo: bool,
    combo_skip_count: ComboSkipCount,

    curve_type: CurveType,
    curve_points: Vec<(Integer, Integer)>,
    slides: Integer,
    length: Decimal,
    // TODO
    edge_sounds: Vec<Integer>,
    // TODO
    edge_sets: Vec<String>,
}

impl HitObject for Slider {
    fn x(&self) -> Integer {
        self.x
    }

    fn y(&self) -> Integer {
        self.y
    }

    fn set_x(&mut self, x: Integer) {
        self.x = x;
    }

    fn set_y(&mut self, y: Integer) {
        self.y = y;
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

pub enum CurveType {
    Bezier,
    Centripetal,
    Linear,
    PerfectCircle,
}

pub struct PipeVec<T> {
    vec: Vec<T>,
}

impl<T> FromStr for PipeVec<T> {
    type Err = PipeVecParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug)]
pub struct PipeVecParseErr;

impl Display for PipeVecParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "There was a problem parsing a pipe-separated list of values"
        )
    }
}

impl Error for PipeVecParseErr {}
