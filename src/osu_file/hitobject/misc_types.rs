use std::{fmt::Display, num::ParseIntError, str::FromStr};

use crate::osu_file::Integer;

use super::error::*;

#[derive(Clone, Copy)]
pub struct ColonSet<F, S>(pub F, pub S);

impl<F, S> Display for ColonSet<F, S>
where
    F: Display,
    S: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.0, self.1)
    }
}

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

#[derive(Clone, Copy)]
pub enum SampleSet {
    NoCustomSampleSet,
    NormalSet,
    SoftSet,
    DrumSet,
}

impl Display for SampleSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err = match self {
            SampleSet::NoCustomSampleSet => '0',
            SampleSet::NormalSet => '1',
            SampleSet::SoftSet => '2',
            SampleSet::DrumSet => '3',
        };

        write!(f, "{err}")
    }
}

impl FromStr for SampleSet {
    type Err = SampleSetParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        SampleSet::try_from(s.parse::<Integer>()?)
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

#[derive(Clone, Copy)]
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

fn nth_bit_state_i64(value: i64, nth_bit: u8) -> bool {
    value >> nth_bit & 1 == 1
}

#[derive(Clone)]
pub struct PipeVec<T>(pub Vec<T>);

impl<T> Display for PipeVec<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join("|")
        )
    }
}

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

#[derive(PartialEq, Eq, Clone, Copy)]
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

#[derive(Default, Clone)]
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
