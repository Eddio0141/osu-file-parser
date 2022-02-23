use std::{error::Error, fmt::Display};

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
