pub mod error;
pub mod types;

use crate::osu_file::types::Decimal;
use either::Either;
use nom::{
    branch::alt,
    combinator::{cut, map_res, success, verify},
    error::context,
    sequence::{preceded, tuple},
    Parser,
};
use rust_decimal_macros::dec;

use crate::{helper::parse_zero_one_bool, helper::trait_ext::MapStringNewLineVersion, parsers::*};

use super::{
    Error, Integer, Version, VersionedDefault, VersionedFrom, VersionedFromStr, VersionedToString,
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
    beat_length: Decimal,
    meter: Integer,
    sample_set: SampleSet,
    sample_index: SampleIndex,
    volume: Volume,
    uninherited: Option<bool>,
    effects: Option<Effects>,
}

impl TimingPoint {
    /// Converts beat duration in milliseconds to BPM.
    pub fn beat_duration_ms_to_bpm(
        beat_duration_ms: rust_decimal::Decimal,
    ) -> rust_decimal::Decimal {
        rust_decimal::Decimal::ONE / beat_duration_ms * dec!(60000)
    }

    /// Converts BPM to beat duration in milliseconds.
    pub fn bpm_to_beat_duration_ms(bpm: rust_decimal::Decimal) -> rust_decimal::Decimal {
        rust_decimal::Decimal::ONE / (bpm / dec!(60000))
    }

    /// New instance of `TimingPoint` that is inherited.
    pub fn new_inherited(
        time: Integer,
        slider_velocity_multiplier: rust_decimal::Decimal,
        meter: Integer,
        sample_set: SampleSet,
        sample_index: SampleIndex,
        volume: Volume,
        effects: Effects,
    ) -> Self {
        let beat_length = (rust_decimal::Decimal::ONE / slider_velocity_multiplier) * dec!(-100);

        Self {
            time: time.into(),
            beat_length: beat_length.into(),
            meter,
            sample_set,
            sample_index,
            volume,
            uninherited: Some(false),
            effects: Some(effects),
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
            uninherited: Some(true),
            effects: Some(effects),
        }
    }

    /// Calculates BPM using the `beatLength` field when unherited.
    /// - Returns `None` if the timing point is inherited or `beat_length` isn't a valid decimal.
    pub fn calc_bpm(&self) -> Option<rust_decimal::Decimal> {
        match self.uninherited {
            Some(uninherited) => {
                if uninherited {
                    match self.beat_length.get() {
                        Either::Left(value) => Some(Self::beat_duration_ms_to_bpm(*value)),
                        Either::Right(_) => None,
                    }
                } else {
                    None
                }
            }
            None => None,
        }
    }
    /// Calculates the slider velocity multiplier when the timing point is inherited.
    /// - Returns `None` if the timing point is uninherited or `beat_length` isn't a valid decimal.
    pub fn calc_slider_velocity_multiplier(&self) -> Option<rust_decimal::Decimal> {
        match self.uninherited {
            Some(uninherited) => {
                if uninherited {
                    None
                } else {
                    match self.beat_length.get() {
                        Either::Left(value) => {
                            Some(rust_decimal::Decimal::ONE / (value / dec!(-100)))
                        }
                        Either::Right(_) => None,
                    }
                }
            }
            None => None,
        }
    }

    /// Start time of the timing section, in milliseconds from the beginning of the beatmap's audio.
    /// The end of the timing section is the next timing point's time (or never, if this is the last timing point).
    pub fn time(&self) -> &Decimal {
        &self.time
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
        self.uninherited.unwrap_or(true)
    }

    /// Set the timing point's uninherited.
    pub fn set_uninherited(&mut self, uninherited: bool) {
        self.uninherited = Some(uninherited);
    }

    /// Get the timing point's effects.
    pub fn effects(&self) -> Option<&Effects> {
        self.effects.as_ref()
    }

    /// Get a mutable reference to the timing point's effects.
    pub fn effects_mut(&mut self) -> &mut Option<Effects> {
        &mut self.effects
    }
}

const OLD_VERSION_TIME_OFFSET: rust_decimal::Decimal = dec!(24);

impl VersionedFromStr for TimingPoint {
    type Err = ParseTimingPointError;

    fn from_str(s: &str, version: Version) -> std::result::Result<Option<Self>, Self::Err> {
        let meter_fallback = 4;
        let sample_set_fallback = SampleSet::Normal;
        let sample_index_fallback = <SampleIndex as VersionedFrom<u32>>::from(1, version).unwrap();
        let volume_fallback = <Volume as VersionedFrom<i8>>::from(100, version).unwrap();

        let (
            _,
            (
                time,
                (beat_length, meter, sample_set, (sample_index, (volume, uninherited, effects))),
            ),
        ) = tuple((
            context(
                ParseTimingPointError::InvalidTime.into(),
                comma_field_type(),
            )
            .map(|mut t: Decimal| {
                if (3..=4).contains(&version) {
                    if let Either::Left(value) = t.get_mut() {
                        *value += OLD_VERSION_TIME_OFFSET;
                    }
                }

                t
            }),
            preceded(
                context(ParseTimingPointError::MissingBeatLength.into(), comma()),
                alt((
                    preceded(verify(success(0), |_| version == 3), consume_rest_type()).map(
                        |beat_length| {
                            (
                                beat_length,
                                meter_fallback,
                                sample_set_fallback,
                                (sample_index_fallback, (volume_fallback, None, None)),
                            )
                        },
                    ),
                    tuple((
                        comma_field_type(),
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
                                .map(|sample_index| (sample_index, (volume_fallback, None, None))),
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
                                                verify(success(0), |_| version == 4),
                                                context(
                                                    ParseTimingPointError::InvalidVolume.into(),
                                                    cut(consume_rest_versioned_type(version)),
                                                ),
                                            )
                                            .map(|volume| (volume, None, None)),
                                            tuple((
                                                context(
                                                    ParseTimingPointError::InvalidVolume.into(),
                                                    comma_field_versioned_type(version),
                                                ),
                                                cut(preceded(
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
                                                )),
                                                cut(preceded(
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
                                                )),
                                            ))
                                            .map(
                                                |(volume, uninherited, effects)| {
                                                    (volume, Some(uninherited), Some(effects))
                                                },
                                            ),
                                            context(
                                                ParseTimingPointError::InvalidVolume.into(),
                                                cut(consume_rest_versioned_type(version)),
                                            )
                                            .map(|volume| (volume, None, None)),
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
}

impl VersionedToString for &TimingPoint {
    fn to_string(&self, version: Version) -> Option<String> {
        self.to_owned().to_string(version)
    }
}

impl VersionedToString for TimingPoint {
    fn to_string(&self, version: Version) -> Option<String> {
        let mut fields = vec![
            if (3..=4).contains(&version) {
                match self.time.get() {
                    Either::Left(value) => (value - OLD_VERSION_TIME_OFFSET).to_string(),
                    Either::Right(value) => value.to_string(),
                }
            } else {
                self.time.to_string()
            },
            self.beat_length.to_string(),
        ];

        if version > 3 {
            fields.push(self.meter.to_string());
            fields.push(self.sample_set.to_string(version).unwrap());
            fields.push(self.sample_index.to_string(version).unwrap());
        }
        if version > 4 {
            fields.push(self.volume.to_string(version).unwrap());
        }
        if version > 4 {
            match self.uninherited {
                Some(value) => fields.push((value as u8).to_string()),
                None => fields.push(String::new()),
            }
            if let Some(value) = self.effects {
                fields.push(value.to_string(version).unwrap())
            }
        }

        Some(fields.join(","))
    }
}
