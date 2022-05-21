//! Module defining misc types used for different hitobjects, such as [CurveType] used for [Slider][super::Slider] curve types.

use std::{fmt::Display, num::NonZeroUsize, str::FromStr};

use strum_macros::{Display, EnumString, FromRepr};

use crate::{
    helper::nth_bit_state_i64,
    osu_file::{Integer, Position},
};

use super::error::*;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ComboSkipCount(u8);

impl ComboSkipCount {
    pub fn new(count: u8) -> Result<Self, ComboSkipCountTooHigh> {
        Self::try_from(count)
    }

    pub fn get(&self) -> u8 {
        self.0
    }

    pub fn set(&mut self, count: u8) -> Result<(), ComboSkipCountTooHigh> {
        let new_self = Self::try_from(count)?;
        *self = new_self;
        Ok(())
    }
}

impl TryFrom<u8> for ComboSkipCount {
    type Error = ComboSkipCountTooHigh;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        // limit to 3 bits
        if value > 0b111 {
            Err(ComboSkipCountTooHigh(value))
        } else {
            Ok(Self(value))
        }
    }
}

impl From<ComboSkipCount> for u8 {
    fn from(count: ComboSkipCount) -> Self {
        count.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
/// Sample sets used for the `edgeSounds`.
pub struct EdgeSet {
    /// Sample set of the normal sound.
    pub normal_set: SampleSet,
    /// Sample set of the whistle, finish, and clap sounds.
    pub addition_set: SampleSet,
}

impl FromStr for EdgeSet {
    type Err = ColonSetParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split(':').collect::<Vec<_>>();

        if s.len() > 2 {
            return Err(ColonSetParseError::MoreThanTwoItems(s[2..].join(":")));
        }

        let normal_set = s.get(0).ok_or(ColonSetParseError::MissingFirstItem)?;
        let addition_set = s.get(1).ok_or(ColonSetParseError::MissingSecondItem)?;

        let normal_set = normal_set
            .parse()
            .map_err(|err| ColonSetParseError::ValueParseError {
                source: Box::new(err),
                value: normal_set.to_string(),
            })?;
        let addition_set =
            addition_set
                .parse()
                .map_err(|err| ColonSetParseError::ValueParseError {
                    source: Box::new(err),
                    value: addition_set.to_string(),
                })?;

        Ok(Self {
            normal_set,
            addition_set,
        })
    }
}

impl Display for EdgeSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.normal_set, self.addition_set)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
/// Anchor point used to construct the [`slider`][super::Slider].
pub struct CurvePoint(pub Position);

impl FromStr for CurvePoint {
    type Err = ColonSetParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split(':').collect::<Vec<_>>();

        if s.len() > 2 {
            return Err(ColonSetParseError::MoreThanTwoItems(s[2..].join(":")));
        }

        let x = s.get(0).ok_or(ColonSetParseError::MissingFirstItem)?;
        let y = s.get(1).ok_or(ColonSetParseError::MissingSecondItem)?;

        let x = x
            .parse()
            .map_err(|err| ColonSetParseError::ValueParseError {
                source: Box::new(err),
                value: x.to_string(),
            })?;
        let y = y
            .parse()
            .map_err(|err| ColonSetParseError::ValueParseError {
                source: Box::new(err),
                value: y.to_string(),
            })?;

        let position = Position { x, y };

        Ok(Self(position))
    }
}

impl Display for CurvePoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.0.x, self.0.y)
    }
}

/// Used for `normal_set` and `addition_set` for the `[hitobject]`[super::HitObject].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, FromRepr)]
pub enum SampleSet {
    /// No custom sample set.
    NoCustomSampleSet,
    /// Normal set.
    NormalSet,
    /// Soft set.
    SoftSet,
    /// Drum set.
    DrumSet,
}

impl Default for SampleSet {
    fn default() -> Self {
        Self::NoCustomSampleSet
    }
}

impl Display for SampleSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as usize)
    }
}

