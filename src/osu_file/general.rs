use std::{
    error::Error,
    fmt::{Debug, Display},
    num::ParseIntError,
    str::FromStr,
};

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use strum_macros::FromRepr;
use thiserror::Error;

use super::{helper::parse_zero_one_bool, Integer, SECTION_DELIMITER};

/// A struct representing the general section of the .osu file.
#[derive(PartialEq, Debug, Clone, Eq, Hash)]
pub struct General {
    /// Location of the audio file relative to the current folder.
    pub audio_filename: String,
    /// Milliseconds of silence before the audio starts playing.
    pub audio_lead_in: Integer,
    /// Deprecated.
    pub audio_hash: String,
    /// Time in milliseconds when the audio preview should start.
    /// - Defaults to `-1`.
    pub preview_time: Integer,
    /// Speed of the countdown before the first hit object.
    /// - Defaults to `Normal`.
    pub countdown: CountdownSpeed,
    /// Sample set that will be used if timing points do not override it.
    /// - Defaults to `Normal`.
    pub sample_set: SampleSet,
    /// Multiplier for the threshold in time where hit objects placed close together stack.
    /// - Defaults to `0.7`.
    pub stack_leniency: Decimal,
    /// Game mode.
    /// - Defaults to `osu`.
    pub mode: GameMode,
    /// Whether or not breaks have a letterboxing effect.
    /// - Defaults to `false`.
    pub letterbox_in_breaks: bool,
    /// Deprecated.
    /// - Defaults to `true`.
    pub story_fire_in_front: bool,
    /// Whether or not the storyboard can use the user's skin images.
    /// - Defaults to `false`.
    pub use_skin_sprites: bool,
    /// Deprecated.
    /// - Defaults to `false`.
    pub always_show_playfield: bool,
    /// Draw order of hit circle overlays compared to hit numbers.
    /// - Defaults to `NoChange`.
    pub overlay_position: OverlayPosition,
    /// Preferred skin to use during gameplay.
    pub skin_preference: String,
    /// Whether or not a warning about flashing colours should be shown at the beginning of the map.
    /// - Defaults to `false`.
    pub epilepsy_warning: bool,
    /// Time in beats that the countdown starts before the first hit object.
    /// - Defaults to `false`.
    pub countdown_offset: Integer,
    /// Whether or not the "N+1" style key layout is used for osu!mania.
    /// - Defaults to `false`.
    pub special_style: bool,
    /// Whether or not the storyboard allows widescreen viewing.
    /// - Defaults to `false`.
    pub widescreen_storyboard: bool,
    /// Whether or not sound samples will change rate when playing with speed-changing mods.
    /// - Defaults to `false`.
    pub samples_match_playback_rate: bool,
}

impl Default for General {
    fn default() -> Self {
        Self {
            audio_filename: Default::default(),
            audio_lead_in: Default::default(),
            audio_hash: Default::default(),
            preview_time: -1,
            countdown: Default::default(),
            sample_set: Default::default(),
            stack_leniency: dec!(0.7),
            mode: Default::default(),
            letterbox_in_breaks: Default::default(),
            story_fire_in_front: true,
            use_skin_sprites: Default::default(),
            always_show_playfield: Default::default(),
            overlay_position: Default::default(),
            skin_preference: Default::default(),
            epilepsy_warning: Default::default(),
            countdown_offset: Default::default(),
            special_style: Default::default(),
            widescreen_storyboard: Default::default(),
            samples_match_playback_rate: Default::default(),
        }
    }
}

impl FromStr for General {
    type Err = GeneralParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut general = Self::default();

        let s = s.trim();

