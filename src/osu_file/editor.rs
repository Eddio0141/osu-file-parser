use std::{error::Error, fmt::Display, str::FromStr};

use rust_decimal::Decimal;

use super::Integer;

/// A struct representing the editor section of thye .osu file
#[derive(Default, Debug, PartialEq)]
pub struct Editor {
    pub bookmarks: Vec<Bookmark>,
    pub distance_spacing: Decimal,
    pub beat_divisor: Decimal,
    pub grid_size: Integer,
    pub timeline_zoom: Decimal,
}

impl FromStr for Editor {
    type Err = EditorParseError;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug)]
pub struct EditorKeyParseError {
    source: Box<dyn Error>,
    pub line: String,
}

impl Display for EditorKeyParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error parsing a key: value in Editor")
    }
}

impl Error for EditorKeyParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.source.as_ref())
    }
}

#[derive(Debug)]
pub struct EditorParseError {
    source: EditorKeyParseError,
}

impl Display for EditorParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error parsing the `Editor` section")
    }
}

impl Error for EditorParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

impl Display for Editor {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

/// A struct representing a single bookmark for Editor section
#[derive(Debug, PartialEq, Eq)]
pub struct Bookmark(Integer);
