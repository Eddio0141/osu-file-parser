pub mod error;

use std::{fmt::Display, str::FromStr};

use rust_decimal::Decimal;

use super::SECTION_DELIMITER;

pub use self::error::*;

#[derive(Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
/// Difficulty settings.
pub struct Difficulty {
    /// `HP` settings.
    pub hp_drain_rate: Option<Decimal>,
    /// `CS` settings.
    pub circle_size: Option<Decimal>,
    /// `OD` settings.
    pub overall_difficulty: Option<Decimal>,
    /// `AR` settings.
    pub approach_rate: Option<Decimal>,
    /// Base slider velocity in hundreds of `osu!pixels` per beat.
    pub slider_multiplier: Option<Decimal>,
    /// Amount of slider ticks per beat.
    pub slider_tickrate: Option<Decimal>,
}

impl FromStr for Difficulty {
    type Err = DifficultyParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut difficulty = Self::default();

        let s = s.trim();

        for line in s.lines() {
            match line.split_once(SECTION_DELIMITER) {
                Some((key, value)) => {
                    let value = value.trim();
                    match key.trim() {
                        "HPDrainRate" => {
                            difficulty.hp_drain_rate = Some(value.parse().map_err(|err| {
                                DifficultyParseError::SectionParseError {
                                    source: err,
                                    name: "HPDrainRate",
                                }
                            })?)
                        }
                        "CircleSize" => {
                            difficulty.circle_size = Some(value.parse().map_err(|err| {
                                DifficultyParseError::SectionParseError {
                                    source: err,
                                    name: "CircleSize",
                                }
                            })?)
                        }
                        "OverallDifficulty" => {
                            difficulty.overall_difficulty = Some(value.parse().map_err(|err| {
                                DifficultyParseError::SectionParseError {
                                    source: err,
                                    name: "OverallDifficulty",
                                }
                            })?)
                        }
                        "ApproachRate" => {
                            difficulty.approach_rate = Some(value.parse().map_err(|err| {
                                DifficultyParseError::SectionParseError {
                                    source: err,
                                    name: "ApproachRate",
                                }
                            })?)
                        }
                        "SliderMultiplier" => {
                            difficulty.slider_multiplier = Some(value.parse().map_err(|err| {
                                DifficultyParseError::SectionParseError {
                                    source: err,
                                    name: "SliderMultiplier",
                                }
                            })?)
                        }
                        "SliderTickRate" => {
                            difficulty.slider_tickrate = Some(value.parse().map_err(|err| {
                                DifficultyParseError::SectionParseError {
                                    source: err,
                                    name: "SliderTickRate",
                                }
                            })?)
                        }
                        _ => return Err(DifficultyParseError::InvalidKey(key.to_string())),
                    }
                }
                None => return Err(DifficultyParseError::MissingValue(line.to_owned())),
            }
        }

        Ok(difficulty)
    }
}

impl Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut key_value = Vec::new();

        key_value.push(("HPDrainRate", &self.hp_drain_rate));
        key_value.push(("CircleSize", &self.circle_size));
        key_value.push(("OverallDifficulty", &self.overall_difficulty));
        key_value.push(("ApproachRate", &self.approach_rate));
        key_value.push(("SliderMultiplier", &self.slider_multiplier));
        key_value.push(("SliderTickRate", &self.slider_tickrate));

        write!(
            f,
            "{}",
            key_value
                .iter()
                .filter_map(|(k, v)| v.as_ref().map(|v| format!("{k}:{v}")))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}
