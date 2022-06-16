pub mod error;

use std::fmt::Debug;
use std::num::ParseIntError;
use std::path::PathBuf;

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use strum_macros::{Display, EnumString, FromRepr};

use crate::helper;
use crate::helper::macros::*;
use crate::parsers::get_colon_field_value_lines;

use crate::osu_file::Integer;

pub use self::error::*;

use super::MIN_VERSION;
use super::{types::Error, VersionedDefault, VersionedFromString, VersionedToString};

versioned_field!(AudioFilename, PathBuf, no_versions, |s| { Ok(PathBuf::from(s)) } -> (), |v| { v.display().to_string() }, PathBuf::from(""));
versioned_field!(AudioLeadIn, Integer, no_versions, |s| { s.parse() } -> ParseIntError,, 0);
versioned_field!(AudioHash, String, no_versions, |s| { Ok(s.to_string()) } -> (),
    |v, version| { if version > 13 { None } else { Some(v.to_string()) } },
    |version| { if version > 13 { None } else { Some(String::new())}
});
versioned_field!(PreviewTime, Integer, no_versions, |s| { s.parse() } -> ParseIntError,, -1);
versioned_field!(StackLeniency, Decimal, no_versions, |s| { s.parse() } -> rust_decimal::Error,, dec!(0.7));
versioned_field!(LetterboxInBreaks, bool, no_versions, |s| { helper::parse_zero_one_bool(s) } -> helper::ParseZeroOneBoolError, boolean, false);
versioned_field!(StoryFireInFront, bool, no_versions, |s| { helper::parse_zero_one_bool(s) } -> helper::ParseZeroOneBoolError, boolean, true);
versioned_field!(UseSkinSprites, bool, no_versions, |s| { helper::parse_zero_one_bool(s) } -> helper::ParseZeroOneBoolError, boolean, false);
versioned_field!(AlwaysShowPlayfield, bool, no_versions, |s| { helper::parse_zero_one_bool(s) } -> helper::ParseZeroOneBoolError, boolean, false);
versioned_field!(SkinPreference, String, no_versions, |s| { Ok(s.to_string()) } -> (),, String::new());
versioned_field!(EpilepsyWarning, bool, no_versions, |s| { helper::parse_zero_one_bool(s) } -> helper::ParseZeroOneBoolError, boolean, false);
versioned_field!(CountdownOffset, Integer, no_versions, |s| { s.parse() } -> ParseIntError,, 0);
versioned_field!(SpecialStyle, bool, no_versions, |s| { helper::parse_zero_one_bool(s) } -> helper::ParseZeroOneBoolError, boolean, false);
versioned_field!(WidescreenStoryboard, bool, no_versions, |s| { helper::parse_zero_one_bool(s) } -> helper::ParseZeroOneBoolError, boolean, false);
versioned_field!(SamplesMatchPlaybackRate, bool, no_versions, |s| { helper::parse_zero_one_bool(s) } -> helper::ParseZeroOneBoolError, boolean, false);

general_section!(
    /// A struct representing the general section of an osu file.
    pub struct General {
        /// The name of the beatmap.
        pub audio_filename: AudioFilename,
        /// Milliseconds of silence before the audio starts playing.
        pub audio_lead_in: AudioLeadIn,
        /// Deprecated.
        pub audio_hash: AudioHash,
        /// Time in milliseconds when the audio preview should start.
        /// - Defaults to `-1`.
        pub preview_time: PreviewTime,
        /// Speed of the countdown before the first hit object.
        /// - Defaults to `Normal`.
        pub countdown: Countdown,
        /// Sample set that will be used if timing points do not override it.
        /// - Defaults to `Normal`.
        pub sample_set: SampleSet,
        /// Multiplier for the threshold in time where hit objects placed close together stack.
        /// - Defaults to `0.7`.
        pub stack_leniency: StackLeniency,
        /// Game mode.
        /// - Defaults to `osu`.
        pub mode: Mode,
        /// Whether or not breaks have a letterboxing effect.
        /// - Defaults to `false`.
        pub letterbox_in_breaks: LetterboxInBreaks,
        /// Deprecated.
        /// - Defaults to `true`.
        pub story_fire_in_front: StoryFireInFront,
        /// Whether or not the storyboard can use the user's skin images.
        /// - Defaults to `false`.
        pub use_skin_sprites: UseSkinSprites,
        /// Deprecated.
        /// - Defaults to `false`.
        pub always_show_playfield: AlwaysShowPlayfield,
        /// Draw order of hit circle overlays compared to hit numbers.
        /// - Defaults to `NoChange`.
        pub overlay_position: OverlayPosition,
        /// Preferred skin to use during gameplay.
        pub skin_preference: SkinPreference,
        /// Whether or not a warning about flashing colours should be shown at the beginning of the map.
        /// - Defaults to `false`.
        pub epilepsy_warning: EpilepsyWarning,
        /// Time in beats that the countdown starts before the first hit object.
        /// - Defaults to `0`.
        pub countdown_offset: CountdownOffset,
        /// Whether or not the "N+1" style key layout is used for osu!mania.
        /// - Defaults to `false`.
        pub special_style: SpecialStyle,
        /// Whether or not the storyboard allows widescreen viewing.
        /// - Defaults to `false`.
        pub widescreen_storyboard: WidescreenStoryboard,
        /// Whether or not sound samples will change rate when playing with speed-changing mods.
        /// - Defaults to `false`.
        pub samples_match_playback_rate: SamplesMatchPlaybackRate,
    },
    ParseError
);

