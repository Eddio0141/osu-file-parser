use std::{
    error::Error,
    fmt::Display,
    num::{NonZeroUsize, ParseIntError},
    str::{FromStr, Split},
};

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use strum_macros::FromRepr;
use thiserror::Error;

use crate::helper::{nth_bit_state_i64, parse_zero_one_bool};

use super::Integer;

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct TimingPoints(pub Vec<TimingPoint>);

impl FromStr for TimingPoints {
    type Err = TimingPointsParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut timing_points = Vec::new();

        for s in s.lines() {
            if !s.is_empty() {
                timing_points.push(s.parse()?);
            }
        }

        Ok(TimingPoints(timing_points))
    }
}

impl Display for TimingPoints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[derive(Debug, Error)]
#[error(transparent)]
pub struct TimingPointsParseError(#[from] TimingPointParseError);

/// Struct representing a timing point.
/// Each timing point influences a specified portion of the map, commonly called a `timing section`.
/// The .osu file format requires these to be sorted in chronological order.
#[derive(Default, Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct TimingPoint {
    time: Integer,
    beat_length: Decimal,
    meter: Integer,
    sample_set: SampleSet,
    sample_index: SampleIndex,
    volume: Volume,
    uninherited: bool,
    effects: Effects,
}

impl TimingPoint {
    /// Converts beat duration in milliseconds to BPM.
    pub fn beat_duration_ms_to_bpm(beat_duration_ms: Decimal) -> Decimal {
        Decimal::ONE / beat_duration_ms * dec!(60000)
    }

    /// Converts BPM to beat duration in milliseconds.
    pub fn bpm_to_beat_duration_ms(bpm: Decimal) -> Decimal {
        Decimal::ONE / (bpm / dec!(60000))
    }

    /// New instance of `TimingPoint` that is inherited.
    pub fn new_inherited(
        time: Integer,
        slider_velocity_multiplier: Decimal,
        meter: Integer,
        sample_set: SampleSet,
        sample_index: SampleIndex,
        volume: Volume,
        effects: Effects,
    ) -> Self {
        Self {
            time,
            beat_length: (Decimal::ONE / slider_velocity_multiplier) * dec!(-100),
            meter,
            sample_set,
            sample_index,
            volume,
            uninherited: false,
            effects,
        }
    }

    /// New instance of `TimingPoint` that is uninherited.
    pub fn new_uninherited(
        time: Integer,
        beat_duration_ms: Decimal,
        meter: Integer,
        sample_set: SampleSet,
        sample_index: SampleIndex,
        volume: Volume,
        effects: Effects,
    ) -> Self {
        Self {
            time,
            beat_length: beat_duration_ms,
            meter,
            sample_set,
            sample_index,
            volume,
            uninherited: true,
            effects,
        }
    }

    /// Calculates BPM using the `beatLength` field when unherited.
    pub fn calc_bpm(&self) -> Option<Decimal> {
        if self.uninherited {
            Some(Self::beat_duration_ms_to_bpm(self.beat_length))
        } else {
            None
        }
    }
    /// Calculates the slider velocity multiplier when the timing point is inherited.
    pub fn slider_velocity_multiplier(&self) -> Option<Decimal> {
        if self.uninherited {
            None
        } else {
            Some(Decimal::ONE / (self.beat_length / dec!(-100)))
        }
    }

    /// Start time of the timing section, in milliseconds from the beginning of the beatmap's audio.
    /// The end of the timing section is the next timing point's time (or never, if this is the last timing point).
    pub fn time(&self) -> i32 {
        self.time
    }

    /// Set the timing point's start time.
    pub fn set_time(&mut self, time: Integer) {
        self.time = time;
    }

    /// Amount of beats in a measure. Inherited timing points ignore this property.
    pub fn meter(&self) -> i32 {
        self.meter
    }

    /// Set the timing point's meter field.
    pub fn set_meter(&mut self, meter: Integer) {
        self.meter = meter;
    }

    /// Default sample set for hit objects
    pub fn sample_set(&self) -> SampleSet {
        self.sample_set
    }

    /// Set the timing point's sample set.
    pub fn set_sample_set(&mut self, sample_set: SampleSet) {
        self.sample_set = sample_set;
    }

    /// Custom sample index for hit objects.
    pub fn sample_index(&self) -> SampleIndex {
        self.sample_index
    }

    /// Get a mutable reference to the timing point's sample index.
    pub fn sample_index_mut(&mut self) -> &mut SampleIndex {
        &mut self.sample_index
    }

