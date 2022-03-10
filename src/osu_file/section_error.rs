use std::{error::Error, fmt::Display};

use thiserror::Error;

use super::OsuFileParseError;

// TODO doc
// TODO error boilerplate reduction
// TODO better error

#[derive(Debug)]
pub struct InvalidKey(pub String);

impl Error for InvalidKey {}

impl Display for InvalidKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The input has an invalid key: {}", self.0)
    }
}

#[derive(Debug)]
pub struct MissingValue(pub String);

impl Error for MissingValue {}

impl Display for MissingValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The key doesn't have a value: {}", self.0)
    }
}

#[derive(Debug, Error)]
#[error("There was a problem parsing the osu file section {section_name}")]
pub struct SectionParseError {
    #[source]
    source: Box<dyn Error>,
    section_name: String,
}

impl From<SectionParseError> for OsuFileParseError {
    fn from(err: SectionParseError) -> Self {
        Self::SectionParseError {
            source: err.source,
            section_name: err.section_name,
        }
    }
}