/// Speed of the countdown before the first hitobject.
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash, FromRepr)]
#[non_exhaustive]
pub enum Countdown {
    /// No countdown.
    NoCountdown,
    /// Normal speed.
    Normal,
    /// Half speed.
    Half,
    /// Double speed.
    Double,
}

// TODO investigate versions
impl VersionedFromString for Countdown {
    type ParseError = ParseCountdownSpeedError;

    fn from_str(s: &str, version: usize) -> std::result::Result<Option<Self>, Self::ParseError> {
        match version {
            MIN_VERSION..=4 => Ok(None),
            _ => Countdown::from_repr(s.parse()?)
                .ok_or(ParseCountdownSpeedError::UnknownType)
                .map(Some),
        }
    }
}

impl VersionedToString for Countdown {
    fn to_string(&self, version: usize) -> Option<String> {
        match version {
            MIN_VERSION..=4 => None,
            _ => Some((*self as usize).to_string()),
        }
    }
}

impl VersionedDefault for Countdown {
    fn default(version: usize) -> Option<Self> {
        match version {
            MIN_VERSION..=4 => None,
            _ => Some(Countdown::Normal),
        }
    }
}

/// Sample set that will be used if timing points do not override it
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash, Display, EnumString)]
#[non_exhaustive]
pub enum SampleSet {
    /// The `Normal` sample set.
    Normal,
    /// The `Soft` sample set.
    Soft,
    /// The `Drum` sample set.
    Drum,
    /// The `None` sample set, used before version 14.
    /// - Converted to `Normal` in version 14.
    None,
}

impl Default for SampleSet {
    fn default() -> Self {
        SampleSet::Normal
    }
}

impl VersionedFromString for SampleSet {
    // TODO investigate versions
    type ParseError = strum::ParseError;

    fn from_str(s: &str, version: usize) -> std::result::Result<Option<Self>, Self::ParseError> {
        match version {
            3 => Ok(None),
            4..=13 => Ok(Some(s.parse()?)),
            _ => {
                let mut sample_set = s.parse()?;

                if let SampleSet::None = sample_set {
                    sample_set = SampleSet::Normal;
                };

                Ok(Some(sample_set))
            }
        }
    }
}

impl VersionedToString for SampleSet {
    fn to_string(&self, version: usize) -> Option<String> {
        match version {
            3 => None,
            // I dont think we have to revert to None
            _ => Some(ToString::to_string(&self)),
        }
    }
}

impl VersionedDefault for SampleSet {
    fn default(version: usize) -> Option<Self> {
        match version {
            3 => None,
            _ => Some(Self::Normal),
        }
    }
}

/// Game mode of the .osu file
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash, FromRepr)]
#[non_exhaustive]
pub enum Mode {
    /// Osu! gamemode.
    Osu,
    /// Osu!Taiko gamemode.
    Taiko,
    /// Osu!Catch gamemode.
    Catch,
    /// Osu!Mania gamemode.
    Mania,
}

impl VersionedFromString for Mode {
    type ParseError = ParseGameModeError;

    // TODO check what gamemodes exist in versions
    fn from_str(s: &str, _: usize) -> std::result::Result<Option<Self>, Self::ParseError> {
        Ok(Some(
            Mode::from_repr(s.parse()?).ok_or(ParseGameModeError::UnknownType)?,
        ))
    }
}

impl VersionedToString for Mode {
    fn to_string(&self, _: usize) -> Option<String> {
        Some((*self as usize).to_string())
    }
}

impl VersionedDefault for Mode {
    fn default(_: usize) -> Option<Self> {
        Some(Self::Osu)
    }
}

/// Draw order of hit circle overlays compared to hit numbers
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash, Display, EnumString)]
#[non_exhaustive]
pub enum OverlayPosition {
    /// Use skin setting.
    NoChange,
    /// Draw overlays under numbers.
    Below,
    /// Draw overlays on top of numbers.
    Above,
}

impl Default for OverlayPosition {
    fn default() -> Self {
        Self::NoChange
    }
}

impl VersionedFromString for OverlayPosition {
    type ParseError = strum::ParseError;

    // TODO investigate versions

    fn from_str(s: &str, version: usize) -> std::result::Result<Option<Self>, Self::ParseError> {
        match version {
            MIN_VERSION..=13 => Ok(None),
            _ => Ok(Some(s.parse()?)),
        }
    }
}

impl VersionedToString for OverlayPosition {
    fn to_string(&self, version: usize) -> Option<String> {
        match version {
            MIN_VERSION..=13 => None,
            _ => Some(ToString::to_string(&self)),
        }
    }
}

impl VersionedDefault for OverlayPosition {
    fn default(version: usize) -> Option<Self> {
        match version {
            MIN_VERSION..=13 => None,
            _ => Some(Self::NoChange),
        }
    }
}

// TODO separate types in types.rs
