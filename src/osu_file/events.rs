use std::str::FromStr;

use thiserror::Error;

use super::{Integer, Position};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Event {
    pub start_time: Integer,
    pub event_type: EventType,
}

impl FromStr for Event {
    type Err = EventsParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(',');

        let event_type = s.next().ok_or(EventsParseError)
    }
}

#[derive(Debug, Error)]
pub enum EventsParseError {
    #[]
    MissingField(&'static str),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Background {
    pub filename: String,
    // only used to make resulting output match the input file.
    filename_has_quotes: bool,
    pub position: Position,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Video {
    pub filename: String,
    // only used to make resulting output match the input file.
    filename_has_quotes: bool,
    pub position: Position,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Break {
    pub end_time: Integer,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum EventType {
    Background(Background),
    Video(Video),
    Break(Break),
    Storyboard,
}