impl FromStr for SampleSet {
    type Err = SampleSetParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.parse()?;
        SampleSet::from_repr(s).ok_or(SampleSetParseError::UnknownType(s))
    }
}

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
/// Volume of the sample from `1` to `100`. If [volume][Self::volume] returns `None`, the timing point's volume will be used instead.
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
    /// Creates a new instance of `Volume`.
    /// - Requires the `volume` to be in range of `1` ~ `100`.
    /// - Setting the `volume` to be `None` will use the timingpoint's volume instead.
    pub fn new(volume: Option<u8>) -> Result<Volume, VolumeParseError> {
        if let Some(volume) = volume {
            if volume == 0 {
                Err(VolumeParseError::VolumeTooLow)
            } else if volume > 100 {
                Err(VolumeParseError::VolumeTooHigh(volume))
            } else {
                Ok(Volume(Some(volume)))
            }
        } else {
            Ok(Volume(volume))
        }
    }

    /// Returns the `volume`.
    pub fn volume(&self) -> Option<u8> {
        self.0
    }

    /// Sets the `volume`.
    /// Requires to be in the boundary of `1` ~ `100`
    pub fn set_volume(&mut self, volume: u8) -> Result<(), VolumeSetError> {
        if volume == 0 {
            Err(VolumeSetError::VolumeTooLow)
        } else if volume > 100 {
            Err(VolumeSetError::VolumeTooHigh(volume))
        } else {
            self.0 = Some(volume);
            Ok(())
        }
    }

    /// Returns `true` if the timingpoint volume is used instead.
    pub fn use_timing_point_volume(&self) -> bool {
        self.0.is_none()
    }

    /// Sets if to use the timingpoint volume instead.
    pub fn set_use_timing_point_volume(&mut self) {
        self.0 = None;
    }
}

impl FromStr for Volume {
    type Err = VolumeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let volume = s.parse::<u8>()?;

        let volume = if volume == 0 { None } else { Some(volume) };

        Volume::new(volume)
    }
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
/// Flags that determine which sounds will play when the object is hit.
/// # Possible sounds
/// [`normal`][Self::normal] [`whistle`][Self::whistle] [`finish`][Self::finish] [`clap`][Self::clap]
/// - If no flags are set, it will default to using the `normal` sound.
/// - In every mode except osu!mania, the `LayeredHitSounds` skin property forces the `normal` sound to be used, and ignores those flags.
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

impl HitSound {
    pub fn new(normal: bool, whistle: bool, finish: bool, clap: bool) -> Self {
        Self {
            normal,
            whistle,
            finish,
            clap,
        }
    }

    /// Returns the `normal` flag. Also will return `true` if no sound flags are set.
    pub fn normal(&self) -> bool {
        if !self.normal && !self.whistle && !self.finish && !self.clap {
            true
        } else {
            self.normal
        }
    }
    /// Returns the `whistle` flag.
    pub fn whistle(&self) -> bool {
        self.whistle
    }
    /// Returns the `finish` flag.
    pub fn finish(&self) -> bool {
        self.finish
    }
    /// Returns the `clap` flag.
    pub fn clap(&self) -> bool {
        self.clap
    }

    /// Sets the `normal` flag.
    pub fn set_normal(&mut self, normal: bool) {
        self.normal = normal;
    }
    /// Sets the `whistle` flag.
    pub fn set_whistle(&mut self, whistle: bool) {
        self.whistle = whistle;
    }
    /// Sets the `finish` flag.
    pub fn set_finish(&mut self, finish: bool) {
        self.finish = finish;
    }
    /// Sets the `clap` flag.
    pub fn set_clap(&mut self, clap: bool) {
        self.clap = clap;
    }
}

impl FromStr for HitSound {
    type Err = HitSoundParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(HitSound::from(s.parse::<u8>()?))
    }
}

impl TryFrom<i32> for HitSound {
    type Error = HitSoundParseError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(HitSound::from(u8::try_from(value)?))
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

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, EnumString, Display)]
/// Type of curve used to construct the [`slider`][super::Slider].
pub enum CurveType {
    /// Bézier curve.
    #[strum(serialize = "B")]
    Bezier,
    /// Centripetal catmull-rom curve.
    #[strum(serialize = "C")]
    Centripetal,
    /// Linear curve.
    #[strum(serialize = "L")]
    Linear,
    /// Perfect circle curve.
    #[strum(serialize = "P")]
    PerfectCircle,
}

