use nom::{
    character::complete::{char, digit1, multispace0},
    combinator::map_res,
    error::{context, VerboseError},
    sequence::tuple,
    IResult,
};
use strum_macros::{EnumString, IntoStaticStr};

use super::Rgb;

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
