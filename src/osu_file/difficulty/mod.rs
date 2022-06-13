pub mod error;

use rust_decimal::Decimal;

use crate::{helper::macros::*, parsers::get_colon_field_value_lines};

use super::Error;

pub use self::error::*;
use crate::osu_file::types::Version;

versioned_field!(HPDrainRate, Decimal, no_versions, |s| { s.parse() } -> rust_decimal::Error, |v| { v.to_string() });
versioned_field!(CircleSize, Decimal, no_versions, |s| { s.parse() } -> rust_decimal::Error, |v| { v.to_string() });
versioned_field!(OverallDifficulty, Decimal, no_versions, |s| { s.parse() } -> rust_decimal::Error, |v| { v.to_string() });
versioned_field!(ApproachRate, Decimal, no_versions, |s| { s.parse() } -> rust_decimal::Error, |v| { v.to_string() });
versioned_field!(SliderMultiplier, Decimal, no_versions, |s| { s.parse() } -> rust_decimal::Error, |v| { v.to_string() });
versioned_field!(SliderTickRate, Decimal, no_versions, |s| { s.parse() } -> rust_decimal::Error, |v| { v.to_string() });

general_section!(
    /// Difficulty settings.
    pub struct Difficulty {
        /// `HP` settings.
        pub hp_drain_rate: HPDrainRate,
        /// `CS` settings.
        pub circle_size: CircleSize,
        /// `OD` settings.
        pub overall_difficulty: OverallDifficulty,
        /// `AR` settings.
        pub approach_rate: ApproachRate,
        /// Base slider velocity in hundreds of `osu!pixels` per beat.
        pub slider_multiplier: SliderMultiplier,
        /// Amount of slider ticks per beat.
        pub slider_tickrate: SliderTickRate,
    },
    ParseError
);