    /// Volume percentage for hit objects.
    pub fn volume(&self) -> &Volume {
        &self.volume
    }

    /// Set the timing point's volume.
    pub fn set_volume(&mut self, volume: Volume) {
        self.volume = volume;
    }

    /// Get the timing point's uninherited.
    pub fn uninherited(&self) -> bool {
        self.uninherited
    }

    /// Set the timing point's uninherited.
    pub fn set_uninherited(&mut self, uninherited: bool) {
        self.uninherited = uninherited;
    }

    /// Get the timing point's effects.
    pub fn effects(&self) -> Effects {
        self.effects
    }

    /// Get a mutable reference to the timing point's effects.
    pub fn effects_mut(&mut self) -> &mut Effects {
        &mut self.effects
    }

    fn parse_field<T>(
        s: &mut Split<char>,
        field_name: &'static str,
    ) -> Result<T, TimingPointParseError>
    where
        T: FromStr,
        <T as FromStr>::Err: Error + 'static,
    {
        let value = s
            .next()
            .ok_or(TimingPointParseError::MissingField(field_name))?;
        value
            .parse()
            .map_err(|err| TimingPointParseError::FieldParseError {
                source: Box::new(err),
                value: value.to_string(),
                field_name,
            })
    }
}

impl FromStr for TimingPoint {
    type Err = TimingPointParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(',');

        let time = Self::parse_field(&mut s, "time")?;
        let beat_length = Self::parse_field(&mut s, "beatLength")?;
        let meter = Self::parse_field(&mut s, "meter")?;
        let sample_set = Self::parse_field(&mut s, "sampleSet")?;
        let sample_index = Self::parse_field(&mut s, "sampleIndex")?;
        let volume = Self::parse_field(&mut s, "volume")?;
        let uninherited = s
            .next()
            .ok_or(TimingPointParseError::MissingField("uninherited"))?;
        let uninherited = parse_zero_one_bool(uninherited).map_err(|err| {
            TimingPointParseError::FieldParseError {
                source: Box::new(err),
                value: uninherited.to_string(),
                field_name: "uninherited",
            }
        })?;
        let effects = Self::parse_field(&mut s, "effects")?;

        Ok(TimingPoint {
            time,
            beat_length,
            meter,
            sample_set,
            sample_index,
            volume,
            uninherited,
            effects,
        })
    }
}

impl Display for TimingPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fields = vec![
            self.time.to_string(),
            self.beat_length.to_string(),
            self.meter.to_string(),
            self.sample_set.to_string(),
            self.sample_index.to_string(),
            self.volume.to_string(),
            // TODO check all bool types to be integer
            (self.uninherited as u8).to_string(),
            self.effects.to_string(),
        ];

        write!(f, "{}", fields.join(","))
    }
}

/// Error used when there was a problem parsing the [`TimingPoint`].
#[derive(Debug, Error)]
pub enum TimingPointParseError {
    /// A field is missing.
    #[error("The field {0} is missing")]
    MissingField(&'static str),
    #[error("There was a problem parsing the value {value}")]
    /// The field failed to parse to some type from a `str`.
    FieldParseError {
        #[source]
        source: Box<dyn Error>,
        value: String,
        field_name: &'static str,
    },
}

// TODO figure out default
/// Default sample set for hitobjects.
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug, FromRepr)]
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

impl Default for SampleSet {
    fn default() -> Self {
        Self::BeatmapDefault
    }
}

impl FromStr for SampleSet {
    type Err = SampleSetParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .parse()
            .map_err(|err| SampleSetParseError::ValueParseError {
                source: err,
                value: s.to_string(),
            })?;

        SampleSet::from_repr(s).ok_or(SampleSetParseError::UnknownSampleSet(s))
    }
}

impl Display for SampleSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

/// There was some problem parsing the [`SampleSet`].
#[derive(Debug, Error, PartialEq)]
pub enum SampleSetParseError {
    /// The value failed to parse from a `str`.
    #[error("There was a problem parsing {value} as an `Integer`")]
    ValueParseError {
        #[source]
        source: ParseIntError,
        value: String,
    },
    /// The `SampleSet` type is invalid.
    #[error("Expected `SampleSet` to have a value of 0 ~ 3, got {0}")]
    UnknownSampleSet(usize),
}

// TODO figure out default
/// Flags that give the [`TimingPoint`] extra effects.
#[derive(Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Effects {
    pub kiai_time_enabled: bool,
    pub no_first_barline_in_taiko_mania: bool,
}

