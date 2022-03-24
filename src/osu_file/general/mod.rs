pub mod error;

use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use strum_macros::{Display, EnumString, FromRepr};

use crate::osu_file::{helper::display_colon_fields, parsers::get_colon_field_value_lines};

use self::error::*;

use super::{helper::parse_zero_one_bool, Integer};

/// A struct representing the general section of the .osu file.
#[derive(PartialEq, Debug, Clone, Eq, Hash)]
#[non_exhaustive]
pub struct General {
    /// Location of the audio file relative to the current folder.
    pub audio_filename: Option<String>,
    /// Milliseconds of silence before the audio starts playing.
    pub audio_lead_in: Option<Integer>,
    /// Deprecated.
    pub audio_hash: Option<String>,
    /// Time in milliseconds when the audio preview should start.
    /// - Defaults to `-1`.
    pub preview_time: Option<Integer>,
    /// Speed of the countdown before the first hit object.
    /// - Defaults to `Normal`.
    pub countdown: Option<CountdownSpeed>,
    /// Sample set that will be used if timing points do not override it.
    /// - Defaults to `Normal`.
    pub sample_set: Option<SampleSet>,
    /// Multiplier for the threshold in time where hit objects placed close together stack.
    /// - Defaults to `0.7`.
    pub stack_leniency: Option<Decimal>,
    /// Game mode.
    /// - Defaults to `osu`.
    pub mode: Option<GameMode>,
    /// Whether or not breaks have a letterboxing effect.
    /// - Defaults to `false`.
    pub letterbox_in_breaks: Option<bool>,
    /// Deprecated.
    /// - Defaults to `true`.
    pub story_fire_in_front: Option<bool>,
    /// Whether or not the storyboard can use the user's skin images.
    /// - Defaults to `false`.
    pub use_skin_sprites: Option<bool>,
    /// Deprecated.
    /// - Defaults to `false`.
    pub always_show_playfield: Option<bool>,
    /// Draw order of hit circle overlays compared to hit numbers.
    /// - Defaults to `NoChange`.
    pub overlay_position: Option<OverlayPosition>,
    /// Preferred skin to use during gameplay.
    pub skin_preference: Option<String>,
    /// Whether or not a warning about flashing colours should be shown at the beginning of the map.
    /// - Defaults to `false`.
    pub epilepsy_warning: Option<bool>,
    /// Time in beats that the countdown starts before the first hit object.
    /// - Defaults to `false`.
    pub countdown_offset: Option<Integer>,
    /// Whether or not the "N+1" style key layout is used for osu!mania.
    /// - Defaults to `false`.
    pub special_style: Option<bool>,
    /// Whether or not the storyboard allows widescreen viewing.
    /// - Defaults to `false`.
    pub widescreen_storyboard: Option<bool>,
    /// Whether or not sound samples will change rate when playing with speed-changing mods.
    /// - Defaults to `false`.
    pub samples_match_playback_rate: Option<bool>,
}

impl General {
    /// Creates an empty `General` instance, with all fields being `None`.
    pub fn empty() -> Self {
        Self {
            audio_filename: None,
            audio_lead_in: None,
            audio_hash: None,
            preview_time: None,
            countdown: None,
            sample_set: None,
            stack_leniency: None,
            mode: None,
            letterbox_in_breaks: None,
            story_fire_in_front: None,
            use_skin_sprites: None,
            always_show_playfield: None,
            overlay_position: None,
            skin_preference: None,
            epilepsy_warning: None,
            countdown_offset: None,
            special_style: None,
            widescreen_storyboard: None,
            samples_match_playback_rate: None,
        }
    }
}

impl Default for General {
    fn default() -> Self {
        Self {
            audio_filename: Some(Default::default()),
            audio_lead_in: Some(Default::default()),
            audio_hash: Some(Default::default()),
            preview_time: Some(-1),
            countdown: Some(Default::default()),
            sample_set: Some(Default::default()),
            stack_leniency: Some(dec!(0.7)),
            mode: Some(Default::default()),
            letterbox_in_breaks: Some(Default::default()),
            story_fire_in_front: Some(true),
            use_skin_sprites: Some(Default::default()),
            always_show_playfield: Some(Default::default()),
            overlay_position: Some(Default::default()),
            skin_preference: Some(Default::default()),
            epilepsy_warning: Some(Default::default()),
            countdown_offset: Some(Default::default()),
            special_style: Some(Default::default()),
            widescreen_storyboard: Some(Default::default()),
            samples_match_playback_rate: Some(Default::default()),
        }
    }
}

