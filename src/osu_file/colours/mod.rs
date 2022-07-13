pub mod error;

use std::fmt::Display;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space0},
    combinator::{cut, fail, map_res},
    error::context,
    sequence::{preceded, tuple},
    Parser,
};

use crate::parsers::comma;

pub use self::error::*;

use super::{Error, VersionedDefault, VersionedFromString, VersionedToString, MIN_VERSION};
use crate::helper::trait_ext::MapStringNewLine;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Colours(pub Vec<Colour>);

impl VersionedFromString for Colours {
    type ParseError = Error<ParseError>;

    fn from_str(s: &str, version: usize) -> std::result::Result<Option<Self>, Self::ParseError> {
        match version {
            MIN_VERSION..=4 => Ok(None),
            _ => {
                let mut colours = Vec::new();

                for (line_index, s) in s.lines().enumerate() {
                    if !s.is_empty() {
                        let colour =
                            Error::new_from_result_into(Colour::from_str(s, version), line_index)?;
                        if let Some(colour) = colour {
                            colours.push(colour);
                        }
                    }
                }

                Ok(Some(Colours(colours)))
            }
        }
    }
}

impl VersionedToString for Colours {
    fn to_string(&self, version: usize) -> Option<String> {
        match version {
            MIN_VERSION..=4 => None,
            _ => Some(self.0.iter().map_string_new_line()),
        }
    }
}

impl VersionedDefault for Colours {
    fn default(version: usize) -> Option<Self> {
        match version {
            MIN_VERSION..=4 => None,
            _ => Some(Colours(Vec::new())),
        }
    }
}

/// Struct representing a single `colour` component in the `Colours` section.
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
#[non_exhaustive]
pub enum Colour {
    /// Additive combo colours.
    Combo(i32, Rgb),
    /// Additive slider track colour.
    SliderTrackOverride(Rgb),
    /// Slider border colour.
    SliderBorder(Rgb),
}

impl VersionedFromString for Colour {
    type ParseError = ColourParseError;

    fn from_str(s: &str, _: usize) -> Result<Option<Self>, Self::ParseError> {
        let separator = || tuple((space0, tag(":"), space0));
        let combo_type = tag("Combo");
        let combo_count = map_res(digit1, |s: &str| s.parse());
        let slider_track_override_type = tag("SliderTrackOverride");
        let slider_border_type = tag("SliderBorder");
        let byte = || map_res(digit1, |s: &str| s.parse());
        let rgb = || {
            tuple((
                preceded(space0, context(ColourParseError::InvalidRed.into(), byte())),
                preceded(
                    tuple((
                        space0,
                        context(ColourParseError::MissingGreen.into(), comma()),
                        space0,
                    )),
                    context(ColourParseError::InvalidGreen.into(), byte()),
                ),
                preceded(
                    tuple((
                        space0,
                        context(ColourParseError::MissingBlue.into(), comma()),
                        space0,
                    )),
                    context(ColourParseError::InvalidBlue.into(), byte()),
                ),
            ))
            .map(|(red, green, blue)| Rgb { red, green, blue })
        };

        let combo = tuple((
            preceded(
                combo_type,
                context(ColourParseError::InvalidComboCount.into(), cut(combo_count)),
            ),
            cut(preceded(
                context(ColourParseError::InvalidColonSeparator.into(), separator()),
                rgb(),
            )),
        ))
        .map(|(combo, rgb)| Colour::Combo(combo, rgb));

        let slide_track_override = preceded(
            tuple((
                slider_track_override_type,
                context(
                    ColourParseError::InvalidColonSeparator.into(),
                    cut(separator()),
                ),
            )),
            cut(rgb()),
        )
        .map(Colour::SliderTrackOverride);

        let slider_border = preceded(
            tuple((
                slider_border_type,
                context(
                    ColourParseError::InvalidColonSeparator.into(),
                    cut(separator()),
                ),
            )),
            cut(rgb()),
        )
        .map(Colour::SliderBorder);

        let (_, colour) = alt((
            combo,
            slide_track_override,
            slider_border,
            context(ColourParseError::UnknownColourType.into(), fail),
        ))(s)?;

        Ok(Some(colour))
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