impl FromStr for Effects {
    type Err = EffectsParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.parse::<u8>().map_err(|err| EffectsParseError {
            source: err,
            value: s.to_string(),
        })?;

        Ok(Self::from(s))
    }
}

impl From<u8> for Effects {
    fn from(value: u8) -> Self {
        let kiai_time_enabled = nth_bit_state_i64(value as i64, 0);
        let no_first_barline_in_taiko_mania = nth_bit_state_i64(value as i64, 3);

        Self {
            kiai_time_enabled,
            no_first_barline_in_taiko_mania,
        }
    }
}

impl From<Effects> for u8 {
    fn from(effects: Effects) -> Self {
        let mut bit_flag = 0;

        if effects.kiai_time_enabled {
            bit_flag |= 0b1;
        }
        if effects.no_first_barline_in_taiko_mania {
            bit_flag |= 0b100;
        }

        bit_flag
    }
}

impl Display for Effects {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", u8::from(*self))
    }
}

/// There was a problem parsing `str` as [`Effects`].
#[derive(Debug, Error)]
#[error("There was a problem parsing {value} as an `Integer`")]
pub struct EffectsParseError {
    #[source]
    pub source: ParseIntError,
    pub value: String,
}

/// Custom sample index for hitobjects.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SampleIndex {
    /// Osu!'s default hitsounds.
    OsuDefaultHitsounds,
    /// Sample index for hitobjects.
    Index(NonZeroUsize),
}

impl FromStr for SampleIndex {
    type Err = SampleIndexParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        SampleIndex::try_from(s.parse::<Integer>().map_err(|err| SampleIndexParseError {
            source: Box::new(err),
            value: s.to_string(),
        })?)
    }
}

// TODO do we accept negative index
impl TryFrom<Integer> for SampleIndex {
    type Error = SampleIndexParseError;

    fn try_from(value: Integer) -> Result<Self, Self::Error> {
        let value = usize::try_from(value).map_err(|err| SampleIndexParseError {
            source: Box::new(err),
            value: value.to_string(),
        })?;

        if value == 0 {
            Ok(SampleIndex::OsuDefaultHitsounds)
        } else {
            Ok(SampleIndex::Index(NonZeroUsize::try_from(value).unwrap()))
        }
    }
}

impl From<SampleIndex> for Integer {
    fn from(sample_index: SampleIndex) -> Self {
        match sample_index {
            SampleIndex::OsuDefaultHitsounds => 0,
            SampleIndex::Index(sample_index) => sample_index.get() as Integer,
        }
    }
}

impl Display for SampleIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Integer::from(*self))
    }
}

impl Default for SampleIndex {
    fn default() -> Self {
        Self::OsuDefaultHitsounds
    }
}

/// Error used when `str` failed to parse as [`SampleIndex`].
#[derive(Debug, Error)]
#[error("There was a problem parsing {value} as an usize for `SampleIndex`")]
pub struct SampleIndexParseError {
    #[source]
    pub source: Box<dyn Error>,
    pub value: String,
}

/// The volume percentage in the range of 0 ~ 100.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Volume(u8);

impl FromStr for Volume {
    type Err = VolumeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: u8 = s.parse().map_err(|err| VolumeError::VolumeParseError {
            source: err,
            value: s.to_string(),
        })?;
        Volume::try_from(s)
    }
}

impl TryFrom<u8> for Volume {
    type Error = VolumeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 100 {
            Err(VolumeError::VolumeTooHigh(value))
        } else {
            Ok(Volume(value))
        }
    }
}

impl From<Volume> for u8 {
    fn from(volume: Volume) -> Self {
        volume.0
    }
}

impl Display for Volume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Volume {
    /// Creates a new volme instance.
    pub fn new(volume: u8) -> Result<Self, VolumeError> {
        if volume > 100 {
            Err(VolumeError::VolumeTooHigh(volume))
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
            Err(VolumeError::VolumeTooHigh(volume))
        } else {
            self.0 = volume;
            Ok(())
        }
    }
}

/// Error for when there was a problem setting / parsing the volume.
#[derive(Debug, Error, PartialEq)]
pub enum VolumeError {
    /// Error when volume is out of range of the 0 ~ 100 range.
    #[error("The volume was too high, expected 0 ~ 100, got {0}")]
    VolumeTooHigh(u8),
    /// There was a problem parsing the `str` as [`Volume`].
    #[error("There was a problem parsing a `str` as [`Volume`]")]
    VolumeParseError {
        #[source]
        source: ParseIntError,
        value: String,
    },
}
