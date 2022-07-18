use std::num::NonZeroU32;

use super::*;
use crate::{
    helper::nth_bit_state_i64,
    osu_file::{InvalidRepr, VersionedFromRepr, VersionedFromStr},
};

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
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Effects {
    pub kiai_time_enabled: bool,
    pub no_first_barline_in_taiko_mania: bool,
}

impl VersionedFromStr for Effects {
    type Err = ParseEffectsError;

    fn from_str(s: &str, version: Version) -> Result<Option<Self>, Self::Err> {
        Ok(<Effects as VersionedFrom<u8>>::from(s.parse()?, version))
    }
}

impl VersionedFrom<u8> for Effects {
    fn from(value: u8, _: Version) -> Option<Self> {
        let kiai_time_enabled = nth_bit_state_i64(value as i64, 0);
        let no_first_barline_in_taiko_mania = nth_bit_state_i64(value as i64, 3);

        Some(Self {
            kiai_time_enabled,
            no_first_barline_in_taiko_mania,
        })
    }
}

impl VersionedFrom<Effects> for u8 {
    fn from(effects: Effects, _: Version) -> Option<Self> {
        let mut bit_flag = 0;

        if effects.kiai_time_enabled {
            bit_flag |= 0b1;
        }
        if effects.no_first_barline_in_taiko_mania {
            bit_flag |= 0b100;
        }

        Some(bit_flag)
    }
}

impl VersionedToString for Effects {
    fn to_string(&self, version: Version) -> Option<String> {
        <u8 as VersionedFrom<Effects>>::from(*self, version).map(|effects| effects.to_string())
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
pub struct Volume(u8);

impl VersionedFromStr for Volume {
    type Err = VolumeError;

    fn from_str(s: &str, version: Version) -> Result<Option<Self>, Self::Err> {
        <Volume as VersionedTryFrom<u8>>::try_from(s.parse()?, version)
    }
}

impl VersionedTryFrom<u8> for Volume {
    type Error = VolumeError;

    fn try_from(value: u8, _: Version) -> Result<Option<Self>, Self::Error> {
        if value > 100 {
            Err(VolumeError::VolumeTooHigh)
        } else {
            Ok(Some(Volume(value)))
        }
    }
}

impl VersionedFrom<Volume> for u8 {
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
    /// Creates a new volme instance.
    pub fn new(volume: u8) -> Result<Self, VolumeError> {
        if volume > 100 {
            Err(VolumeError::VolumeTooHigh)
        } else {
            Ok(Volume(volume))
        }
    }

    /// Gets the volume percentage.
    pub fn volume(&self) -> u8 {
        self.0
    }

    /// Sets the volume percentage.
    /// - Volume is in the range of 0 ~ 100 and setting it above 100 will return a [`VolumeError`].
    pub fn set_volume(&mut self, volume: u8) -> Result<(), VolumeError> {
        if volume > 100 {
            Err(VolumeError::VolumeTooHigh)
        } else {
            self.0 = volume;
            Ok(())
        }
    }
}
