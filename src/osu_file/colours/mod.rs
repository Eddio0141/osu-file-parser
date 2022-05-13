mod colour_parser;
pub mod error;

use std::{fmt::Display, str::FromStr};

use nom::{bytes::complete::take_till, error::VerboseErrorKind, Finish};

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
            if !s.is_empty() {
                colours.push(s.parse()?);
            }
        }

        Ok(Colours(colours))
    }
}

// TODO check of combo can be i32
/// Struct representing a single `colour` component in the `Colours` section.
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Colour {
    /// Additive combo colours.
    Combo(u32, Rgb),
    /// Additive slider track colour.
    SliderTrackOverride(Rgb),
    /// Slider border colour.
    SliderBorder(Rgb),
}

impl FromStr for Colour {
    type Err = ColourParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match colour_parser::colour(s).finish() {
            Ok((_, colour)) => Ok(colour),
            Err(err) => {
                for err in &err.errors {
                    if let VerboseErrorKind::Context(context) = err.1 {
                        let err = match colour_parser::Context::from_str(context).unwrap() {
                            colour_parser::Context::UnknownColourOption => {
                                ColourParseError::UnknownColourOption(
                                    take_till::<_, _, nom::error::Error<_>>(|c| c == ':')(err.0)
                                        .unwrap()
                                        .1
                                        .trim()
                                        .to_string(),
                                )
                            }
                            colour_parser::Context::InvalidAdditiveComboCount => {
                                ColourParseError::InvalidComboCount(
                                    take_till::<_, _, nom::error::Error<_>>(|c| c == ' ')(err.0)
                                        .unwrap()
                                        .1
                                        .trim()
                                        .to_string(),
                                )
                            }
                            colour_parser::Context::InvalidRed
                            | colour_parser::Context::InvalidBlue
                            | colour_parser::Context::InvalidGreen => {
                                ColourParseError::InvalidColourValue(
                                    take_till::<_, _, nom::error::Error<_>>(|c| {
                                        c == ',' || c == ' '
                                    })(err.0)
                                    .unwrap()
                                    .1
                                    .trim()
                                    .to_string(),
                                )
                            }
                            colour_parser::Context::NoCommaAfterRed => {
                                ColourParseError::MissingGreenValue
                            }
                            colour_parser::Context::NoCommaAfterGreen => {
                                ColourParseError::MissingBlueValue
                            }
                        };

                        return Err(err);
                    }
                }

                unimplemented!("unimplemented colour nom parser error, {}", err)
            }
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

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.red, self.green, self.blue)
    }
}
