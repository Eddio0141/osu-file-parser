use std::str::FromStr;

use thiserror::Error;

#[derive(Default)]
pub struct Events;

impl FromStr for Events {
    type Err = EventsParseError;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Error)]
#[error("")]
pub struct EventsParseError;
