use thiserror::Error;

#[derive(Debug, Error)]
/// Error used when there was a problem parsing the `Difficulty` section.
pub enum DifficultyParseError {
    #[error("There was a problem parsing the `{name}` property from a `str`")]
    /// A section in `Difficulty` failed to parse.
    SectionParseError {
        #[source]
        source: rust_decimal::Error,
        name: &'static str,
    },
    #[error("The key {0} doesn't exist in `Difficulty`")]
    /// Invalid key name was used.
    InvalidKey(String),
    #[error("The key {0} has no value set")]
    /// The value is missing from the `key: value` set.
    MissingValue(String),
}
