pub mod error;
pub mod types;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, space0},
    combinator::{cut, fail, map_res, rest},
    error::context,
    sequence::{preceded, tuple},
    Finish, Parser,
};

use crate::parsers::comma;

pub use error::*;
pub use types::*;

use super::{Error, Version, VersionedDefault, VersionedFromStr, VersionedToString, MIN_VERSION};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Colours(pub Vec<Colour>);

impl VersionedFromStr for Colours {
    type Err = Error<ParseError>;

    fn from_str(s: &str, version: Version) -> std::result::Result<Option<Self>, Self::Err> {
        match version {
            MIN_VERSION..=4 => Ok(None),
            _ => {
                let mut colours = Vec::new();

                for (line_index, s) in s.lines().enumerate() {
                    if s.trim().is_empty() {
                        continue;
                    }

                    let colour =
                        Error::new_from_result_into(Colour::from_str(s, version), line_index)?;
                    if let Some(colour) = colour {
                        colours.push(colour);
                    }
                }

                Ok(Some(Colours(colours)))
            }
        }
    }
}

impl VersionedToString for Colours {
    fn to_string(&self, version: Version) -> Option<String> {
        match version {
            MIN_VERSION..=4 => None,
            _ => Some(
                self.0
                    .iter()
                    .filter_map(|c| c.to_string(version))
                    .collect::<Vec<_>>()
                    .join("\n"),
            ),
        }
    }
}

impl VersionedDefault for Colours {
    fn default(version: Version) -> Option<Self> {
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

impl VersionedFromStr for Colour {
    type Err = ParseColourError;

    fn from_str(s: &str, version: Version) -> Result<Option<Self>, Self::Err> {
        let s = s.trim();

        let separator = || tuple((space0, tag(":"), space0));
        let combo_type = tag("Combo");
        let combo_count = map_res(digit1, |s: &str| s.parse());
        let slider_track_override_type = tag("SliderTrackOverride");
        let slider_border_type = tuple((tag("Slider"), alt((char('B'), char('b'))), tag("order")));
        let rgb_parse_error = "rgb_parse_error";
        let rgb = || {
            context(
                rgb_parse_error,
                map_res(rest, |s: &str| {
                    Rgb::from_str(s, version).map(|rgb| rgb.unwrap())
                }),
            )
        };

        let combo = tuple((
            preceded(
                combo_type,
                context(ParseColourError::InvalidComboCount.into(), cut(combo_count)),
            ),
            cut(preceded(
                context(ParseColourError::InvalidColonSeparator.into(), separator()),
                rgb(),
            )),
        ))
        .map(|(combo, rgb)| Colour::Combo(combo, rgb));

        let slide_track_override = preceded(
            tuple((
                slider_track_override_type,
                context(
                    ParseColourError::InvalidColonSeparator.into(),
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
                    ParseColourError::InvalidColonSeparator.into(),
                    cut(separator()),
                ),
            )),
            cut(rgb()),
        )
        .map(Colour::SliderBorder);

        let colour_res: Result<(_, _), nom::error::VerboseError<&str>> = alt((
            combo,
            slide_track_override,
            slider_border,
            context(ParseColourError::UnknownColourType.into(), fail),
        ))(s)
        .finish();

        match colour_res {
            Ok((_, colour)) => Ok(Some(colour)),
            Err(e) => {
                for (i, e) in &e.errors {
                    if let nom::error::VerboseErrorKind::Context(context) = e {
                        if context == &rgb_parse_error {
                            // re-parse to get actual error message

                            let err = Rgb::from_str(i, MIN_VERSION).unwrap_err();
                            return Err(err.into());
                        }
                    }
                }

                // else just return the error
                Err(nom::Err::Error(e).into())
            }
        }
    }
}

impl VersionedToString for Colour {
    fn to_string(&self, version: Version) -> Option<String> {
        let colour_str = match self {
            Colour::Combo(num, rgb) => format!("Combo{num} : {}", rgb.to_string(version).unwrap()),
            Colour::SliderTrackOverride(rgb) => {
                format!("SliderTrackOverride : {}", rgb.to_string(version).unwrap())
            }
            Colour::SliderBorder(rgb) => {
                format!("SliderBorder : {}", rgb.to_string(version).unwrap())
            }
        };

        Some(colour_str)
    }
}
