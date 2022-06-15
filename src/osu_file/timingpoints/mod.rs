pub mod error;

use std::{fmt::Display, num::NonZeroUsize, str::FromStr};

use nom::{
    branch::alt,
    combinator::{cut, map_res, success, verify},
    error::context,
    sequence::{preceded, tuple},
    Parser,
};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use strum_macros::FromRepr;

use crate::{
    helper::{nth_bit_state_i64, parse_zero_one_bool},
    parsers::*,
};

use super::{Error, Integer, Version};

pub use self::error::*;

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct TimingPoints(pub Vec<TimingPoint>);

impl Version for TimingPoints {
    type ParseError = Error<ParseError>;

    // TODO versions
    fn from_str(s: &str, version: usize) -> std::result::Result<Option<Self>, Self::ParseError> {
        let mut timing_points = Vec::new();

        for (line_index, s) in s.lines().enumerate() {
            if !s.is_empty() {
                timing_points.push(Error::new_from_result_into(
                    TimingPoint::from_str(s, version),
                    line_index,
                )?);
            }
        }

        if let Some(s) = timing_points.get(0) {
            if s.is_some() {
                Ok(Some(TimingPoints(
                    timing_points.iter().map(|v| v.unwrap()).collect::<Vec<_>>(),
                )))
            } else {
                Ok(None)
            }
        } else {
            Ok(Some(TimingPoints(Vec::new())))
        }
    }

    fn to_string(&self, version: usize) -> Option<String> {
        // TODO add this as trait extension to Vec with Display impl
        let s = self
            .0
            .iter()
            .map(|p| p.to_string(version))
            .collect::<Vec<_>>();

        if let Some(v) = s.get(0) {
            if v.is_some() {
                Some(
                    s.into_iter()
                        .map(|v| v.unwrap())
                        .collect::<Vec<_>>()
                        .join("\n"),
                )
            } else {
                None
            }
        } else {
            Some(String::new())
        }
    }

    fn default(_: usize) -> Option<Self> {
        Some(TimingPoints(Vec::new()))
    }
}

/// Struct representing a timing point.
/// Each timing point influences a specified portion of the map, commonly called a `timing section`.
/// The .osu file format requires these to be sorted in chronological order.
#[derive(Default, Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct TimingPoint {
    // for some reason decimal is parsed anyway in the beatmap???
    time: Decimal,
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
            time: time.into(),
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
            time: time.into(),
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
    pub fn time(&self) -> Decimal {
        self.time
    }

    /// Set the timing point's start time.
    pub fn set_time(&mut self, time: Integer) {
        self.time = time.into();
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
}

const OLD_VERSION_TIME_OFFSET: Decimal = dec!(24);

impl Version for TimingPoint {
    type ParseError = TimingPointParseError;

    fn from_str(s: &str, version: usize) -> std::result::Result<Option<Self>, Self::ParseError> {
        let (
            _,
            (
                time,
                beat_length,
                (meter, sample_set, sample_index, (volume, (uninherited, effects))),
            ),
        ) = tuple((
            context(
                TimingPointParseError::InvalidTime.into(),
                comma_field_type(),
            )
            .map(|t| {
                if (3..=4).contains(&version) {
                    t + OLD_VERSION_TIME_OFFSET
                } else {
                    t
                }
            }),
            preceded(
                context(TimingPointParseError::MissingBeatLength.into(), comma()),
                context(
                    TimingPointParseError::InvalidBeatLength.into(),
                    comma_field_type(),
                ),
            ),
            // in v3, rest of the fields doesn't exist
            alt((
                nothing()
                    .and(context(
                        TimingPointParseError::InvalidBeatLength.into(),
                        cut(verify(success(0), |_| version == 3)),
                    ))
                    .map(|_| {
                        (
                            // TODO default values?
                            // TODO consume rest instead of handling like this
                            4,
                            SampleSet::Normal,
                            SampleIndex::Index(NonZeroUsize::new(1).unwrap()),
                            (Volume::try_from(100).unwrap(), (true, Effects::from(0))),
                        )
                    }),
                tuple((
                    preceded(
                        context(TimingPointParseError::MissingMeter.into(), comma()),
                        context(
                            TimingPointParseError::InvalidMeter.into(),
                            comma_field_type(),
                        ),
                    ),
                    preceded(
                        context(TimingPointParseError::MissingSampleSet.into(), comma()),
                        context(
                            TimingPointParseError::InvalidSampleSet.into(),
                            comma_field_type(),
                        ),
                    ),
                    preceded(
                        context(TimingPointParseError::MissingSampleIndex.into(), comma()),
                        context(
                            TimingPointParseError::InvalidSampleIndex.into(),
                            comma_field_type(),
                        ),
                    ),
                    alt((
                        nothing()
                            .and(context(
                                TimingPointParseError::InvalidSampleIndex.into(),
                                cut(verify(success(0), |_| version == 4)),
                            ))
                            .map(|_| {
                                (
                                    // TODO default values?
                                    Volume::try_from(100).unwrap(),
                                    (true, Effects::from(0)),
                                )
                            }),
                        tuple((
                            preceded(
                                context(TimingPointParseError::MissingVolume.into(), comma()),
                                context(
                                    TimingPointParseError::InvalidVolume.into(),
                                    comma_field_type(),
                                ),
                            ),
                            alt((
                                nothing()
                                    .and(context(
                                        TimingPointParseError::InvalidVolume.into(),
                                        cut(verify(success(0), |_| version == 5)),
                                    ))
                                    .map(|_| {
                                        (
                                            // TODO default values?
                                            true,
                                            Effects::from(0),
                                        )
                                    }),
                                tuple((
                                    preceded(
                                        context(
                                            TimingPointParseError::MissingUninherited.into(),
                                            comma(),
                                        ),
                                        context(
                                            TimingPointParseError::InvalidUninherited.into(),
                                            map_res(comma_field(), parse_zero_one_bool),
                                        ),
                                    ),
                                    preceded(
                                        context(
                                            TimingPointParseError::MissingEffects.into(),
                                            comma(),
                                        ),
                                        context(
                                            TimingPointParseError::InvalidEffects.into(),
                                            consume_rest_type(),
                                        ),
                                    ),
                                )),
                            )),
                        )),
                    )),
                )),
            )),
        ))(s)?;

        (
            time,
            beat_length,
            meter,
            sample_set,
            sample_index,
            volume,
            uninherited,
            effects,
        );

        Ok(Some(TimingPoint {
            time,
            beat_length,
            meter,
            sample_set,
            sample_index,
            volume,
            uninherited,
            effects,
        }))
    }

    fn to_string(&self, version: usize) -> Option<String> {
        let mut fields = vec![
            if (3..=4).contains(&version) {
                self.time - OLD_VERSION_TIME_OFFSET
            } else {
                self.time
            }
            .to_string(),
            self.beat_length.to_string(),
        ];

        if version > 3 {
            fields.push(self.meter.to_string());
            fields.push(self.sample_set.to_string());
            fields.push(self.sample_index.to_string());
        }
        if version > 4 {
            fields.push(self.volume.to_string());
        }
        if version > 5 {
            // TODO check all bool types to be integer
            fields.push((self.uninherited as u8).to_string());
            fields.push(self.effects.to_string());
        }

        Some(format!("{}", fields.join(",")))
    }

    fn default(version: usize) -> Option<Self> {
        let mut default = <Self as Default>::default();

        if (3..=4).contains(&version) {
            default.time -= OLD_VERSION_TIME_OFFSET;
        }

        Some(default)
    }
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