        for line in s.lines() {
            match line.split_once(SECTION_DELIMITER) {
                Some((key, value)) => {
                    let value = value.trim();

                    match key.trim() {
                        "AudioFilename" => {
                            general.audio_filename = value.to_owned();
                        }
                        "AudioLeadIn" => {
                            general.audio_lead_in = value.parse().map_err(|err| {
                                GeneralParseError::SectionParseError {
                                    source: Box::new(err),
                                    name: "AudioLeadIn",
                                }
                            })?
                        }
                        "AudioHash" => general.audio_hash = value.to_owned(),
                        "PreviewTime" => {
                            general.preview_time = value.parse().map_err(|err| {
                                GeneralParseError::SectionParseError {
                                    source: Box::new(err),
                                    name: "PreviewTime",
                                }
                            })?
                        }
                        "Countdown" => {
                            general.countdown = value.parse().map_err(|err| {
                                GeneralParseError::SectionParseError {
                                    source: Box::new(err),
                                    name: "Countdown",
                                }
                            })?;
                        }
                        "SampleSet" => {
                            general.sample_set = value.parse().map_err(|err| {
                                GeneralParseError::SectionParseError {
                                    source: Box::new(err),
                                    name: "SampleSet",
                                }
                            })?
                        }
                        "StackLeniency" => {
                            general.stack_leniency = value.parse().map_err(|err| {
                                GeneralParseError::SectionParseError {
                                    source: Box::new(err),
                                    name: "StackLeniency",
                                }
                            })?
                        }
                        "Mode" => {
                            general.mode = value.parse().map_err(|err| {
                                GeneralParseError::SectionParseError {
                                    source: Box::new(err),
                                    name: "Mode",
                                }
                            })?
                        }
                        "LetterboxInBreaks" => {
                            general.letterbox_in_breaks =
                                parse_zero_one_bool(value).map_err(|err| {
                                    GeneralParseError::SectionParseError {
                                        source: Box::new(err),
                                        name: "LetterboxInBreaks",
                                    }
                                })?
                        }
                        "StoryFireInFront" => {
                            general.story_fire_in_front =
                                parse_zero_one_bool(value).map_err(|err| {
                                    GeneralParseError::SectionParseError {
                                        source: Box::new(err),
                                        name: "StoryFireInFront",
                                    }
                                })?
                        }
                        "UseSkinSprites" => {
                            general.use_skin_sprites =
                                parse_zero_one_bool(value).map_err(|err| {
                                    GeneralParseError::SectionParseError {
                                        source: Box::new(err),
                                        name: "UseSkinSprites",
                                    }
                                })?
                        }
                        "AlwaysShowPlayfield" => {
                            general.always_show_playfield =
                                parse_zero_one_bool(value).map_err(|err| {
                                    GeneralParseError::SectionParseError {
                                        source: Box::new(err),
                                        name: "AlwaysShowPlayfield",
                                    }
                                })?
                        }
                        "OverlayPosition" => {
                            general.overlay_position = value.parse().map_err(|err| {
                                GeneralParseError::SectionParseError {
                                    source: Box::new(err),
                                    name: "OverlayPosition",
                                }
                            })?
                        }
                        "SkinPreference" => general.skin_preference = value.to_owned(),
                        "EpilepsyWarning" => {
                            general.epilepsy_warning =
                                parse_zero_one_bool(value).map_err(|err| {
                                    GeneralParseError::SectionParseError {
                                        source: Box::new(err),
                                        name: "EpilepsyWarning",
                                    }
                                })?
                        }
                        "CountdownOffset" => {
                            general.countdown_offset = value.parse().map_err(|err| {
                                GeneralParseError::SectionParseError {
                                    source: Box::new(err),
                                    name: "CountdownOffset",
                                }
                            })?
                        }
                        "SpecialStyle" => {
                            general.special_style = parse_zero_one_bool(value).map_err(|err| {
                                GeneralParseError::SectionParseError {
                                    source: Box::new(err),
                                    name: "SpecialStyle",
                                }
                            })?
                        }
                        "WidescreenStoryboard" => {
                            general.widescreen_storyboard =
                                parse_zero_one_bool(value).map_err(|err| {
                                    GeneralParseError::SectionParseError {
                                        source: Box::new(err),
                                        name: "WidescreenStoryboard",
                                    }
                                })?
                        }
                        "SamplesMatchPlaybackRate" => {
                            general.samples_match_playback_rate = parse_zero_one_bool(value)
                                .map_err(|err| GeneralParseError::SectionParseError {
                                    source: Box::new(err),
                                    name: "SamplesMatchPlaybackRate",
                                })?
                        }
                        _ => return Err(GeneralParseError::InvalidKey(key.to_string())),
                    }
                }
                None => return Err(GeneralParseError::MissingValue(line.to_owned())),
            }
        }

        Ok(general)
    }
}

impl Display for General {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut key_value = Vec::new();

        key_value.push(format!("AudioFilename: {}", self.audio_filename));
        key_value.push(format!("AudioLeadIn: {}", self.audio_lead_in));
        key_value.push(format!("AudioHash: {}", self.audio_hash));
        key_value.push(format!("PreviewTime: {}", self.preview_time));
        key_value.push(format!("Countdown: {}", self.countdown as Integer));
        key_value.push(format!("SampleSet: {}", self.sample_set));
        key_value.push(format!("StackLeniency: {}", self.stack_leniency));
        key_value.push(format!("Mode: {}", self.mode));
        key_value.push(format!(
            "LetterboxInBreaks: {}",
            self.letterbox_in_breaks as Integer
        ));
        key_value.push(format!(
            "StoryFireInFront: {}",
            self.story_fire_in_front as Integer
        ));
        key_value.push(format!(
            "UseSkinSprites: {}",
            self.use_skin_sprites as Integer
        ));
        key_value.push(format!(
            "AlwaysShowPlayfield: {}",
            self.always_show_playfield as Integer
        ));
        key_value.push(format!("OverlayPosition: {}", self.overlay_position));
        key_value.push(format!("SkinPreference: {}", self.skin_preference));
        key_value.push(format!(
            "EpilepsyWarning: {}",
            self.epilepsy_warning as Integer
        ));
        key_value.push(format!("CountdownOffset: {}", self.countdown_offset));
        key_value.push(format!("SpecialStyle: {}", self.special_style as Integer));
        key_value.push(format!(
            "WidescreenStoryboard: {}",
            self.widescreen_storyboard as Integer
        ));
        key_value.push(format!(
            "SamplesMatchPlaybackRate: {}",
            self.samples_match_playback_rate as Integer
        ));

