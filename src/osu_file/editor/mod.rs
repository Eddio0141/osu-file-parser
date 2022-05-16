pub mod error;

use std::{fmt::Display, str::FromStr};

use rust_decimal::Decimal;

use super::{Integer, SECTION_DELIMITER};

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

impl FromStr for Editor {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut editor = Editor::default();

        let s = s.trim();

        for line in s.lines() {
            match line.split_once(SECTION_DELIMITER) {
                Some((key, value)) => {
                    let value = value.trim();

                    match key.trim() {
                        "Bookmarks" => {
                            let mut err = None;
                            let mut bookmarks = Vec::new();

                            for bookmark in value.split(',') {
                                let bookmark = bookmark.parse();

                                match bookmark {
                                    Ok(bookmark) => bookmarks.push(bookmark),
                                    Err(e) => {
                                        err = Some(e);
                                        break;
                                    }
                                }
                            }

                            match err {
                                Some(err) => {
                                    return Err(ParseError::SectionParseError {
                                        source: Box::new(err),
                                        name: "Bookmarks",
                                    })
                                }
                                None => editor.bookmarks = Some(bookmarks),
                            }
                        }
                        "DistanceSpacing" => {
                            editor.distance_spacing = Some(value.parse().map_err(|err| {
                                ParseError::SectionParseError {
                                    source: Box::new(err),
                                    name: "DistanceSpacing",
                                }
                            })?)
                        }
                        "BeatDivisor" => {
                            editor.beat_divisor = Some(value.parse().map_err(|err| {
                                ParseError::SectionParseError {
                                    source: Box::new(err),
                                    name: "BeatDivisor",
                                }
                            })?)
                        }
                        "GridSize" => {
                            editor.grid_size = Some(value.parse().map_err(|err| {
                                ParseError::SectionParseError {
                                    source: Box::new(err),
                                    name: "GridSize",
                                }
                            })?)
                        }
                        "TimelineZoom" => {
                            editor.timeline_zoom = Some(value.parse().map_err(|err| {
                                ParseError::SectionParseError {
                                    source: Box::new(err),
                                    name: "TimelineZoom",
                                }
                            })?)
                        }
                        _ => return Err(ParseError::InvalidKey(key.to_string())),
                    }
                }
                None => return Err(ParseError::MissingValue(line.to_string())),
            }
        }

        Ok(editor)
    }
}

impl Display for Editor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut key_value = Vec::new();

        if let Some(bookmarks) = &self.bookmarks {
            writeln!(
                f,
                "Bookmarks: {}",
                bookmarks
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(","),
            )?;
        }
        key_value.push((
            "DistanceSpacing",
            self.distance_spacing
                .map(|distance_spacing| distance_spacing.to_string()),
        ));
        key_value.push((
            "BeatDivisor",
            self.beat_divisor
                .map(|beat_divisor| beat_divisor.to_string()),
        ));
        key_value.push(("GridSize", self.grid_size.map(|grid| grid.to_string())));
        key_value.push((
            "TimelineZoom",
            self.timeline_zoom.map(|zoom| zoom.to_string()),
        ));

        write!(
            f,
            "{}",
            key_value
                .iter()
                .filter_map(|(k, v)| v.as_ref().map(|v| format!("{k}:{v}")))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}
