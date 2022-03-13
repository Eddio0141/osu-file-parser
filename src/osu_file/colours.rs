use std::{fmt::Display, num::ParseIntError, str::FromStr};

use thiserror::Error;

use super::Integer;

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

/// Error used when there was a problem parsing a `str` as a `Colour`.
#[derive(Debug, Error)]
pub enum ColourParseError {
    /// The colour key was invalid.
    #[error("The colour key was an invalid key, got {0}")]
    InvalidKey(String),
    /// Attempted to parse a `str` as an `Integer` for the `ComboCount`.
    #[error("Attempted to parse {value} as an `Integer` for `ComboCount`")]
    ComboCountParseError {
        #[source]
        source: ParseIntError,
        value: String,
    },
    /// There is no value. Expected `key : value` pair.
    #[error("There is no value, expected `key : value` pairs, got {0}")]
    NoValue(String),
    #[error(transparent)]
    RGBParseError {
        #[from]
        source: RGBParseError,
    },
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

/// Error when there was a problem parsing `str` as a `RGB` struct.
#[derive(Debug, Error)]
pub enum RGBParseError {
    /// No red field defined.
    #[error("There is no red field defined")]
    NoRedDefined,
    /// No green field defined.
    #[error("There is no green field defined")]
    NoGreenDefined,
    /// No blue field defined.
    #[error("There is no blue field defined")]
    NoBlueDefined,
    /// More than 3 fields defined.
    #[error("There is more than 3 fields defined `{0}`, only needs the `red`,`green`,`blue`")]
    MoreThanThreeFields(String),
    /// There was a problem parsing the field as an `Integer`.
    #[error(
        "There was a problem parsing the {field_name} field with the value {value} as an `Integer`"
    )]
    ValueParseError {
        #[source]
        source: ParseIntError,
        value: String,
        field_name: &'static str,
    },
}
