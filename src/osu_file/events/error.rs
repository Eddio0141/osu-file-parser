use thiserror::Error;

use super::{storyboard::error::*, *};

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
    ParseBackgroundError(#[from] ParseBackgroundError),
    #[error(transparent)]
    ParseVideoError(#[from] ParseVideoError),
    #[error(transparent)]
    ParseBreakError(#[from] ParseBreakError),
    #[error(transparent)]
    ParseColourTransformationError(#[from] ParseColourTransformationError),
    #[error(transparent)]
    ParseSpriteLegacyError(#[from] ParseSpriteLegacyError),
    #[error(transparent)]
    ParseAnimationLegacyError(#[from] ParseAnimationLegacyError),
    #[error(transparent)]
    ParseSampleLegacyError(#[from] ParseSampleLegacyError),
    #[error(transparent)]
    CommandPushError(#[from] CommandPushError),
    #[error(transparent)]
    ParseCommandError(#[from] ParseCommandError),
    /// There was a problem parsing some `storyboard` element.
    #[error(transparent)]
    ParseStoryboardObjectError(#[from] ParseObjectError),
    #[error(transparent)]
    ParseAudioSampleError(#[from] ParseAudioSampleError),
    /// Event doesn't exist on the version.
    #[error("Event type doesn't exist on version")]
    EventNotExistOnVersion,
    /// Unknown event type.
    #[error("Unknown event type")]
    UnknownEventType,
}
