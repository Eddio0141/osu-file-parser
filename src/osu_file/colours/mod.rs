pub mod error;

use std::{fmt::Display, str::FromStr};

use super::Integer;

use self::error::*;

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct Colours(pub Vec<Colour>);

impl Display for Colours {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl FromStr for Colours {
    type Err = ColoursParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut colours = Vec::new();

        for s in s.lines() {
            colours.push(s.parse()?);
        }

        Ok(Colours(colours))
    }
}

/// Struct representing a single `colour` component in the `Colours` section.
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Colour {
    /// Additive combo colours.
    Combo(Integer, Rgb),
    /// Additive slider track colour.
    SliderTrackOverride(Rgb),
    /// Slider border colour.
    SliderBorder(Rgb),
}

impl FromStr for Colour {
    type Err = ColourParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // delimiter not the same
        if let Some((key, value)) = s.split_once(':') {
            let rgb = || value.parse();

            let key = key.trim();
            if key.starts_with("Combo") {
                let combo_count = key.strip_prefix("Combo").unwrap();
                let combo_count =
                    combo_count
                        .parse()
                        .map_err(|err| ColourParseError::ComboCountParseError {
                            source: err,
                            value: combo_count.to_string(),
                        })?;

                Ok(Colour::Combo(combo_count, rgb()?))
            } else {
                match key {
                    "SliderTrackOverride" => Ok(Colour::SliderTrackOverride(rgb()?)),
                    "SliderBorder" => Ok(Colour::SliderBorder(rgb()?)),
                    _ => Err(ColourParseError::InvalidKey(key.to_string())),
                }
            }
        } else {
            Err(ColourParseError::NoValue(s.to_string()))
        }
    }
}

impl Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let colour_str = match self {
            Colour::Combo(num, rgb) => format!("Combo{num} : {rgb}"),
            Colour::SliderTrackOverride(rgb) => format!("SliderTrackOverride : {rgb}"),
            Colour::SliderBorder(rgb) => format!("SliderBorder : {rgb}"),
        };

        write!(f, "{colour_str}")
    }
}

#[derive(Default, Clone, Copy, Hash, Debug, PartialEq, Eq)]
/// Struct representing the RGB colours with each colour having value from 0 ~ 255.
pub struct Rgb {
    /// Red colour.
    pub red: u8,
    /// Green colour.
    pub green: u8,
    /// Blue colour.
    pub blue: u8,
}

impl FromStr for Rgb {
    type Err = RGBParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(',');

        let red = s.next().ok_or(RGBParseError::NoRedDefined)?.trim();
        let red = red.parse().map_err(|err| RGBParseError::ValueParseError {
            source: err,
            value: red.to_string(),
            field_name: "red",
        })?;

        let green = s.next().ok_or(RGBParseError::NoGreenDefined)?.trim();
        let green = green
            .parse()
            .map_err(|err| RGBParseError::ValueParseError {
                source: err,
                value: green.to_string(),
                field_name: "green",
            })?;

        let blue = s.next().ok_or(RGBParseError::NoBlueDefined)?.trim();
        let blue = blue.parse().map_err(|err| RGBParseError::ValueParseError {
            source: err,
            value: blue.to_string(),
            field_name: "blue",
        })?;

        Ok(Self { red, green, blue })
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.red, self.green, self.blue)
    }
}
