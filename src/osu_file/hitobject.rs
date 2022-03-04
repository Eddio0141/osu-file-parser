use std::{
    error::Error,
    fmt::Display,
    num::{ParseFloatError, ParseIntError, TryFromIntError},
    str::FromStr,
};

use super::{Decimal, Integer, OsuFileParseError};

type ComboSkipCount = u8;

pub trait HitObject: Display {
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

    fn type_to_string(&self) -> String {
        let mut bit_flag: u8 = 0;

        bit_flag |= match self.obj_type() {
            HitObjectType::HitCircle => 1,
            HitObjectType::Slider => 2,
            HitObjectType::Spinner => 8,
            HitObjectType::OsuManiaHold => 128,
        };

        println!("{}", bit_flag);

        if self.new_combo() {
            bit_flag |= 4;
        }

        // 3 bit value from 4th ~ 6th bits
        bit_flag |= self.combo_skip_count() << 4;

        bit_flag.to_string()
    }
}

#[derive(Debug)]
pub struct ComboSkipCountParseError;

impl Display for ComboSkipCountParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "There was a problem parsing a value to a 3 bit value")
    }
}

impl Error for ComboSkipCountParseError {}

/// Attempts to parse a `&str` into a `HitObject`.
///
/// # Example
/// ```
/// use osu_file_parser::osu_file::hitobject::parse_hitobject;
///
/// let hitcircle_str = "221,350,9780,1,0,0:0:0:0:";
/// let slider_str = "31,85,3049,2,0,B|129:55|123:136|228:86,1,172.500006580353,2|0,3:2|0:2,0:2:0:0:";
///
/// let hitcircle = parse_hitobject(hitcircle_str).unwrap();
/// let slider = parse_hitobject(slider_str).unwrap();
///
/// assert_eq!(hitcircle_str, hitcircle.to_string());
/// assert_eq!(slider_str, slider.to_string());
/// ```
pub fn parse_hitobject(hitobject: &str) -> Result<Box<dyn HitObject>, HitObjectParseError> {
    let mut obj_properties = hitobject.trim().split(',');

    let (x, y, time, obj_type, hitsound) = {
        let properties = (&mut obj_properties)
            .take(5)
            .map(|property| property.parse::<Integer>())
            .collect::<Result<Vec<_>, _>>()?;

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
        let new_combo = nth_bit_state_i64(obj_type as i64, 2);

        let combo_skip_count = (obj_type >> 4) & 7;

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

    let hitsample = (&mut obj_properties)
        .last()
        .ok_or(HitObjectParseError::MissingProperty)?
        .parse()?;

    Ok(match obj_type {
        HitObjectType::HitCircle => Box::new(HitCircle {
            x,
            y,
            time,
            obj_type,
            hitsound,
            hitsample,
            new_combo,
            combo_skip_count,
        }),
        HitObjectType::Slider => {
            let curve_type = obj_properties
                .next()
                .ok_or(HitObjectParseError::MissingProperty)?
                .parse::<CurveType>()?;

            let curve_points = obj_properties
                .next()
                .ok_or(HitObjectParseError::MissingProperty)?
                .parse::<PipeVec<ColonSet<Integer, Integer>>>()?;

            let slides = obj_properties
                .next()
                .ok_or(HitObjectParseError::MissingProperty)?
                .parse()?;

            let length = obj_properties
                .next()
                .ok_or(HitObjectParseError::MissingProperty)?
                .parse()?;

            let edge_sounds = obj_properties
                .next()
                .ok_or(HitObjectParseError::MissingProperty)?
                .parse()?;

            let edge_sets = obj_properties
                .next()
                .ok_or(HitObjectParseError::MissingProperty)?
                .parse()?;

            Box::new(Slider {
                x,
                y,
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
        HitObjectType::Spinner => todo!(),
        HitObjectType::OsuManiaHold => todo!(),
    })
}

impl From<SampleSetParseError> for ColonSetParseError {
    fn from(err: SampleSetParseError) -> Self {
        ColonSetParseError::ValueParseError(Box::new(err))
    }
}

impl From<ParseFloatError> for HitObjectParseError {
    fn from(err: ParseFloatError) -> Self {
        HitObjectParseError::ValueParseError(Box::new(err))
    }
}
impl From<ParseIntError> for PipeVecParseErr {
    fn from(err: ParseIntError) -> Self {
        PipeVecParseErr(Box::new(err))
    }
}

impl From<PipeVecParseErr> for HitObjectParseError {
    fn from(err: PipeVecParseErr) -> Self {
        HitObjectParseError::ValueParseError(Box::new(err))
    }
}

impl From<ColonSetParseError> for PipeVecParseErr {
    fn from(err: ColonSetParseError) -> Self {
        PipeVecParseErr(Box::new(err))
    }
}

impl From<ParseIntError> for ColonSetParseError {
    fn from(err: ParseIntError) -> Self {
        ColonSetParseError::ValueParseError(Box::new(err))
    }
}

impl From<CurveTypeParseError> for HitObjectParseError {
    fn from(err: CurveTypeParseError) -> Self {
        Self::ValueParseError(Box::new(err))
    }
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

fn nth_bit_state_i64(value: i64, nth_bit: u8) -> bool {
    value >> nth_bit & 1 == 1
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

impl Display for HitSound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut bit_mask = 0;

        if self.normal {
            bit_mask |= 1;
        }
        if self.whistle {
            bit_mask |= 2;
        }
        if self.finish {
            bit_mask |= 4;
        }
        if self.clap {
            bit_mask |= 8;
        }

        write!(f, "{bit_mask}")
    }
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
        if !(self.normal || self.whistle || self.finish || self.clap) {
            true
        } else {
            self.normal
        }
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

    pub fn set_normal(&mut self, normal: bool) {
        self.normal = normal;
    }
    pub fn set_whistle(&mut self, whistle: bool) {
        self.whistle = whistle;
    }

    pub fn set_finish(&mut self, finish: bool) {
        self.finish = finish;
    }

    pub fn set_clap(&mut self, clap: bool) {
        self.clap = clap;
    }
}

impl FromStr for HitSound {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(HitSound::from(s.parse::<u8>()?))
    }
}

impl From<u8> for HitSound {
    fn from(value: u8) -> Self {
        let normal = nth_bit_state_i64(value as i64, 0);
        let whistle = nth_bit_state_i64(value as i64, 1);
        let finish = nth_bit_state_i64(value as i64, 2);
        let clap = nth_bit_state_i64(value as i64, 3);

        Self {
            normal,
            whistle,
            finish,
            clap,
        }
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

impl Display for HitSample {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let normal_set: Integer = self.normal_set.into();
        let addition_set: Integer = self.addition_set.into();
        let index = self.index.unwrap_or(0);
        let volume: Integer = self.volume.into();
        let filename = &self.filename;

        write!(f, "{normal_set}:{addition_set}:{index}:{volume}:{filename}")
    }
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
                sample_sets.push(
                    s.next()
                        .ok_or(HitSampleParseError::MissingProperty)?
                        .parse()?,
                );
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

impl FromStr for SampleSet {
    type Err = SampleSetParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(SampleSet::try_from(s.parse::<Integer>()?)?)
    }
}

impl From<ParseIntError> for SampleSetParseError {
    fn from(err: ParseIntError) -> Self {
        Self::ValueParseError(Box::new(err))
    }
}

impl From<SampleSet> for Integer {
    fn from(sampleset: SampleSet) -> Self {
        match sampleset {
            SampleSet::NoCustomSampleSet => 0,
            SampleSet::NormalSet => 1,
            SampleSet::SoftSet => 2,
            SampleSet::DrumSet => 3,
        }
    }
}

impl Default for SampleSet {
    fn default() -> Self {
        Self::NoCustomSampleSet
    }
}

impl TryFrom<Integer> for SampleSet {
    type Error = SampleSetParseError;

    fn try_from(value: Integer) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SampleSet::NoCustomSampleSet),
            1 => Ok(SampleSet::NormalSet),
            2 => Ok(SampleSet::SoftSet),
            3 => Ok(SampleSet::DrumSet),
            _ => Err(SampleSetParseError::ValueHigherThanThree),
        }
    }
}

#[derive(Debug)]
pub enum SampleSetParseError {
    ValueHigherThanThree,
    ValueParseError(Box<dyn Error>),
}

impl Display for SampleSetParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err = match self {
            SampleSetParseError::ValueHigherThanThree => "The value parsing is higher than 3.",
            SampleSetParseError::ValueParseError(_) => "There was a problem parsing the value.",
        };

        write!(f, "{err}")
    }
}

