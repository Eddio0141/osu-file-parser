use std::{fmt::Display, str::FromStr};

use rust_decimal::Decimal;
use thiserror::Error;

use super::SECTION_DELIMITER;

#[derive(Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
/// Difficulty settings.
pub struct Difficulty {
    /// `HP` settings.
    pub hp_drain_rate: Decimal,
    /// `CS` settings.
    pub circle_size: Decimal,
    /// `OD` settings.
    pub overall_difficulty: Decimal,
    /// `AR` settings.
    pub approach_rate: Decimal,
    /// Base slider velocity in hundreds of `osu!pixels` per beat.
    pub slider_multiplier: Decimal,
    /// Amount of slider ticks per beat.
    pub slider_tickrate: Decimal,
}

impl FromStr for Difficulty {
    type Err = DifficultyParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut difficulty = Self::default();

        let s = s.trim();

        for line in s.lines() {
            match line.split_once(SECTION_DELIMITER) {
                Some((key, value)) => match key.trim() {
                    "HPDrainRate" => {
                        difficulty.hp_drain_rate = value.parse().map_err(|err| {
                            DifficultyParseError::SectionParseError {
                                source: err,
                                name: "HPDrainRate",
                            }
                        })?
                    }
                    "CircleSize" => {
                        difficulty.circle_size = value.parse().map_err(|err| {
                            DifficultyParseError::SectionParseError {
                                source: err,
                                name: "CircleSize",
                            }
                        })?
                    }
                    "OverallDifficulty" => {
                        difficulty.overall_difficulty = value.parse().map_err(|err| {
                            DifficultyParseError::SectionParseError {
                                source: err,
                                name: "OverallDifficulty",
                            }
                        })?
                    }
                    "ApproachRate" => {
                        difficulty.approach_rate = value.parse().map_err(|err| {
                            DifficultyParseError::SectionParseError {
                                source: err,
                                name: "ApproachRate",
                            }
                        })?
                    }
                    "SliderMultiplier" => {
                        difficulty.slider_multiplier = value.parse().map_err(|err| {
                            DifficultyParseError::SectionParseError {
                                source: err,
                                name: "SliderMultiplier",
                            }
                        })?
                    }
                    "SliderTickRate" => {
                        difficulty.slider_tickrate = value.parse().map_err(|err| {
                            DifficultyParseError::SectionParseError {
                                source: err,
                                name: "SliderTickRate",
                            }
                        })?
                    }
                    _ => return Err(DifficultyParseError::InvalidKey(key.to_string())),
                },
                None => return Err(DifficultyParseError::MissingValue(line.to_owned())),
            }
        }

        Ok(difficulty)
    }
}

impl Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut key_value = Vec::new();

        key_value.push(format!("HPDrainRate:{}", self.hp_drain_rate));
        key_value.push(format!("CircleSize:{}", self.circle_size));
        key_value.push(format!("OverallDifficulty:{}", self.overall_difficulty));
        key_value.push(format!("ApproachRate:{}", self.approach_rate));
        key_value.push(format!("SliderMultiplier:{}", self.slider_multiplier));
        key_value.push(format!("SliderTickRate:{}", self.slider_tickrate));

        write!(f, "{}", key_value.join("\n"))
    }
}

#[derive(Debug, Error)]
/// Error used when there was a problem parsing the `Difficulty` section.
pub enum DifficultyParseError {
    #[error("There was a problem parsing the `{name}` property from a `str`")]
    /// A section in `Difficulty` failed to parse.
    SectionParseError {
        #[source]
        source: rust_decimal::Error,
        name: &'static str,
    },
    #[error("The key {0} doesn't exist in `Difficulty`")]
    /// Invalid key name was used.
    InvalidKey(String),
    #[error("The key {0} has no value set")]
    /// The value is missing from the `key: value` set.
    MissingValue(String),
}
