pub mod error;
pub mod types;

use std::str::FromStr;

use either::Either;
use nom::{
    branch::alt,
    combinator::{cut, map_res, rest, success, verify},
    error::context,
    sequence::{preceded, tuple},
    Parser,
};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::{helper::parse_zero_one_bool, helper::trait_ext::MapStringNewLineVersion, parsers::*};

use super::{
    Error, Integer, Version, VersionedDefault, VersionedFrom, VersionedFromStr, VersionedToString,
    VersionedTryFrom,
};

pub use error::*;
pub use types::*;

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct TimingPoints(pub Vec<TimingPoint>);

impl VersionedFromStr for TimingPoints {
    type Err = Error<ParseError>;

    fn from_str(s: &str, version: Version) -> std::result::Result<Option<Self>, Self::Err> {
        let mut timing_points = Vec::new();

        for (line_index, s) in s.lines().enumerate() {
            if s.trim().is_empty() {
                continue;
            }

            timing_points.push(Error::new_from_result_into(
                TimingPoint::from_str(s, version),
                line_index,
            )?);
        }

        if let Some(s) = timing_points.get(0) {
            if s.is_some() {
                Ok(Some(TimingPoints(
                    timing_points
                        .into_iter()
                        .map(|v| v.unwrap())
                        .collect::<Vec<_>>(),
                )))
            } else {
                Ok(None)
            }
        } else {
            Ok(Some(TimingPoints(Vec::new())))
        }
    }
}

impl VersionedToString for TimingPoints {
    fn to_string(&self, version: Version) -> Option<String> {
        Some(self.0.iter().map_string_new_line(version))
    }
}

impl VersionedDefault for TimingPoints {
    fn default(_: Version) -> Option<Self> {
        Some(TimingPoints(Vec::new()))
    }
}

