use thiserror::Error;

use std::error::Error;

#[derive(Debug, Error)]
#[error("Error parsing a key: value in Editor")]
/// Error used when there was a problem parsing the `Editor` section.
pub enum ParseError {
    #[error("There was a problem parsing the `{name}` property from a `str`")]
    /// A section in `Editor` failed to parse.
    SectionParseError {
        #[source]
        source: Box<dyn Error>,
        name: &'static str,
    },
    #[error("The key {0} doesn't exist in `Editor`")]
    /// Invalid key name was used.
    InvalidKey(String),
    #[error("The key {0} has no value set")]
    /// The value is missing from the `key: value` set.
    MissingValue(String),
}
