use thiserror::Error;

use super::{
    storyboard::error::*, BackgroundParseError, BreakParseError, ColourTransformationParseError,
    VideoParseError,
};

/// Errors used when there was a problem parsing an [`Event`][super::Event] from a `str`.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ParseError {
    /// When the line isn't in a `key: value` format.
    #[error("Invalid colon set, expected format of `key: value`")]
    InvalidColonSet,
    /// Invalid key name was used.
    #[error("The key doesn't exist in `General`")]
    InvalidKey,
    /// A `storyboard` `command` was used without defined sprite or animation sprite.
    #[error("A storyboard command was used without defined sprite or animation sprite")]
    StoryboardCmdWithNoSprite,
    #[error(transparent)]
    BackgroundParseError(#[from] BackgroundParseError),
    #[error(transparent)]
    VideoParseError(#[from] VideoParseError),
    #[error(transparent)]
    BreakParseError(#[from] BreakParseError),
    #[error(transparent)]
    ColourTransformationParseError(#[from] ColourTransformationParseError),
    #[error(transparent)]
    CommandPushError(#[from] CommandPushError),
    #[error(transparent)]
    CommandParseError(#[from] CommandParseError),
    /// There was a problem parsing some `storyboard` element.
    #[error(transparent)]
    StoryboardObjectParseError(#[from] ObjectParseError),
}
