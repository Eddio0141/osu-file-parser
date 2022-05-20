use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub struct ParseError(#[from] ColourParseError);

/// Error used when there was a problem parsing a `str` as a `Colour`.
#[derive(Debug, Error)]
pub enum ColourParseError {
    /// The colour key was invalid.
    #[error("Unknown colour option")]
    UnknownColourOption,
    /// The colour value was invalid.
    #[error("Invalid colour value")]
    InvalidColourValue,
    /// Invalid additive combo count.
    #[error("Invalid additive combo count")]
    InvalidComboCount,
    /// Missing green value.
    #[error("The rgb section is missing a green value")]
    MissingGreenValue,
    /// Missing blue value.
    #[error("The rgb section is missing a blue value")]
    MissingBlueValue,
}