impl Error for SampleSetParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if let SampleSetParseError::ValueParseError(err) = self {
            Some(err.as_ref())
        } else {
            None
        }
    }
}

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Volume(Option<u8>);

impl From<Volume> for Integer {
    fn from(volume: Volume) -> Self {
        match volume.0 {
            Some(volume) => volume as Integer,
            None => 0,
        }
    }
}

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

impl Display for HitCircle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let properties: Vec<String> = vec![
            self.x.to_string(),
            self.y.to_string(),
            self.time.to_string(),
            self.type_to_string(),
            self.hitsound.to_string(),
            self.hitsample.to_string(),
        ];

        write!(f, "{}", properties.join(","))
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
    curve_points: PipeVec<ColonSet<Integer, Integer>>,
    slides: Integer,
    length: Decimal,
    edge_sounds: PipeVec<HitSound>,
    edge_sets: PipeVec<ColonSet<SampleSet, SampleSet>>,
}

impl Display for Slider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let properties: Vec<String> = vec![self.x.to_string()];

        todo!();

        write!(f, "{}", properties.join(","))
    }
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

#[derive(PartialEq, Eq)]
pub enum CurveType {
    Bezier,
    Centripetal,
    Linear,
    PerfectCircle,
}

impl Display for CurveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            CurveType::Bezier => 'B',
            CurveType::Centripetal => 'C',
            CurveType::Linear => 'L',
            CurveType::PerfectCircle => 'P',
        };

        write!(f, "{value}")
    }
}

