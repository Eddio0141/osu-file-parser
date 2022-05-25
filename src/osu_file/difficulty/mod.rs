pub mod error;

use rust_decimal::Decimal;

use crate::{helper::display_colon_fields, parsers::get_colon_field_value_lines};

use super::{Error, Version};

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

impl Version for Difficulty {
    type ParseError = Error<ParseError>;

    // TODO investigate
    fn from_str_v3(s: &str) -> std::result::Result<Option<Self>, Self::ParseError>
    where
        Self: Sized,
    {
        let mut difficulty = Self::default();

        let (s, fields) = get_colon_field_value_lines(s).unwrap();

        if !s.trim().is_empty() {
            // line count from fields
            // TODO test this and for general too
            let line_count = { fields.iter().map(|(_, _, ws)| ws.lines().count()).sum() };

            return Err(Error::new(ParseError::InvalidColonSet, line_count));
        }

        let mut line_count = 0;
        let mut parsed_fields = Vec::new();

        for (name, value, ws) in fields {
            let new_into_decimal = move |err| Error::new_into(err, line_count);

            if parsed_fields.contains(&name) {
                return Err(Error::new(ParseError::DuplicateField, line_count));
            }

            match name {
                "HPDrainRate" => {
                    difficulty.hp_drain_rate = Some(value.parse().map_err(new_into_decimal)?)
                }
                "CircleSize" => {
                    difficulty.circle_size = Some(value.parse().map_err(new_into_decimal)?)
                }
                "OverallDifficulty" => {
                    difficulty.overall_difficulty = Some(value.parse().map_err(new_into_decimal)?)
                }
                "ApproachRate" => {
                    difficulty.approach_rate = Some(value.parse().map_err(new_into_decimal)?)
                }
                "SliderMultiplier" => {
                    difficulty.slider_multiplier = Some(value.parse().map_err(new_into_decimal)?)
                }
                "SliderTickRate" => {
                    difficulty.slider_tickrate = Some(value.parse().map_err(new_into_decimal)?)
                }
                _ => return Err(Error::new(ParseError::InvalidKey, line_count)),
            }

            line_count += ws.lines().count();
            parsed_fields.push(name);
        }

        Ok(Some(difficulty))
    }

    fn to_string_v3(&self) -> Option<String> {
        let hp_drain_rate = self.hp_drain_rate.as_ref().map(|v| v.to_string());
        let circle_size = self.circle_size.as_ref().map(|v| v.to_string());
        let overall_difficulty = self.overall_difficulty.as_ref().map(|v| v.to_string());
        let approach_rate = self.approach_rate.as_ref().map(|v| v.to_string());
        let slider_multiplier = self.slider_multiplier.as_ref().map(|v| v.to_string());
        let slider_tickrate = self.slider_tickrate.as_ref().map(|v| v.to_string());

        let fields = vec![
            ("HPDrainRate", &hp_drain_rate),
            ("CircleSize", &circle_size),
            ("OverallDifficulty", &overall_difficulty),
            ("ApproachRate", &approach_rate),
            ("SliderMultiplier", &slider_multiplier),
            ("SliderTickRate", &slider_tickrate),
        ];

        Some(display_colon_fields(&fields, false))
    }
}
