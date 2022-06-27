use nom::{
    branch::alt,
    bytes::complete::tag,
    error::context,
    sequence::{preceded, tuple},
    Parser,
};

use crate::{
    osu_file::{FilePath, Integer, Position, VersionedFromString, VersionedToString},
    parsers::{comma, comma_field_type, consume_rest_type},
};

pub use self::error::*;

use super::OLD_VERSION_TIME_OFFSET;

pub mod error;
mod parsers;
use parsers::*;

fn position_str(position: &Option<Position>) -> String {
    match position {
        Some(position) => format!(",{},{}", position.x, position.y),
        None => String::new(),
    }
}

fn time_to_string(time: Integer, version: usize) -> String {
    let time = if (3..=4).contains(&version) {
        time - OLD_VERSION_TIME_OFFSET
    } else {
        time
    };

    time.to_string()
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Background {
    pub start_time: Integer,
    pub file_name: FilePath,
    pub position: Option<Position>,
}

impl VersionedFromString for Background {
    type ParseError = BackgroundParseError;

    fn from_str(s: &str, _: usize) -> std::result::Result<Option<Self>, Self::ParseError> {
        let (_, (start_time, (filename, position))) = preceded(
            tuple((
                context(BackgroundParseError::WrongEventType.into(), tag("0")),
                context(BackgroundParseError::MissingStartTime.into(), comma()),
            )),
            tuple((
                context(
                    BackgroundParseError::InvalidStartTime.into(),
                    comma_field_type(),
                ),
                preceded(
                    context(BackgroundParseError::MissingFileName.into(), comma()),
                    file_name_and_position(
                        BackgroundParseError::MissingX.into(),
                        BackgroundParseError::InvalidX.into(),
                        BackgroundParseError::MissingY.into(),
                        BackgroundParseError::InvalidY.into(),
                    ),
                ),
            )),
        )(s)?;

        Ok(Some(Background {
            start_time,
            file_name: filename,
            position,
        }))
    }
}

impl VersionedToString for Background {
    fn to_string(&self, _: usize) -> Option<String> {
        Some(format!(
            "0,{},{}{}",
            self.start_time,
            self.file_name,
            position_str(&self.position),
        ))
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Video {
    pub start_time: Integer,
    pub file_name: FilePath,
    pub position: Option<Position>,
    short_hand: bool,
}

impl VersionedFromString for Video {
    type ParseError = VideoParseError;

    fn from_str(s: &str, _: usize) -> std::result::Result<Option<Self>, Self::ParseError> {
        let (_, (short_hand, start_time, (file_name, position))) = tuple((
            alt((
                tag("1").map(|_| true),
                context(
                    VideoParseError::WrongEventType.into(),
                    tag("Video").map(|_| false),
                ),
            )),
            preceded(
                context(VideoParseError::MissingStartTime.into(), comma()),
                context(VideoParseError::InvalidStartTime.into(), comma_field_type()),
            ),
            preceded(
                context(VideoParseError::MissingFileName.into(), comma()),
                file_name_and_position(
                    VideoParseError::MissingX.into(),
                    VideoParseError::InvalidX.into(),
                    VideoParseError::MissingY.into(),
                    VideoParseError::InvalidY.into(),
                ),
            ),
        ))(s)?;

        Ok(Some(Video {
            start_time,
            file_name,
            position,
            short_hand,
        }))
    }
}

// TODO replace all FromStr and Display with versioned variant for future compatibility
// TODO enum non exhastive check
impl VersionedToString for Video {
    fn to_string(&self, version: usize) -> Option<String> {
        Some(format!(
            "{},{},{}{}",
            if self.short_hand { "1" } else { "Video" },
            time_to_string(self.start_time, version),
            self.file_name,
            position_str(&self.position)
        ))
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Break {
    pub start_time: Integer,
    pub end_time: Integer,
    short_hand: bool,
}

impl VersionedFromString for Break {
    type ParseError = BreakParseError;

    fn from_str(s: &str, version: usize) -> std::result::Result<Option<Self>, Self::ParseError> {
        let (_, (short_hand, start_time, end_time)) = tuple((
            alt((
                tag("2").map(|_| true),
                context(
                    BreakParseError::WrongEventType.into(),
                    tag("Break").map(|_| false),
                ),
            )),
            preceded(
                context(BreakParseError::MissingStartTime.into(), comma()),
                context(BreakParseError::InvalidStartTime.into(), comma_field_type()),
            ),
            preceded(
                context(BreakParseError::MissingEndTime.into(), comma()),
                end_time(BreakParseError::InvalidEndTime.into(), version),
            ),
        ))(s)?;

        Ok(Some(Break {
            start_time,
            end_time,
            short_hand,
        }))
    }
}

impl VersionedToString for Break {
    fn to_string(&self, version: usize) -> Option<String> {
        Some(format!(
            "{},{},{}",
            if self.short_hand { "2" } else { "Break" },
            time_to_string(self.start_time, version),
            self.end_time
        ))
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ColourTransformation {
    pub start_time: Integer,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl VersionedFromString for ColourTransformation {
    type ParseError = ColourTransformationParseError;

    fn from_str(s: &str, _: usize) -> std::result::Result<Option<Self>, Self::ParseError> {
        let (_, (start_time, red, green, blue)) = tuple((
            preceded(
                tuple((
                    context(
                        ColourTransformationParseError::WrongEventType.into(),
                        tag("3"),
                    ),
                    context(
                        ColourTransformationParseError::MissingStartTime.into(),
                        comma(),
                    ),
                )),
                context(
                    ColourTransformationParseError::InvalidStartTime.into(),
                    comma_field_type(),
                ),
            ),
            preceded(
                context(ColourTransformationParseError::MissingRed.into(), comma()),
                context(
                    ColourTransformationParseError::InvalidRed.into(),
                    comma_field_type(),
                ),
            ),
            preceded(
                context(ColourTransformationParseError::MissingGreen.into(), comma()),
                context(
                    ColourTransformationParseError::InvalidGreen.into(),
                    comma_field_type(),
                ),
            ),
            preceded(
                context(ColourTransformationParseError::MissingBlue.into(), comma()),
                context(
                    ColourTransformationParseError::InvalidBlue.into(),
                    consume_rest_type(),
                ),
            ),
        ))(s)?;

        Ok(Some(ColourTransformation {
            start_time,
            red,
            green,
            blue,
        }))
    }
}

impl VersionedToString for ColourTransformation {
    fn to_string(&self, version: usize) -> Option<String> {
        if version < 14 {
            Some(format!(
                "3,{},{},{},{}",
                time_to_string(self.start_time, version),
                self.red,
                self.green,
                self.blue
            ))
        } else {
            None
        }
    }
}
