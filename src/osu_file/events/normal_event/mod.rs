use nom::{
    branch::alt,
    bytes::complete::tag,
    error::context,
    sequence::{preceded, tuple},
    Parser,
};

use crate::{
    osu_file::{FilePath, Integer, Position, VersionedFromStr, VersionedToString},
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

pub const BACKGROUND_HEADER: &str = "0";

impl VersionedFromStr for Background {
    type Err = BackgroundParseError;

    fn from_str(s: &str, _: usize) -> std::result::Result<Option<Self>, Self::Err> {
        let (_, (start_time, (filename, position))) = preceded(
            tuple((
                context(
                    BackgroundParseError::WrongEventType.into(),
                    tag(BACKGROUND_HEADER),
                ),
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
    fn to_string(&self, version: usize) -> Option<String> {
        Some(format!(
            "{BACKGROUND_HEADER},{},{}{}",
            self.start_time,
            self.file_name.to_string(version).unwrap(),
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

impl Video {
    pub fn new(start_time: Integer, file_name: FilePath, position: Option<Position>) -> Self {
        Self {
            start_time,
            file_name,
            position,
            short_hand: true,
        }
    }
}

pub const VIDEO_HEADER: &str = "1";
pub const VIDEO_HEADER_LONG: &str = "Video";

impl VersionedFromStr for Video {
    type Err = VideoParseError;

    fn from_str(s: &str, version: usize) -> std::result::Result<Option<Self>, Self::Err> {
        let (_, (short_hand, start_time, (file_name, position))) = tuple((
            alt((
                tag(VIDEO_HEADER).map(|_| true),
                context(
                    VideoParseError::WrongEventType.into(),
                    tag(VIDEO_HEADER_LONG).map(|_| false),
                ),
            )),
            preceded(
                context(VideoParseError::MissingStartTime.into(), comma()),
                start_time_offset(VideoParseError::InvalidStartTime.into(), version),
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

// TODO enum non exhastive check
impl VersionedToString for Video {
    fn to_string(&self, version: usize) -> Option<String> {
        Some(format!(
            "{},{},{}{}",
            if self.short_hand {
                VIDEO_HEADER
            } else {
                VIDEO_HEADER_LONG
            },
            time_to_string(self.start_time, version),
            self.file_name.to_string(version).unwrap(),
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

impl Break {
    pub fn new(start_time: Integer, end_time: Integer) -> Self {
        Self {
            start_time,
            end_time,
            short_hand: true,
        }
    }
}

pub const BREAK_HEADER: &str = "2";
pub const BREAK_HEADER_LONG: &str = "Break";

impl VersionedFromStr for Break {
    type Err = BreakParseError;

    fn from_str(s: &str, version: usize) -> std::result::Result<Option<Self>, Self::Err> {
        let (_, (short_hand, start_time, end_time)) = tuple((
            alt((
                tag(BREAK_HEADER).map(|_| true),
                context(
                    BreakParseError::WrongEventType.into(),
                    tag(BREAK_HEADER_LONG).map(|_| false),
                ),
            )),
            preceded(
                context(BreakParseError::MissingStartTime.into(), comma()),
                start_time_offset(BreakParseError::InvalidStartTime.into(), version),
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
            if self.short_hand {
                BREAK_HEADER
            } else {
                BREAK_HEADER_LONG
            },
            time_to_string(self.start_time, version),
            time_to_string(self.end_time, version)
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

pub const COLOUR_TRANSFORMATION_HEADER: &str = "3";

impl VersionedFromStr for ColourTransformation {
    type Err = ColourTransformationParseError;

    fn from_str(s: &str, version: usize) -> std::result::Result<Option<Self>, Self::Err> {
        let (_, (start_time, red, green, blue)) = tuple((
            preceded(
                tuple((
                    context(
                        ColourTransformationParseError::WrongEventType.into(),
                        tag(COLOUR_TRANSFORMATION_HEADER),
                    ),
                    context(
                        ColourTransformationParseError::MissingStartTime.into(),
                        comma(),
                    ),
                )),
                start_time_offset(
                    ColourTransformationParseError::InvalidStartTime.into(),
                    version,
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
                "{COLOUR_TRANSFORMATION_HEADER},{},{},{},{}",
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