impl FromStr for General {
    type Err = GeneralParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut general = General::empty();

        let (s, fields) = get_colon_field_value_lines(s).unwrap();

        if !s.trim().is_empty() {
            return Err(GeneralParseError::InvalidColonSet(
                s.lines().next().unwrap_or_default().to_string(),
            ));
        }

        for (name, value) in fields {
            match name {
                "AudioFilename" => general.audio_filename = Some(value.to_owned()),
                "AudioLeadIn" => {
                    general.audio_lead_in =
                        Some(value.parse().map_err(|err| FieldError::AudioLeadIn {
                            source: err,
                            value: value.to_string(),
                        })?)
                }
                "AudioHash" => general.audio_hash = Some(value.to_owned()),
                "PreviewTime" => {
                    general.preview_time =
                        Some(value.parse().map_err(|err| FieldError::PreviewTime {
                            source: err,
                            value: value.to_string(),
                        })?)
                }
                "Countdown" => {
                    general.countdown =
                        Some(value.parse().map_err(|err| FieldError::Countdown {
                            source: err,
                            value: value.to_string(),
                        })?)
                }
                "SampleSet" => {
                    general.sample_set =
                        Some(value.parse().map_err(|err| FieldError::SampleSet {
                            source: err,
                            value: value.to_string(),
                        })?)
                }
                "StackLeniency" => {
                    general.stack_leniency =
                        Some(value.parse().map_err(|err| FieldError::StackLeniency {
                            source: err,
                            value: value.to_string(),
                        })?)
                }
                "Mode" => {
                    general.mode = Some(value.parse().map_err(|err| FieldError::Mode {
                        source: err,
                        value: value.to_string(),
                    })?)
                }
                "LetterboxInBreaks" => {
                    general.letterbox_in_breaks =
                        Some(parse_zero_one_bool(value).map_err(|err| {
                            FieldError::LetterboxInBreaks {
                                source: err,
                                value: value.to_string(),
                            }
                        })?)
                }
                "StoryFireInFront" => {
                    general.story_fire_in_front =
                        Some(parse_zero_one_bool(value).map_err(|err| {
                            FieldError::StoryFireInFront {
                                source: err,
                                value: value.to_string(),
                            }
                        })?)
                }
                "UseSkinSprites" => {
                    general.use_skin_sprites = Some(parse_zero_one_bool(value).map_err(|err| {
                        FieldError::UseSkinSprites {
                            source: err,
                            value: value.to_string(),
                        }
                    })?)
                }
                "AlwaysShowPlayfield" => {
                    general.always_show_playfield =
                        Some(parse_zero_one_bool(value).map_err(|err| {
                            FieldError::AlwaysShowPlayfield {
                                source: err,
                                value: value.to_string(),
                            }
                        })?)
                }
                "OverlayPosition" => {
                    general.overlay_position =
                        Some(value.parse().map_err(|err| FieldError::OverlayPosition {
                            source: err,
                            value: value.to_string(),
                        })?)
                }
                "SkinPreference" => general.skin_preference = Some(value.to_owned()),
                "EpilepsyWarning" => {
                    general.epilepsy_warning = Some(parse_zero_one_bool(value).map_err(|err| {
                        FieldError::EpilepsyWarning {
                            source: err,
                            value: value.to_string(),
                        }
                    })?)
                }
                "CountdownOffset" => {
                    general.countdown_offset =
                        Some(value.parse().map_err(|err| FieldError::CountdownOffset {
                            source: err,
                            value: value.to_string(),
                        })?)
                }
                "SpecialStyle" => {
                    general.special_style = Some(parse_zero_one_bool(value).map_err(|err| {
                        FieldError::SpecialStyle {
                            source: err,
                            value: value.to_string(),
                        }
                    })?)
                }
                "WidescreenStoryboard" => {
                    general.widescreen_storyboard =
                        Some(parse_zero_one_bool(value).map_err(|err| {
                            FieldError::WidescreenStoryboard {
                                source: err,
                                value: value.to_string(),
                            }
                        })?)
                }
                "SamplesMatchPlaybackRate" => {
                    general.samples_match_playback_rate =
                        Some(parse_zero_one_bool(value).map_err(|err| {
                            FieldError::SamplesMatchPlaybackRate {
                                source: err,
                                value: value.to_string(),
                            }
                        })?)
                }
                _ => return Err(GeneralParseError::InvalidKey(name.to_string())),
            }
        }

