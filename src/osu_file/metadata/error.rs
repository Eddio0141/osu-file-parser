use thiserror::Error;

use std::num::ParseIntError;

#[derive(Debug, Error)]
/// Error used when there was a problem parsing the `Metadata` section.
pub enum MetadataParseError {
    #[error("There was a problem parsing the `{name}` property from a `str` to an `Integer`")]
    /// A section in `Metadata` failed to parse.
    SectionParseError {
        #[source]
        source: ParseIntError,
        name: &'static str,
    },
    #[error("The key {0} doesn't exist in `Metadata`")]
    /// Invalid key name was used.
    InvalidKey(String),
    #[error("The key {0} has no value set")]
    /// The value is missing from the `key:value` set.
    MissingValue(String),
}