impl FromStr for CurveType {
    type Err = CurveTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "B" => Ok(Self::Bezier),
            "C" => Ok(Self::Centripetal),
            "L" => Ok(Self::Linear),
            "P" => Ok(Self::PerfectCircle),
            _ => Err(CurveTypeParseError),
        }
    }
}

#[derive(Debug)]
pub struct CurveTypeParseError;

impl Error for CurveTypeParseError {}

impl Display for CurveTypeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error, tried to parse an invalid string as curve type.")
    }
}

struct PipeVec<T>(pub Vec<T>);

impl<T> FromStr for PipeVec<T>
where
    T: FromStr,
    PipeVecParseErr: From<<T as FromStr>::Err>,
{
    type Err = PipeVecParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split('|')
                .map(|s| s.parse())
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

struct ColonSet<F, S>(pub F, pub S);

impl<F, S> FromStr for ColonSet<F, S>
where
    F: FromStr,
    S: FromStr,
    ColonSetParseError: From<<F as FromStr>::Err> + From<<S as FromStr>::Err>,
{
    type Err = ColonSetParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(':');

        let first = s
            .next()
            .ok_or(ColonSetParseError::MissingFirstItem)?
            .parse::<F>()?;
        let second = s
            .next()
            .ok_or(ColonSetParseError::MissingSecondItem)?
            .parse::<S>()?;

        if s.count() > 0 {
            Err(ColonSetParseError::MoreThanTwoItems)
        } else {
            Ok(ColonSet(first, second))
        }
    }
}

#[derive(Debug)]
enum ColonSetParseError {
    MissingFirstItem,
    MissingSecondItem,
    MoreThanTwoItems,
    ValueParseError(Box<dyn Error>),
}

impl Display for ColonSetParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err = match self {
            ColonSetParseError::MissingFirstItem => "Missing the first item in the colon set",
            ColonSetParseError::MissingSecondItem => "Missing the second item in the colon set",
            ColonSetParseError::MoreThanTwoItems => "There is more than 2 items in the colon set",
            ColonSetParseError::ValueParseError(_) => {
                "There is a problem parsing a value to a colon set"
            }
        };

        write!(f, "{err}")
    }
}

impl Error for ColonSetParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if let ColonSetParseError::ValueParseError(err) = self {
            Some(err.as_ref())
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct PipeVecParseErr(Box<dyn Error>);

impl Display for PipeVecParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "There was a problem parsing a pipe-separated list of values"
        )
    }
}

impl Error for PipeVecParseErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.0.as_ref())
    }
}
