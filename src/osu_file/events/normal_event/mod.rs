use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{cut, eof, fail},
    error::context,
    sequence::{preceded, terminated, tuple},
    Parser,
};

use crate::{
    osu_file::{FilePath, Integer, Position, VersionedFromString, VersionedToString},
    parsers::{comma, comma_field, comma_field_type, consume_rest_type},
};

pub use self::error::*;

use super::OLD_VERSION_TIME_OFFSET;

pub mod error;
mod parsers;

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



// impl VersionedFromString for NormalEventParams {
//     type ParseError = NormalEventParamsParseError;

//     fn from_str(s: &str, version: usize) -> std::result::Result<Option<Self>, Self::ParseError> {
//         let start_time = comma_field;
//         let file_name = || comma_field().map(|f| f.into());
//         let coordinates = || {
//             alt((
//                 eof.map(|_| None),
//                 tuple((
//                     preceded(
//                         context(NormalEventParamsParseError::MissingXOffset.into(), comma()),
//                         context(
//                             NormalEventParamsParseError::InvalidXOffset.into(),
//                             comma_field_type(),
//                         ),
//                     ),
//                     preceded(
//                         context(NormalEventParamsParseError::MissingYOffset.into(), comma()),
//                         context(
//                             NormalEventParamsParseError::InvalidYOffset.into(),
//                             consume_rest_type(),
//                         ),
//                     ),
//                 ))
//                 .map(|(x, y)| Some((x, y))),
//             ))
//             .map(|position| position.map(|(x, y)| Position { x, y }))
//         };
//         let file_name_and_coordinates = || tuple((file_name(), coordinates()));
//         let end_time = consume_rest_type().map(|end_time| {
//             if (3..=4).contains(&version) {
//                 end_time + OLD_VERSION_TIME_OFFSET
//             } else {
//                 end_time
//             }
//         });

//         let background = preceded(
//             tuple((
//                 tag("0"),
//                 cut(tuple((
//                     context(
//                         NormalEventParamsParseError::MissingStartTime.into(),
//                         comma(),
//                     ),
//                     start_time(),
//                     context(NormalEventParamsParseError::MissingFileName.into(), comma()),
//                 ))),
//             )),
//             cut(file_name_and_coordinates()),
//         )
//         .map(|(filename, position)| {
//             NormalEventParams::Background(Background { filename, position })
//         });
//         let video = tuple((
//             terminated(
//                 alt((tag("1").map(|_| true), tag("Video").map(|_| false))),
//                 cut(tuple((
//                     context(
//                         NormalEventParamsParseError::MissingStartTime.into(),
//                         comma(),
//                     ),
//                     start_time(),
//                     context(NormalEventParamsParseError::MissingFileName.into(), comma()),
//                 ))),
//             ),
//             cut(file_name_and_coordinates()),
//         ))
//         .map(|(short_hand, (filename, position))| {
//             NormalEventParams::Video(Video {
//                 filename,
//                 position,
//                 short_hand,
//             })
//         });
//         let break_ = tuple((
//             terminated(
//                 alt((tag("2").map(|_| true), tag("Break").map(|_| false))),
//                 cut(tuple((
//                     context(
//                         NormalEventParamsParseError::MissingStartTime.into(),
//                         comma(),
//                     ),
//                     start_time(),
//                     context(NormalEventParamsParseError::MissingEndTime.into(), comma()),
//                 ))),
//             ),
//             cut(context(
//                 NormalEventParamsParseError::InvalidEndTime.into(),
//                 end_time,
//             )),
//         ))
//         .map(|(short_hand, end_time)| {
//             NormalEventParams::Break(Break {
//                 end_time,
//                 short_hand,
//             })
//         });
//         let colour_transformation = preceded(
//             tuple((
//                 tag("3"),
//                 cut(tuple((
//                     context(
//                         NormalEventParamsParseError::MissingStartTime.into(),
//                         comma(),
//                     ),
//                     start_time(),
//                     context(NormalEventParamsParseError::MissingRed.into(), comma()),
//                 ))),
//             )),
//             cut(tuple((
//                 context(
//                     NormalEventParamsParseError::InvalidRed.into(),
//                     comma_field_type(),
//                 ),
//                 preceded(
//                     context(NormalEventParamsParseError::MissingGreen.into(), comma()),
//                     context(
//                         NormalEventParamsParseError::InvalidGreen.into(),
//                         comma_field_type(),
//                     ),
//                 ),
//                 preceded(
//                     context(NormalEventParamsParseError::MissingBlue.into(), comma()),
//                     context(
//                         NormalEventParamsParseError::InvalidBlue.into(),
//                         consume_rest_type(),
//                     ),
//                 ),
//             ))),
//         )
//         .map(|(red, green, blue)| {
//             NormalEventParams::ColourTransformation(ColourTransformation { red, green, blue })
//         });

//         let result = alt((
//             background,
//             video,
//             break_,
//             colour_transformation,
//             context(NormalEventParamsParseError::UnknownEventType.into(), fail),
//         ))(s)?;

//         Ok(Some(result.1))
//     }
// }

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Background {
    pub start_time: Integer,
    pub filename: FilePath,
    pub position: Option<Position>,
}

impl VersionedFromString for Background {
    type ParseError = BackgroundParseError;

    fn from_str(s: &str, _: usize) -> std::result::Result<Option<Self>, Self::ParseError> {
        let (_, (filename, start_time, position)) = preceded(
            tuple((
                tag("0"),
                cut(tuple((
                    context(BackgroundParseError::MissingStartTime.into(), comma()),
                    context(
                        BackgroundParseError::InvalidStartTime.into(),
                        comma_field_type(),
                    ),
                    context(BackgroundParseError::MissingFileName.into(), comma()),
                ))),
            )),
            cut(file_name_and_coordinates()),
        )(s)?;

        Ok(Some(Background {
            start_time,
            filename,
            position,
        }))
    }
}

impl VersionedToString for Background {
    fn to_string(&self, _: usize) -> Option<String> {
        Some(format!(
            "0,{},{}{}",
            self.start_time,
            self.filename,
            position_str(&self.position),
        ))
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Video {
    pub start_time: Integer,
    pub filename: FilePath,
    pub position: Option<Position>,
    short_hand: bool,
}

// TODO replace all FromStr and Display with versioned variant for future compatibility
// TODO enum non exhastive check
impl VersionedToString for Video {
    fn to_string(&self, version: usize) -> Option<String> {
        Some(format!(
            "{},{},{}{}",
            if self.short_hand { "1" } else { "Video" },
            time_to_string(self.start_time, version),
            self.filename,
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
