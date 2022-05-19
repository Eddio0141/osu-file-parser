pub mod error;

use nom::{
    bytes::complete::{tag, take_till},
    combinator::map_res,
    multi::separated_list0,
    Finish,
};
use rust_decimal::Decimal;

use crate::parsers::get_colon_field_value_lines;

use super::{Error, Integer, Version};

pub use self::error::*;

/// A struct representing the editor section of the .osu file.
#[derive(Default, Debug, PartialEq, Clone, Hash, Eq)]
pub struct Editor {
    /// Time in milliseconds of bookmarks.
    pub bookmarks: Option<Vec<Integer>>,
    /// Distance snap multiplier.
    pub distance_spacing: Option<Decimal>,
    /// Beat snap divisor.
    pub beat_divisor: Option<Decimal>,
    /// Grid size.
    pub grid_size: Option<Integer>,
    /// Scale factor for the objecct timeline.
    pub timeline_zoom: Option<Decimal>,
}

impl Version for Editor {
    type ParseError = Error<ParseError>;

    // TODO check versions
    fn from_str_v3(s: &str) -> std::result::Result<Option<Self>, Self::ParseError>
    where
        Self: Sized,
    {
        let mut editor = Editor::default();

        let (s, fields) = get_colon_field_value_lines(s).unwrap();

        if !s.trim().is_empty() {
            // line count from fields
            let line_count = { fields.iter().map(|(_, _, ws)| ws.lines().count()).sum() };

            return Err(Error::new(ParseError::InvalidColonSet, line_count));
        }

        let mut line_count = 0;

        for (name, value, ws) in fields {
            let new_into_int = move |err| Error::new_into(err, line_count);
            let new_into_decimal = move |err| Error::new_into(err, line_count);

            match name {
                "Bookmarks" => {
                    let comma = tag::<_, _, nom::error::Error<_>>(",");
                    let bookmark = map_res(take_till(|c| c == ','), |s: &str| s.parse::<Integer>());
                    let mut bookmarks = separated_list0(comma, bookmark);

                    let (_, bookmarks) = match bookmarks(value).finish() {
                        Ok(ok) => ok,
                        Err(err) => {
                            let err = match err.code {
                                // TODO test those errors
                                nom::error::ErrorKind::SeparatedList => {
                                    ParseError::InvalidCommaList
                                }
                                nom::error::ErrorKind::MapRes => {
                                    // get section of the input that caused the error, and re-parse to get error
                                    let err = err.input.parse::<Integer>().unwrap_err();

                                    ParseError::ParseIntError(err)
                                }
                                _ => unimplemented!(),
                            };

                            return Err(Error::new(err, line_count));
                        }
                    };

                    editor.bookmarks = Some(bookmarks);
                }
                "DistanceSpacing" => {
                    editor.distance_spacing = Some(value.parse().map_err(new_into_decimal)?)
                }
                "BeatDivisor" => {
                    editor.beat_divisor = Some(value.parse().map_err(new_into_decimal)?)
                }
                "GridSize" => editor.grid_size = Some(value.parse().map_err(new_into_int)?),
                "TimelineZoom" => {
                    editor.timeline_zoom = Some(value.parse().map_err(new_into_decimal)?)
                }
                _ => return Err(Error::new(ParseError::InvalidKey, line_count)),
            }

            line_count += ws.lines().count();
        }

        Ok(Some(editor))
    }

    fn to_string_v3(&self) -> String {
        let mut key_value = Vec::new();

        if let Some(bookmarks) = &self.bookmarks {
            key_value.push(format!(
                "Bookmarks: {}",
                bookmarks
                    .iter()
                    .map(|b| b.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            ));
        }
        if let Some(spacing) = &self.distance_spacing {
            key_value.push(format!("DistanceSpacing: {spacing}"));
        }
        if let Some(divisor) = &self.beat_divisor {
            key_value.push(format!("BeatDivisor: {divisor}"));
        }
        if let Some(grid_size) = &self.grid_size {
            key_value.push(format!("GridSize: {grid_size}"));
        }
        if let Some(zoom) = &self.timeline_zoom {
            key_value.push(format!("TimelineZoom: {zoom}"));
        }

        key_value.join("\n")
    }
}
