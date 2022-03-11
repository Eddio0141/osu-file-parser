use std::{error::Error, fmt::Display, str::FromStr};

use rust_decimal::Decimal;
use thiserror::Error;

use super::{Integer, SECTION_DELIMITER};

/// A struct representing the editor section of thye .osu file.
#[derive(Default, Debug, PartialEq, Clone, Hash, Eq)]
pub struct Editor {
    /// Time in milliseconds of bookmarks.
    pub bookmarks: Vec<Integer>,
    /// Distance snap multiplier.
    pub distance_spacing: Decimal,
    /// Beat snap divisor.
    pub beat_divisor: Decimal,
    /// Grid size.
    pub grid_size: Integer,
    /// Scale factor for the objecct timeline.
    pub timeline_zoom: Decimal,
}

impl FromStr for Editor {
    type Err = EditorParseError;

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
                                    return Err(EditorParseError::SectionParseError {
                                        source: Box::new(err),
                                        name: "Bookmarks",
                                    })
                                }
                                None => editor.bookmarks = bookmarks,
                            }
                        }
                        "DistanceSpacing" => {
                            editor.distance_spacing = value.parse().map_err(|err| {
                                EditorParseError::SectionParseError {
                                    source: Box::new(err),
                                    name: "DistanceSpacing",
                                }
                            })?
                        }
                        "BeatDivisor" => {
                            editor.beat_divisor = value.parse().map_err(|err| {
                                EditorParseError::SectionParseError {
                                    source: Box::new(err),
                                    name: "BeatDivisor",
                                }
                            })?
                        }
                        "GridSize" => {
                            editor.grid_size = value.parse().map_err(|err| {
                                EditorParseError::SectionParseError {
                                    source: Box::new(err),
                                    name: "GridSize",
                                }
                            })?
                        }
                        "TimelineZoom" => {
                            editor.timeline_zoom = value.parse().map_err(|err| {
                                EditorParseError::SectionParseError {
                                    source: Box::new(err),
                                    name: "TimelineZoom",
                                }
                            })?
                        }
                        _ => return Err(EditorParseError::InvalidKey(key.to_string())),
                    }
                }
                None => return Err(EditorParseError::MissingValue(line.to_string())),
            }
        }

        Ok(editor)
    }
}

impl Display for Editor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut key_value = Vec::new();

        key_value.push(format!(
            "Bookmarks: {}",
            self.bookmarks
                .iter()
                .map(|bookmark| bookmark.to_string())
                .collect::<Vec<_>>()
                .join(",")
        ));
        key_value.push(format!("DistanceSpacing: {}", self.distance_spacing));
        key_value.push(format!("BeatDivisor: {}", self.beat_divisor));
        key_value.push(format!("GridSize: {}", self.grid_size));
        key_value.push(format!("TimelineZoom: {}", self.timeline_zoom));

        write!(f, "{}", key_value.join("\r\n"))
    }
}

#[derive(Debug, Error)]
#[error("Error parsing a key: value in Editor")]
pub enum EditorParseError {
    #[error("There was a problem parsing the `{name}` property from a `str`")]
    /// A section in `Editor` failed to parse.
    SectionParseError {
        #[source]
        source: Box<dyn Error>,
        name: &'static str,
    },
    #[error("The key {0} doesn't exist in `General`")]
    /// Invalid key name was used.
    InvalidKey(String),
    #[error("The key {0} has no value set")]
    /// The value is missing from the `key: value` set.
    MissingValue(String),
}
