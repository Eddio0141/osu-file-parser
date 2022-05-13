use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub struct ColoursParseError(#[from] ColourParseError);

/// Error used when there was a problem parsing a `str` as a `Colour`.
#[derive(Debug, Error)]
pub enum ColourParseError {
    /// The colour key was invalid.
    #[error("The colour option `{0}` is unknown")]
    UnknownColourOption(String),
    /// The colour value was invalid.
    #[error("The colour value `{0}` is invalid")]
    InvalidColourValue(String),
    /// Invalid additive combo count.
    #[error("The additive combo count `{0}` is invalid")]
    InvalidComboCount(String),
    /// Missing green value.
    #[error("The rgb section is missing a green value")]
    MissingGreenValue,
    /// Missing blue value.
    #[error("The rgb section is missing a blue value")]
    MissingBlueValue,
}