#[derive(Default, Clone, Debug, Hash, PartialEq, Eq)]
/// Information about which samples are played when the object is hit.
/// It is closely related to [`hitSound`][HitSound].
pub struct HitSample {
    normal_set: SampleSet,
    addition_set: SampleSet,
    index: Option<NonZeroUsize>,
    volume: Volume,
    filename: String,
}

impl Display for HitSample {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let index = match self.index {
            Some(index) => index.into(),
            None => 0,
        };
        let volume: Integer = self.volume.into();
        let filename = &self.filename;

        write!(
            f,
            "{}:{}:{index}:{volume}:{filename}",
            self.normal_set, self.addition_set
        )
    }
}

impl HitSample {
    pub fn new(
        normal_set: SampleSet,
        addition_set: SampleSet,
        index: Option<NonZeroUsize>,
        volume: Volume,
        filename: String,
    ) -> Self {
        Self {
            normal_set,
            addition_set,
            index,
            volume,
            filename,
        }
    }

    /// Returns the sample set of the normal sound.
    pub fn normal_set(&self) -> SampleSet {
        self.normal_set
    }

    /// Sets the sample set of the normal sound.
    pub fn set_normal_set(&mut self, normal_set: SampleSet) {
        self.normal_set = normal_set;
    }

    /// Returns the sample set of the whistle, finish, and clap sounds.
    pub fn addition_set(&self) -> SampleSet {
        self.addition_set
    }

    /// Sets the sample set of the whistle, finish, and clap sounds.
    pub fn set_addition_set(&mut self, addition_set: SampleSet) {
        self.addition_set = addition_set;
    }

    /// Index of the sample.
    /// - If this returns `None`, the timing point's sample index will be used instead.
    pub fn index(&self) -> Option<NonZeroUsize> {
        self.index
    }

    // TODO make custom type for those 0 index cases
    /// Sets the index of the sample.
    /// - If this returns `None`, the timing point's sample index will be used instead.
    pub fn set_index(&mut self, index: NonZeroUsize) {
        self.index = Some(index);
    }

    /// Returns `true` if the timing point index is used instead.
    pub fn use_timing_point_index(&self) -> bool {
        self.index.is_none()
    }

    /// Sets if the timing point index is to be used.
    pub fn set_use_timing_point_index(&mut self) {
        self.index = None;
    }

    /// Returns the reference to the hit sample's `volume`.
    pub fn volume(&self) -> &Volume {
        &self.volume
    }

    /// Get a mutable reference to the hit sample's `volume`.
    pub fn volume_mut(&mut self) -> &mut Volume {
        &mut self.volume
    }

    /// Returns the reference to the custom filename of the addition sound.
    pub fn filename(&self) -> &str {
        self.filename.as_ref()
    }

    /// Sets the custom filename of the addition sound.
    pub fn set_filename(&mut self, filename: &str) {
        self.filename = filename.to_owned();
    }
}

impl FromStr for HitSample {
    type Err = HitSampleParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(':');

        let sample_set_count = 2;
        let (normal_set, addition_set) = {
            let mut sample_sets = Vec::new();

            for i in 0..sample_set_count {
                let s = s.next().ok_or(HitSampleParseError::MissingProperty(i))?;

                sample_sets.push(s.parse().map_err(|err| HitSampleParseError::ParseError {
                    source: Box::new(err),
                    value: s.to_string(),
                })?);
            }

            (sample_sets[0], sample_sets[1])
        };

        let index = s.next().ok_or(HitSampleParseError::MissingProperty(2))?;
        let index = index
            .parse::<usize>()
            .map_err(|err| HitSampleParseError::ParseError {
                source: Box::new(err),
                value: index.to_string(),
            })?;
        let index = if index == 0 {
            None
        } else {
            // safe to unwrap since the 0 check is done before hand
            Some(NonZeroUsize::new(index).unwrap())
        };

        let volume = s.next().ok_or(HitSampleParseError::MissingProperty(3))?;
        let volume = volume
            .parse()
            .map_err(|err| HitSampleParseError::ParseError {
                source: Box::new(err),
                value: volume.to_string(),
            })?;

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