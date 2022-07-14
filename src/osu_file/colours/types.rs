use nom::{
    error::context,
    sequence::{preceded, tuple},
};

use crate::parsers::consume_rest_type;

use super::*;

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

impl VersionedFromStr for Rgb {
    type Err = RgbParseError;

    fn from_str(s: &str, _: usize) -> Result<Option<Self>, Self::Err> {
        let byte = || map_res(digit1, |s: &str| s.parse());

        let (_, (red, green, blue)) = tuple((
            preceded(space0, context(RgbParseError::InvalidRed.into(), byte())),
            preceded(
                tuple((
                    space0,
                    context(RgbParseError::MissingGreen.into(), comma()),
                    space0,
                )),
                context(RgbParseError::InvalidGreen.into(), byte()),
            ),
            preceded(
                tuple((
                    space0,
                    context(RgbParseError::MissingBlue.into(), comma()),
                    space0,
                )),
                context(RgbParseError::InvalidBlue.into(), consume_rest_type()),
            ),
        ))(s)?;

        Ok(Some(Rgb { red, green, blue }))
    }
}

impl VersionedToString for Rgb {
    fn to_string(&self, _: usize) -> Option<String> {
        Some(format!("{},{},{}", self.red, self.green, self.blue))
    }
}
