use std::num::NonZeroU32;

use super::*;
use crate::osu_file::{InvalidRepr, VersionedFromRepr, VersionedFromStr};

/// Default sample set for hitobjects.
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
#[non_exhaustive]
pub enum SampleSet {
    /// Beatmap's default.
    BeatmapDefault,
    /// Normal.
    Normal,
    /// Soft.
    Soft,
    /// Drum.
    Drum,
}

impl VersionedFromRepr for SampleSet {
    fn from_repr(repr: usize, _: Version) -> Result<Option<Self>, InvalidRepr> {
        match repr {
            0 => Ok(Some(SampleSet::BeatmapDefault)),
            1 => Ok(Some(SampleSet::Normal)),
            2 => Ok(Some(SampleSet::Soft)),
            3 => Ok(Some(SampleSet::Drum)),
            _ => Err(InvalidRepr),
        }
    }
}

impl VersionedFromStr for SampleSet {
    type Err = ParseSampleSetError;

    fn from_str(s: &str, version: Version) -> Result<Option<Self>, Self::Err> {
        SampleSet::from_repr(s.parse()?, version).map_err(|_| ParseSampleSetError::UnknownSampleSet)
    }
}

impl VersionedToString for SampleSet {
    fn to_string(&self, _: Version) -> Option<String> {
        Some((*self as u8).to_string())
    }
}

/// Flags that give the [`TimingPoint`] extra effects.
/// - It will keep the original value stored containing the unused bits.
/// - The unused bits will come in effect when to_string is called.
/// - You can clear the unused bits by calling [`Effects::clear_unused_bits`].
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Effects(u32);

impl VersionedFromStr for Effects {
    type Err = ParseEffectsError;

    fn from_str(s: &str, version: Version) -> Result<Option<Self>, Self::Err> {
        Ok(<Effects as VersionedFrom<u32>>::from(s.parse()?, version))
    }
}

impl VersionedFrom<u32> for Effects {
    fn from(value: u32, _: Version) -> Option<Self> {
        Some(Self(value))
    }
}

impl VersionedFrom<Effects> for u32 {
    fn from(effects: Effects, _: Version) -> Option<Self> {
        Some(effects.0)
    }
}

impl VersionedToString for Effects {
    fn to_string(&self, version: Version) -> Option<String> {
        <u32 as VersionedFrom<Effects>>::from(*self, version).map(|effects| effects.to_string())
    }
}

impl Effects {
    /// Clears the unused bits.
    /// - This will keep the `kiai_time_enabled` and `no_first_barline_in_taiko_mania` flags untouched.
    pub fn clear_unused_bits(&mut self) {
        self.0 &= 0b1001;
    }

    pub fn kiai_time_enabled(&self) -> bool {
        self.0 & 0b1 == 0b1
    }

    pub fn no_first_barline_in_taiko_mania(&self) -> bool {
        self.0 & 0b1000 == 0b1000
    }

    pub fn set_kiai_time_enabled(&mut self, value: bool) {
        if value {
            self.0 |= 0b1;
        } else {
            self.0 &= !0b1;
        }
    }

    pub fn set_no_first_barline_in_taiko_mania(&mut self, value: bool) {
        if value {
            self.0 |= 0b1000;
        } else {
            self.0 &= !0b1000;
        }
    }

    pub fn new(kiai_time_enabled: bool, no_first_barline_in_taiko_mania: bool) -> Self {
        let mut effects = 0;
        if kiai_time_enabled {
            effects |= 0b1;
        }
        if no_first_barline_in_taiko_mania {
            effects |= 0b1000;
        }
        Self(effects)
    }
}

/// Custom sample index for hitobjects.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[non_exhaustive]
pub enum SampleIndex {
    /// Osu!'s default hitsounds.
    OsuDefaultHitsounds,
    /// Sample index for hitobjects.
    Index(NonZeroU32),
}

impl VersionedFromStr for SampleIndex {
    type Err = ParseSampleIndexError;

    fn from_str(s: &str, version: Version) -> Result<Option<Self>, Self::Err> {
        Ok(<SampleIndex as VersionedFrom<u32>>::from(
            s.parse()?,
            version,
        ))
    }
}

impl VersionedFrom<u32> for SampleIndex {
    fn from(value: u32, _: Version) -> Option<Self> {
        let index = if value == 0 {
            SampleIndex::OsuDefaultHitsounds
        } else {
            SampleIndex::Index(NonZeroU32::try_from(value).unwrap())
        };

        Some(index)
    }
}

impl VersionedFrom<SampleIndex> for Integer {
    fn from(sample_index: SampleIndex, _: Version) -> Option<Self> {
        let sample_index = match sample_index {
            SampleIndex::OsuDefaultHitsounds => 0,
            SampleIndex::Index(sample_index) => sample_index.get() as Integer,
        };

        Some(sample_index)
    }
}

impl VersionedToString for SampleIndex {
    fn to_string(&self, version: Version) -> Option<String> {
        <i32 as VersionedFrom<SampleIndex>>::from(*self, version).map(|index| index.to_string())
    }
}

impl VersionedDefault for SampleIndex {
    fn default(_: Version) -> Option<Self> {
        Some(Self::OsuDefaultHitsounds)
    }
}

/// The volume percentage in the range of 0 ~ 100.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Volume(i8);

impl VersionedFromStr for Volume {
    type Err = VolumeError;

    fn from_str(s: &str, version: Version) -> Result<Option<Self>, Self::Err> {
        Ok(<Volume as VersionedFrom<i8>>::from(s.parse()?, version))
    }
}

impl VersionedFrom<i8> for Volume {
    fn from(value: i8, _: Version) -> Option<Self> {
        Some(Volume(value))
    }
}

impl VersionedFrom<Volume> for i8 {
    fn from(volume: Volume, _: Version) -> Option<Self> {
        Some(volume.0)
    }
}

impl VersionedToString for Volume {
    fn to_string(&self, _: Version) -> Option<String> {
        Some(self.0.to_string())
    }
}

impl Volume {
    /// Creates a new volume instance.
    pub fn new(volume: i8, version: Version) -> Option<Self> {
        <Self as VersionedFrom<i8>>::from(volume, version)
    }

    /// Gets the volume percentage.
    pub fn volume(&self) -> i8 {
        self.0
    }

    /// Sets the volume percentage.
    pub fn set_volume(&mut self, volume: i8) {
        self.0 = volume;
    }
}
