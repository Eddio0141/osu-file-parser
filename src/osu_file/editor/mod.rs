pub mod error;

use nom::{bytes::complete::take_till, combinator::map_res, multi::separated_list0, Finish};
use rust_decimal::Decimal;

use crate::{
    helper::display_colon_fields,
    parsers::{comma, get_colon_field_value_lines},
};

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
        let mut parsed_fields = Vec::new();

        for (name, value, ws) in fields {
            let new_into_int = move |err| Error::new_into(err, line_count);
            let new_into_decimal = move |err| Error::new_into(err, line_count);

            if parsed_fields.contains(&name) {
                return Err(Error::new(ParseError::DuplicateField, line_count));
            }

            match name {
                "Bookmarks" => {
                    let bookmark = map_res(take_till(|c| c == ','), |s: &str| s.parse::<Integer>());
                    let mut bookmarks = separated_list0(comma::<nom::error::Error<_>>(), bookmark);

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
            parsed_fields.push(name);
        }

        Ok(Some(editor))
    }

    fn to_string_v3(&self) -> String {
        let bookmarks = self.bookmarks.as_ref().map(|b| {
            b.iter()
                .map(|b| b.to_string())
                .collect::<Vec<_>>()
                .join(",")
        });
        let distance_spacing = self.distance_spacing.as_ref().map(|d| d.to_string());
        let beat_divisor = self.beat_divisor.as_ref().map(|d| d.to_string());
        let grid_size = self.grid_size.as_ref().map(|g| g.to_string());
        let timeline_zoom = self.timeline_zoom.as_ref().map(|t| t.to_string());

        let fields = vec![
            ("Bookmarks", &bookmarks),
            ("DistanceSpacing", &distance_spacing),
            ("BeatDivisor", &beat_divisor),
            ("GridSize", &grid_size),
            ("TimelineZoom", &timeline_zoom),
        ];

        display_colon_fields(&fields, true)
    }
}
