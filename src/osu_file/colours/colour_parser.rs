use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0},
    combinator::map_res,
    error::{context, VerboseError},
    sequence::tuple,
    IResult, Parser,
};
use strum_macros::{EnumString, IntoStaticStr};

use super::{Colour, Rgb};

pub fn colour(s: &str) -> IResult<&str, Colour, VerboseError<&str>> {
    let combo = tag::<_, _, nom::error::Error<_>>("Combo");
    let slide_track_override = tag::<_, _, nom::error::Error<_>>("SliderTrackOverride");
    let slider_border = tag("SliderBorder");

    let mut rgb_end = tuple((multispace0, char(':'), rgb)).map(|(_, _, rgb)| rgb);

    // test one by one to see if they match
    if let Ok((s, _)) = combo(s) {
        let (s, (count, rgb)) = tuple((
            context(
                Context::InvalidAdditiveComboCount.into(),
                map_res(digit1, |s: &str| s.parse::<u32>()),
            ),
            rgb_end,
        ))(s)?;

        Ok((s, Colour::Combo(count, rgb)))
    } else if let Ok((s, _)) = slide_track_override(s) {
        let (s, rgb) = rgb_end.parse(s)?;

        Ok((s, Colour::SliderTrackOverride(rgb)))
    } else {
        let (s, (_, rgb)) = tuple((
            context(Context::UnknownColourOption.into(), slider_border),
            rgb_end,
        ))(s)?;

        Ok((s, Colour::SliderBorder(rgb)))
    }
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
    UnknownColourOption,
    InvalidAdditiveComboCount,
    InvalidRed,
    InvalidGreen,
    InvalidBlue,
    NoCommaAfterRed,
    NoCommaAfterGreen,
}