        Ok(general)
    }
}

impl Display for General {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fields = [
            ("AudioFilename", &self.audio_filename),
            ("AudioLeadIn", &self.audio_lead_in.map(|v| v.to_string())),
            ("AudioHash", &self.audio_hash),
            ("PreviewTime", &self.preview_time.map(|v| v.to_string())),
            (
                "Countdown",
                &self.countdown.map(|v| (v as Integer).to_string()),
            ),
            ("SampleSet", &self.sample_set.map(|v| v.to_string())),
            ("StackLeniency", &self.stack_leniency.map(|v| v.to_string())),
            ("Mode", &self.mode.map(|v| (v as Integer).to_string())),
            (
                "LetterboxInBreaks",
                &self.letterbox_in_breaks.map(|v| (v as Integer).to_string()),
            ),
            (
                "StoryFireInFront",
                &self.story_fire_in_front.map(|v| (v as Integer).to_string()),
            ),
            (
                "UseSkinSprites",
                &self.use_skin_sprites.map(|v| (v as Integer).to_string()),
            ),
            (
                "AlwaysShowPlayfield",
                &self
                    .always_show_playfield
                    .map(|v| (v as Integer).to_string()),
            ),
            (
                "OverlayPosition",
                &self.overlay_position.map(|v| v.to_string()),
            ),
            (
                "SkinPreference",
                &self.skin_preference.as_ref().map(|v| v.to_string()),
            ),
            (
                "EpilepsyWarning",
                &self.epilepsy_warning.map(|v| (v as Integer).to_string()),
            ),
            (
                "CountdownOffset",
                &self.countdown_offset.map(|v| v.to_string()),
            ),
            (
                "SpecialStyle",
                &self.special_style.map(|v| (v as Integer).to_string()),
            ),
            (
                "WidescreenStoryboard",
                &self
                    .widescreen_storyboard
                    .map(|v| (v as Integer).to_string()),
            ),
            (
                "SamplesMatchPlaybackRate",
                &self
                    .samples_match_playback_rate
                    .map(|v| (v as Integer).to_string()),
            ),
        ];

        display_colon_fields(f, &fields, true)
    }
}

/// Speed of the countdown before the first hitobject.
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash, FromRepr)]
#[non_exhaustive]
pub enum CountdownSpeed {
    /// No countdown.
    NoCountdown,
    /// Normal speed.
    Normal,
    /// Half speed.
    Half,
    /// Double speed.
    Double,
}

impl From<CountdownSpeed> for Integer {
    fn from(c: CountdownSpeed) -> Self {
        c as Integer
    }
}

impl FromStr for CountdownSpeed {
    type Err = CountdownSpeedParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.parse()?;
        CountdownSpeed::from_repr(s).ok_or(CountdownSpeedParseError::UnknownType(s))
    }
}

impl Default for CountdownSpeed {
    fn default() -> Self {
        Self::Normal
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
}

impl Default for SampleSet {
    fn default() -> Self {
        SampleSet::Normal
    }
}

/// Game mode of the .osu file
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash, FromRepr)]
#[non_exhaustive]
pub enum GameMode {
    /// Osu! gamemode.
    Osu,
    /// Osu!Taiko gamemode.
    Taiko,
    /// Osu!Catch gamemode.
    Catch,
    /// Osu!Mania gamemode.
    Mania,
}

impl Default for GameMode {
    fn default() -> Self {
        Self::Osu
    }
}

impl FromStr for GameMode {
    type Err = GameModeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.parse()?;
        GameMode::from_repr(s).ok_or(GameModeParseError::UnknownType(s))
    }
}

impl From<GameMode> for Integer {
    fn from(value: GameMode) -> Self {
        value as Integer
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
