use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0},
    combinator::{cut, map_res},
    error::{context, VerboseError},
    sequence::{preceded, tuple},
    IResult, Parser,
};
use strum_macros::{EnumString, IntoStaticStr};

use super::{Colour, Rgb};

pub fn colour(s: &str) -> IResult<&str, Colour, VerboseError<&str>> {
    // TODO is this i32?
    let separator = || tuple((multispace0, tag(":"), multispace0));
    let combo_type = tag("Combo");
    let combo_count = map_res(digit1, |s: &str| s.parse::<u32>());
    let slider_track_override_type = tag("SliderTrackOverride");
    let slider_border_type = tag("SliderBorder");

    let combo = tuple((
        preceded(combo_type, cut(combo_count)),
        cut(preceded(separator(), rgb)),
    ))
    .map(|(combo, rgb)| Colour::Combo(combo, rgb));
    let slide_track_override = preceded(
        tuple((slider_track_override_type, cut(separator()))),
        cut(rgb),
    )
    .map(|rgb| Colour::SliderTrackOverride(rgb));
    let slider_border = preceded(tuple((slider_border_type, cut(separator()))), cut(rgb))
        .map(|rgb| Colour::SliderBorder(rgb));

    alt((combo, slide_track_override, slider_border))(s)
}

pub fn rgb(s: &str) -> IResult<&str, Rgb, VerboseError<&str>> {
    let byte = || map_res(digit1, |s: &str| s.parse::<u8>());

    let (s, (_, red, _, _, _, green, _, _, _, blue)) = tuple((
        multispace0,
        context(Context::InvalidRed.into(), byte()),
        multispace0,
        context(Context::NoCommaAfterRed.into(), char(',')),
        multispace0,
        context(Context::InvalidGreen.into(), byte()),
        multispace0,
        context(Context::NoCommaAfterGreen.into(), char(',')),
        multispace0,
        context(Context::InvalidBlue.into(), byte()),
    ))(s)?;

    Ok((s, Rgb { red, green, blue }))
}

#[derive(Debug, EnumString, IntoStaticStr)]
pub enum Context {
    InvalidColonSeparator,
    UnknownColourOption,
    InvalidAdditiveComboCount,
    InvalidRed,
    InvalidGreen,
    InvalidBlue,
    NoCommaAfterRed,
    NoCommaAfterGreen,
}
