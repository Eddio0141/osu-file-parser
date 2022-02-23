use std::{error::Error, fmt::Display};

// TODO more specific info
#[derive(Debug)]
pub struct InvalidKey;

impl Error for InvalidKey {}

impl Display for InvalidKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The input has an invalid key")
    }
}

// TODO more specific info
#[derive(Debug)]
pub struct MissingValue;

impl Error for MissingValue {}

impl Display for MissingValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The key doesn't have a value")
    }
}