/// Struct representing a timing point.
/// Each timing point influences a specified portion of the map, commonly called a `timing section`.
/// The .osu file format requires these to be sorted in chronological order.
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct TimingPoint {
    // for some reason decimal is parsed anyway in the beatmap???
    time: Decimal,
    beat_length: Option<Decimal>,
    beat_length_string: Option<String>,
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
        slider_velocity_multiplier: Either<Decimal, String>,
        meter: Integer,
        sample_set: SampleSet,
        sample_index: SampleIndex,
        volume: Volume,
        effects: Effects,
    ) -> Self {
        let (beat_length, beat_length_string) = match slider_velocity_multiplier {
            Either::Left(beat_length) => (Some(beat_length), None),
            Either::Right(beat_length_string) => (None, Some(beat_length_string)),
        };

        Self {
            time: time.into(),
            beat_length,
            beat_length_string,
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
        beat_duration_ms: Either<Decimal, String>,
        meter: Integer,
        sample_set: SampleSet,
        sample_index: SampleIndex,
        volume: Volume,
        effects: Effects,
    ) -> Self {
        let (beat_length, beat_length_string) = match beat_duration_ms {
            Either::Left(beat_length) => (Some(beat_length), None),
            Either::Right(beat_length_string) => (None, Some(beat_length_string)),
        };

        Self {
            time: time.into(),
            beat_length,
            beat_length_string,
            meter,
            sample_set,
            sample_index,
            volume,
            uninherited: true,
            effects,
        }
    }

    /// Calculates BPM using the `beatLength` field when unherited.
    /// - Returns `None` if the timing point is inherited or `beat_length` isn't a valid decimal.
    pub fn calc_bpm(&self) -> Option<Decimal> {
        if self.uninherited {
            self.beat_length.map(Self::beat_duration_ms_to_bpm)
        } else {
            None
        }
    }
    /// Calculates the slider velocity multiplier when the timing point is inherited.
    /// - Returns `None` if the timing point is uninherited or `beat_length` isn't a valid decimal.
    pub fn calc_slider_velocity_multiplier(&self) -> Option<Decimal> {
        if self.uninherited {
            None
        } else {
            self.beat_length
                .map(|beat_length| Decimal::ONE / (beat_length / dec!(-100)))
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

impl VersionedFromStr for TimingPoint {
    type Err = ParseTimingPointError;

    fn from_str(s: &str, version: Version) -> std::result::Result<Option<Self>, Self::Err> {
        let meter_fallback = 4;
        let sample_set_fallback = SampleSet::Normal;
        let sample_index_fallback = <SampleIndex as VersionedFrom<u32>>::from(1, version).unwrap();
        let volume_fallback = <Volume as VersionedTryFrom<u8>>::try_from(100, version)
            .unwrap()
            .unwrap();
        let uninherited_fallback = true;
        let effects_fallback = <Effects as VersionedFrom<u8>>::from(0, version).unwrap();

        let (
            _,
            (
                time,
                (
                    beat_length_string,
                    meter,
                    sample_set,
                    (sample_index, (volume, uninherited, effects)),
                ),
            ),
        ) = tuple((
            context(
                ParseTimingPointError::InvalidTime.into(),
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
                context(ParseTimingPointError::MissingBeatLength.into(), comma()),
                alt((
                    preceded(verify(success(0), |_| version == 3), rest).map(|beat_length| {
                        (
                            beat_length,
                            meter_fallback,
                            sample_set_fallback,
                            (
                                sample_index_fallback,
                                (volume_fallback, uninherited_fallback, effects_fallback),
                            ),
                        )
                    }),
                    tuple((
                        comma_field(),
                        preceded(
                            context(ParseTimingPointError::MissingMeter.into(), comma()),
                            context(
                                ParseTimingPointError::InvalidMeter.into(),
                                comma_field_type(),
                            ),
                        ),
                        preceded(
                            context(ParseTimingPointError::MissingSampleSet.into(), comma()),
                            context(
                                ParseTimingPointError::InvalidSampleSet.into(),
                                comma_field_versioned_type(version),
                            ),
                        ),
                        preceded(
                            context(ParseTimingPointError::MissingSampleIndex.into(), comma()),
                            alt((
                                preceded(
                                    verify(success(0), |_| version == 4),
                                    context(
                                        ParseTimingPointError::InvalidSampleIndex.into(),
                                        cut(consume_rest_versioned_type(version)),
                                    ),
                                )
                                .map(|sample_index| {
                                    (
                                        sample_index,
                                        (volume_fallback, uninherited_fallback, effects_fallback),
                                    )
                                }),
                                tuple((
                                    context(
                                        ParseTimingPointError::InvalidSampleIndex.into(),
                                        comma_field_versioned_type(version),
                                    ),
                                    preceded(
                                        context(
                                            ParseTimingPointError::MissingVolume.into(),
                                            comma(),
                                        ),
                                        alt((
                                            preceded(
                                                verify(success(0), |_| version == 5),
                                                context(
                                                    ParseTimingPointError::InvalidVolume.into(),
                                                    cut(consume_rest_versioned_type(version)),
                                                ),
                                            )
                                            .map(
                                                |volume| {
                                                    (volume, uninherited_fallback, effects_fallback)
                                                },
                                            ),
                                            tuple((
                                                context(
                                                    ParseTimingPointError::InvalidVolume.into(),
                                                    comma_field_versioned_type(version),
                                                ),
                                                preceded(
                                                    context(
                                                        ParseTimingPointError::MissingUninherited
                                                            .into(),
                                                        comma(),
                                                    ),
                                                    context(
                                                        ParseTimingPointError::InvalidUninherited
                                                            .into(),
                                                        map_res(comma_field(), parse_zero_one_bool),
                                                    ),
                                                ),
                                                preceded(
                                                    context(
                                                        ParseTimingPointError::MissingEffects
                                                            .into(),
                                                        comma(),
                                                    ),
                                                    context(
                                                        ParseTimingPointError::InvalidEffects
                                                            .into(),
                                                        consume_rest_versioned_type(version),
                                                    ),
                                                ),
                                            ))
                                            .map(
                                                |(volume, uninherited, effects)| {
                                                    (volume, uninherited, effects)
                                                },
                                            ),
                                        )),
                                    ),
                                ))
                                .map(
                                    |(sample_index, (volume, uninherited, effects))| {
                                        (sample_index, (volume, uninherited, effects))
                                    },
                                ),
                            )),
                        ),
                    ))
                    .map(
                        |(
                            beat_length,
                            meter,
                            sample_set,
                            (sample_index, (volume, uninherited, effects)),
                        )| {
                            (
                                beat_length,
                                meter,
                                sample_set,
                                (sample_index, (volume, uninherited, effects)),
                            )
                        },
                    ),
                )),
            ),
        ))(s)?;

        let (beat_length, beat_length_string) = match Decimal::from_str(beat_length_string) {
            Ok(beat_length) => (Some(beat_length), None),
            Err(_) => (None, Some(beat_length_string.to_string())),
        };

        Ok(Some(TimingPoint {
            time,
            beat_length,
            beat_length_string,
            meter,
            sample_set,
            sample_index,
            volume,
            uninherited,
            effects,
        }))
    }
}

impl VersionedToString for &TimingPoint {
    fn to_string(&self, version: Version) -> Option<String> {
        self.to_owned().to_string(version)
    }
}

impl VersionedToString for TimingPoint {
    fn to_string(&self, version: Version) -> Option<String> {
        let beat_length = if let Some(beat_length) = &self.beat_length {
            beat_length.to_string()
        } else {
            self.beat_length_string.clone().unwrap()
        };

        let mut fields = vec![
            if (3..=4).contains(&version) {
                self.time - OLD_VERSION_TIME_OFFSET
            } else {
                self.time
            }
            .to_string(),
            beat_length,
        ];

        if version > 3 {
            fields.push(self.meter.to_string());
            fields.push(self.sample_set.to_string(version).unwrap());
            fields.push(self.sample_index.to_string(version).unwrap());
        }
        if version > 4 {
            fields.push(self.volume.to_string(version).unwrap());
        }
        if version > 5 {
            fields.push((self.uninherited as u8).to_string());
            fields.push(self.effects.to_string(version).unwrap());
        }

        Some(fields.join(","))
    }
}
