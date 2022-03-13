use std::{
    fmt::Display,
    num::{NonZeroUsize, ParseIntError},
    str::FromStr,
};

use rust_decimal::Decimal;
use thiserror::Error;

use super::{helper::nth_bit_state_i64, Integer};

#[derive(Default, Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct TimingPoint {
    pub time: Integer,
    pub beat_length: Decimal,
    pub meter: Integer,
    pub sample_set: SampleSet,
    pub sample_index: Option<NonZeroUsize>,
    pub volume: Integer,
    pub effects: Effects,
}

impl FromStr for TimingPoint {
    type Err = TimingPointParseError;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Error)]
#[error("")]
pub struct TimingPointParseError;

// TODO figure out default
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum SampleSet {
    BeatmapDefault,
    Normal,
    Soft,
    Drum,
}

impl Default for SampleSet {
    fn default() -> Self {
        Self::BeatmapDefault
    }
}

impl FromStr for SampleSet {
    type Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

pub struct SampleSetParseError;

// TODO figure out default
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

#[derive(Debug, Error)]
#[error("There was a problem parsing {value} as an `Integer`")]
pub struct EffectsParseError {
    #[source]
    source: ParseIntError,
    value: String,
}
