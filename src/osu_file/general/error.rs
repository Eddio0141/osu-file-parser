use std::num::ParseIntError;

use strum::ParseError;
use thiserror::Error;

use crate::osu_file::helper::ParseZeroOneBoolError;

#[derive(Debug, Error)]
/// Error used when there was a problem parsing the `General` section.
pub enum GeneralParseError {
    /// A Field in `General` failed to parse.
    #[error(transparent)]
    FieldParseError {
        #[from]
        field: FieldError,
    },
    /// When the line isn't in a `key: value` format.
    #[error("Invalid colon set: {0}, expected format of `key: value`")]
    InvalidColonSet(String),
    /// Invalid key name was used.
    #[error("The key {0} doesn't exist in `General`")]
    InvalidKey(String),
}

/// All field errors when parsing the value of a key.
#[derive(Debug, Error)]
pub enum FieldError {
    #[error("The `AudioLeadIn` field failed to parse from {value}")]
    AudioLeadIn {
        #[source]
        source: ParseIntError,
        value: String,
    },
    #[error("The `PreviewTime` field failed to parse from {value}")]
    PreviewTime {
        #[source]
        source: ParseIntError,
        value: String,
    },
    #[error("The `Countdown` field failed to parse from {value}")]
    Countdown {
        #[source]
        source: CountdownSpeedParseError,
        value: String,
    },
    #[error("The `SampleSet` field failed to parse from {value}")]
    SampleSet {
        #[source]
        source: ParseError,
        value: String,
    },
    #[error("The `StackLeniency` field failed to parse from {value}")]
    StackLeniency {
        #[source]
        source: rust_decimal::Error,
        value: String,
    },
    #[error("The `Mode` field failed to parse from {value}")]
    Mode {
        #[source]
        source: GameModeParseError,
        value: String,
    },
    #[error("The `LetterboxInBreaks` field failed to parse from {value}")]
    LetterboxInBreaks {
        #[source]
        source: ParseZeroOneBoolError,
        value: String,
    },
    #[error("The `StoryFireInFront` field failed to parse from {value}")]
    StoryFireInFront {
        #[source]
        source: ParseZeroOneBoolError,
        value: String,
    },
    #[error("The `UseSkinSprites` field failed to parse from {value}")]
    UseSkinSprites {
        #[source]
        source: ParseZeroOneBoolError,
        value: String,
    },
    #[error("The `AlwaysShowPlayfield` field failed to parse from {value}")]
    AlwaysShowPlayfield {
        #[source]
        source: ParseZeroOneBoolError,
        value: String,
    },
    #[error("The `OverlayPosition` field failed to parse from {value}")]
    OverlayPosition {
        #[source]
        source: ParseError,
        value: String,
    },
    #[error("The `EpilepsyWarning` field failed to parse from {value}")]
    EpilepsyWarning {
        #[source]
        source: ParseZeroOneBoolError,
        value: String,
    },
    #[error("The `CountdownOffset` field failed to parse from {value}")]
    CountdownOffset {
        #[source]
        source: ParseIntError,
        value: String,
    },
    #[error("The `SpecialStyle` field failed to parse from {value}")]
    SpecialStyle {
        #[source]
        source: ParseZeroOneBoolError,
        value: String,
    },
    #[error("The `WidescreenStoryboard` field failed to parse from {value}")]
    WidescreenStoryboard {
        #[source]
        source: ParseZeroOneBoolError,
        value: String,
    },
    #[error("The `SamplesMatchPlaybackRate` field failed to parse from {value}")]
    SamplesMatchPlaybackRate {
        #[source]
        source: ParseZeroOneBoolError,
        value: String,
    },
}

/// Error used when there's an error parsing the string as enum.
#[derive(Debug, Error)]
pub enum GameModeParseError {
    /// Error when the `GameMode` isn't the value between 0 ~ 3.
    #[error("Expected `GameMode` to be value from 0 ~ 3, got value {0}")]
    UnknownType(usize),
    /// Error trying to parse the `str` into an `Integer`.
    #[error("There was a problem parsing the `str` as an `Integer`")]
    ParseError(#[from] ParseIntError),
}

/// Error used when there's an error parsing the string as enum
#[derive(Debug, Error)]
pub enum CountdownSpeedParseError {
    #[error("Expected `CountdownSpeed` to be value from 0 ~ 3, got value {0}")]
    /// The integer value is an unknown `CountdownSpeed` type.
    UnknownType(usize),
    #[error("There was a problem parsing the `str` as an `Integer`")]
    /// There was a problem converting from `str` to an `Integer`.
    ParseError(#[from] ParseIntError),
}