        write!(f, "{}", key_value.join("\r\n"))
    }
}

#[derive(Debug, Error)]
/// Error used when there was a problem parsing the `General` section.
pub enum GeneralParseError {
    #[error("There was a problem parsing the `{name}` property from a `str`")]
    /// A section in `General` failed to parse.
    SectionParseError {
        #[source]
        source: Box<dyn Error>,
        name: &'static str,
    },
    #[error("The key {0} doesn't exist in `General`")]
    /// Invalid key name was used.
    InvalidKey(String),
    #[error("The key {0} has no value set")]
    /// The value is missing from the `key: value` set.
    MissingValue(String),
}

/// Speed of the countdown before the first hitobject.
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash, FromRepr)]
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

impl FromStr for CountdownSpeed {
    type Err = CountdownSpeedParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.parse()?;
        CountdownSpeed::from_repr(s).ok_or(CountdownSpeedParseError::UnknownType(s))
    }
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

impl Default for CountdownSpeed {
    fn default() -> Self {
        Self::Normal
    }
}

/// Sample set that will be used if timing points do not override it
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum SampleSet {
    /// The `Normal` sample set.
    Normal,
    /// The `Soft` sample set.
    Soft,
    /// The `Drum` sample set.
    Drum,
}

impl Display for SampleSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            SampleSet::Normal => "Normal",
            SampleSet::Soft => "Soft",
            SampleSet::Drum => "Drum",
        };

        write!(f, "{value}")
    }
}

impl Default for SampleSet {
    fn default() -> Self {
        SampleSet::Normal
    }
}

impl FromStr for SampleSet {
    type Err = SampleSetParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Normal" => Ok(SampleSet::Normal),
            "Soft" => Ok(SampleSet::Soft),
            "Drum" => Ok(SampleSet::Drum),
            _ => Err(SampleSetParseError(s.to_string())),
        }
    }
}

/// Error used when there's an error parsing the string as enum
#[derive(Debug, Error)]
#[error("Expected SampleSet `Normal`, `Soft`, or `Drum, got {0}")]
pub struct SampleSetParseError(String);

/// Game mode of the .osu file
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
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

impl Display for GameMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            GameMode::Osu => 0,
            GameMode::Taiko => 1,
            GameMode::Catch => 2,
            GameMode::Mania => 3,
        };

        write!(f, "{value}")
    }
}

impl Default for GameMode {
    fn default() -> Self {
        Self::Osu
    }
}

impl TryFrom<i32> for GameMode {
    type Error = GameModeParseError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GameMode::Osu),
            1 => Ok(GameMode::Taiko),
            2 => Ok(GameMode::Catch),
            3 => Ok(GameMode::Mania),
            _ => Err(GameModeParseError::UnknownType(value)),
        }
    }
}

impl FromStr for GameMode {
    type Err = GameModeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Integer = s.parse()?;
        s.try_into()
    }
}

/// Error used when there's an error parsing the string as enum.
#[derive(Debug, Error)]
pub enum GameModeParseError {
    /// Error when the `GameMode` isn't the value between 0 ~ 3.
    #[error("Expected `GameMode` to be value from 0 ~ 3, got value {0}")]
    UnknownType(Integer),
    /// Error trying to parse the `str` into an `Integer`.
    #[error("There was a problem parsing the `str` as an `Integer`")]
    ParseError(#[from] ParseIntError),
}

/// Draw order of hit circle overlays compared to hit numbers
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum OverlayPosition {
    /// Use skin setting
    NoChange,
    /// Draw overlays under numbers
    Below,
    /// Draw overlays on top of numbers
    Above,
}

impl Display for OverlayPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            OverlayPosition::NoChange => "NoChange",
            OverlayPosition::Below => "Below",
            OverlayPosition::Above => "Above",
        };

        write!(f, "{value}")
    }
}

impl Default for OverlayPosition {
    fn default() -> Self {
        Self::NoChange
    }
}

impl FromStr for OverlayPosition {
    type Err = OverlayPositionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NoChange" => Ok(OverlayPosition::NoChange),
            "Below" => Ok(OverlayPosition::Below),
            "Above" => Ok(OverlayPosition::Above),
            _ => Err(OverlayPositionParseError(s.to_string())),
        }
    }
}

/// Error used when there's an error parsing the string as enum.
#[derive(Debug, Error)]
#[error("Expected `NoChange` or `Below` or `Above` as `OverlayPosition`, got {0}")]
pub struct OverlayPositionParseError(String);
