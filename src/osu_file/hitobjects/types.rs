//! Module defining misc types used for different hitobjects, such as [CurveType] used for [`Slider`][super::SlideParams] curve types.

use std::num::{NonZeroUsize, ParseIntError};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    combinator::map_res,
    error::context,
    sequence::{preceded, tuple},
    Parser,
};

use crate::{helper::nth_bit_state_i64, osu_file::*, parsers::nothing};

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

impl VersionedFromStr for EdgeSet {
    type Err = ColonSetParseError;

    fn from_str(s: &str, version: usize) -> Result<Option<Self>, Self::Err> {
        let s = s.split(':').collect::<Vec<_>>();

        if s.len() > 2 {
            return Err(ColonSetParseError::MoreThanTwoItems(s[2..].join(":")));
        }

        let normal_set = s.get(0).ok_or(ColonSetParseError::MissingFirstItem)?;
        let addition_set = s.get(1).ok_or(ColonSetParseError::MissingSecondItem)?;

        let normal_set = SampleSet::from_str(normal_set, version).map_err(|err| {
            ColonSetParseError::ValueParseError {
                source: Box::new(err),
                value: normal_set.to_string(),
            }
        })?;
        let addition_set = SampleSet::from_str(addition_set, version).map_err(|err| {
            ColonSetParseError::ValueParseError {
                source: Box::new(err),
                value: addition_set.to_string(),
            }
        })?;

        Ok(Some(Self {
            normal_set: normal_set.unwrap(),
            addition_set: addition_set.unwrap(),
        }))
    }
}

