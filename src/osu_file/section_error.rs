use std::{
    error::Error,
    fmt::Display,
    num::{ParseFloatError, ParseIntError},
};

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

#[derive(Debug)]
pub struct SectionParseError {
    source: Box<dyn Error>,
}

impl SectionParseError {
    pub fn new(err: Box<dyn Error>) -> Self {
        Self { source: err }
    }
}

impl Display for SectionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "There was a problem parsing a section")
    }
}

impl Error for SectionParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.source.as_ref())
    }
}

impl From<ParseIntError> for SectionParseError {
    fn from(err: ParseIntError) -> Self {
        Self {
            source: Box::new(err),
        }
    }
}

impl From<ParseFloatError> for SectionParseError {
    fn from(err: ParseFloatError) -> Self {
        Self {
            source: Box::new(err),
        }
    }
}