impl VersionedToString for EdgeSet {
    fn to_string(&self, version: usize) -> Option<String> {
        Some(format!(
            "{}:{}",
            self.normal_set.to_string(version).unwrap(),
            self.addition_set.to_string(version).unwrap()
        ))
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
/// Anchor point used to construct the [`slider`][super::SlideParams].
pub struct CurvePoint(pub Position);

impl VersionedFromStr for CurvePoint {
    type Err = ColonSetParseError;

    fn from_str(s: &str, _: usize) -> Result<Option<Self>, Self::Err> {
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

        Ok(Some(Self(position)))
    }
}

impl VersionedToString for CurvePoint {
    fn to_string(&self, _: usize) -> Option<String> {
        Some(format!("{}:{}", self.0.x, self.0.y))
    }
}

// TODO all enum funcs that converts to string or usize should be moved to the impl of the enum
/// Used for `normal_set` and `addition_set` for the `[hitobject]`[super::HitObject].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
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

impl VersionedToString for SampleSet {
    fn to_string(&self, _: usize) -> Option<String> {
        Some((*self as usize).to_string())
    }
}

impl VersionedFromStr for SampleSet {
    type Err = SampleSetParseError;

    fn from_str(s: &str, _: usize) -> Result<Option<Self>, Self::Err> {
        let s = s.parse()?;

        match s {
            0 => Ok(Some(Self::NoCustomSampleSet)),
            1 => Ok(Some(Self::NormalSet)),
            2 => Ok(Some(Self::SoftSet)),
            3 => Ok(Some(Self::DrumSet)),
            _ => Err(SampleSetParseError::UnknownType(s)),
        }
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

impl VersionedFromStr for Volume {
    type Err = VolumeParseError;

    fn from_str(s: &str, _: usize) -> Result<Option<Self>, Self::Err> {
        let volume = s.parse::<u8>()?;

        let volume = if volume == 0 { None } else { Some(volume) };

        Volume::new(volume).map(Some)
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

impl VersionedToString for HitSound {
    fn to_string(&self, _: usize) -> Option<String> {
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

        Some(bit_mask.to_string())
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

impl VersionedFromStr for HitSound {
    type Err = HitSoundParseError;

    fn from_str(s: &str, _: usize) -> Result<Option<Self>, Self::Err> {
        Ok(Some(HitSound::from(s.parse::<u8>()?)))
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

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
#[non_exhaustive]
/// Type of curve used to construct the [`Slider`][super::SlideParams].
pub enum CurveType {
    /// Bézier curve.
    Bezier,
    /// Centripetal catmull-rom curve.
    Centripetal,
    /// Linear curve.
    Linear,
    /// Perfect circle curve.
    PerfectCircle,
}

impl VersionedFromStr for CurveType {
    type Err = CurveTypeParseError;

    fn from_str(s: &str, _: usize) -> std::result::Result<Option<Self>, Self::Err> {
        match s {
            "Bezier" => Ok(Some(CurveType::Bezier)),
            "Centripetal" => Ok(Some(CurveType::Centripetal)),
            "Linear" => Ok(Some(CurveType::Linear)),
            "PerfectCircle" => Ok(Some(CurveType::PerfectCircle)),
            _ => Err(CurveTypeParseError::UnknownVariant),
        }
    }
}

impl VersionedToString for CurveType {
    fn to_string(&self, _: usize) -> Option<String> {
        let curve = match self {
            CurveType::Bezier => "B",
            CurveType::Centripetal => "C",
            CurveType::Linear => "L",
            CurveType::PerfectCircle => "P",
        };

        Some(curve.to_string())
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum SampleIndex {
    TimingPointSampleIndex,
    Index(NonZeroUsize),
}

impl Default for SampleIndex {
    fn default() -> Self {
        Self::TimingPointSampleIndex
    }
}

impl From<usize> for SampleIndex {
    fn from(index: usize) -> Self {
        if index == 0 {
            Self::TimingPointSampleIndex
        } else {
            Self::Index(NonZeroUsize::new(index).unwrap())
        }
    }
}

impl From<SampleIndex> for usize {
    fn from(index: SampleIndex) -> Self {
        match index {
            SampleIndex::TimingPointSampleIndex => 0,
            SampleIndex::Index(index) => index.get(),
        }
    }
}

impl VersionedToString for SampleIndex {
    fn to_string(&self, _: usize) -> Option<String> {
        Some(usize::from(*self).to_string())
    }
}

impl VersionedFromStr for SampleIndex {
    type Err = ParseIntError;

    fn from_str(s: &str, _: usize) -> Result<Option<Self>, Self::Err> {
        Ok(Some(s.parse::<usize>()?.into()))
    }
}

#[derive(Default, Clone, Debug, Hash, PartialEq, Eq)]
/// Information about which samples are played when the object is hit.
/// It is closely related to [`hitSound`][HitSound].
pub struct HitSample {
    pub normal_set: SampleSet,
    pub addition_set: SampleSet,
    pub index: SampleIndex,
    pub volume: Volume,
    pub filename: String,
}

impl VersionedFromStr for HitSample {
    type Err = HitSampleParseError;

    fn from_str(s: &str, version: usize) -> std::result::Result<Option<Self>, Self::Err> {
        // TODO does the 4th and 5th field exist from v12 onwards?
        let field = || take_while(|c| c != ':');
        let field_separator = || tag(":");
        let sample_set = || {
            context(
                HitSampleParseError::InvalidSampleSet.into(),
                map_res(field(), |v: &str| {
                    SampleSet::from_str(v, version).map(|s| s.unwrap())
                })
                .map(Some),
            )
        };

        let (_, hitsample) = alt((
            nothing().map(|_| <HitSample as VersionedDefault>::default(version).unwrap()),
            tag("0:0:0:0:").map(|_| <HitSample as VersionedDefault>::default(version).unwrap()),
            tuple((
                // normal_set
                alt((nothing().map(|_| None), sample_set())),
                // addition_set
                alt((
                    preceded(field_separator(), nothing()).map(|_| None),
                    nothing().map(|_| None),
                    preceded(
                        context(
                            HitSampleParseError::MissingSeparator.into(),
                            field_separator(),
                        ),
                        sample_set(),
                    ),
                )),
                // index
                alt((
                    preceded(field_separator(), nothing()).map(|_| None),
                    nothing().map(|_| None),
                    preceded(
                        context(
                            HitSampleParseError::MissingSeparator.into(),
                            field_separator(),
                        ),
                        context(
                            HitSampleParseError::InvalidIndex.into(),
                            map_res(field(), |v: &str| {
                                SampleIndex::from_str(v, version).map(|i| i.unwrap())
                            })
                            .map(Some),
                        ),
                    ),
                )),
                // volume
                alt((
                    preceded(field_separator(), nothing()).map(|_| None),
                    nothing().map(|_| None),
                    preceded(
                        context(
                            HitSampleParseError::MissingSeparator.into(),
                            field_separator(),
                        ),
                        context(
                            HitSampleParseError::InvalidVolume.into(),
                            map_res(field(), |v: &str| {
                                Volume::from_str(v, version).map(|v| v.unwrap())
                            })
                            .map(Some),
                        ),
                    ),
                )),
                // filename
                alt((
                    preceded(field_separator(), nothing()).map(|_| None),
                    nothing().map(|_| None),
                    preceded(
                        context(
                            HitSampleParseError::MissingSeparator.into(),
                            field_separator(),
                        ),
                        field().map(Some),
                    ),
                )),
            ))
            .map(|(normal_set, addition_set, index, volume, filename)| {
                let normal_set = normal_set.unwrap_or_default();
                let addition_set = addition_set.unwrap_or_default();
                let index = index.unwrap_or_default();
                let volume = volume.unwrap_or_default();
                let filename = filename.unwrap_or_default().to_string();

                HitSample {
                    normal_set,
                    addition_set,
                    index,
                    volume,
                    filename,
                }
            }),
        ))(s)?;

        Ok(Some(hitsample))
    }
}

impl VersionedToString for HitSample {
    fn to_string(&self, version: usize) -> Option<String> {
        let volume: Integer = self.volume.into();
        let filename = &self.filename;

        match version {
            MIN_VERSION..=9 => None,
            10..=11 => Some(format!(
                "{}:{}:{}",
                self.normal_set.to_string(version).unwrap(),
                self.addition_set.to_string(version).unwrap(),
                self.index.to_string(version).unwrap()
            )),
            _ => Some(format!(
                "{}:{}:{}:{volume}:{filename}",
                self.normal_set.to_string(version).unwrap(),
                self.addition_set.to_string(version).unwrap(),
                self.index.to_string(version).unwrap()
            )),
        }
    }
}

impl VersionedDefault for HitSample {
    fn default(_: usize) -> Option<Self> {
        Some(<Self as Default>::default())
    }
}
