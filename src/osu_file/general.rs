use std::{fmt::Display, str::FromStr};

use super::{Decimal, Integer, DELIMITER};

/// A struct representing the general section of the .osu file
pub struct General {
    /// Location of the audio file relative to the current folder
    audio_filename: String,
    /// Milliseconds of silence before the audio starts playing
    audio_lead_in: Integer,
    /// Deprecated
    audio_hash: String,
    /// Time in milliseconds when the audio preview should start
    /// - Defaults to `-1`
    preview_time: Integer,
    /// Speed of the countdown before the first hit object
    /// - Defaults to `Normal`
    countdown: CountdownSpeed,
    /// Sample set that will be used if timing points do not override it
    /// - Defaults to `Normal`
    sample_set: SampleSet,
    /// Multiplier for the threshold in time where hit objects placed close together stack
    /// - Defaults to `0.7`
    stack_leniency: Decimal,
    /// Game mode
    /// - Defaults to `osu`
    mode: GameMode,
    /// Whether or not breaks have a letterboxing effect
    /// - Defaults to `false`
    letterbox_in_breaks: bool,
    /// Deprecated
    /// - Defaults to `true`
    story_fire_in_front: bool,
    /// Whether or not the storyboard can use the user's skin images
    /// - Defaults to `false`
    use_skin_sprites: bool,
    /// Deprecated
    /// - Defaults to `false`
    always_show_playfield: bool,
    /// Draw order of hit circle overlays compared to hit numbers
    /// - Defaults to `NoChange`
    overlay_position: OverlayPosition,
    /// Preferred skin to use during gameplay
    skin_preference: String,
    /// Whether or not a warning about flashing colours should be shown at the beginning of the map
    /// - Defaults to `false`
    epilepsy_warning: bool,
    /// Time in beats that the countdown starts before the first hit object
    /// - Defaults to `false`
    countdown_offset: Integer,
    /// Whether or not the "N+1" style key layout is used for osu!mania
    /// - Defaults to `false`
    special_style: bool,
    /// Whether or not the storyboard allows widescreen viewing
    /// - Defaults to `false`
    widescreen_storyboard: bool,
    /// Whether or not sound samples will change rate when playing with speed-changing mods
    /// - Defaults to `false`
    samples_match_playback_rate: bool,
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
            stack_leniency: 0.7,
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

        for line in s.lines() {
            match line.split_once(DELIMITER) {
                Some((key, mut value)) => {
                    value = value.trim();

                    match key.trim().to_lowercase().as_str() {
                        "audiofilename" => general.audio_filename = value.to_owned(),
                        "audioleadin" => match value.parse::<Integer>() {
                            Ok(value) => general.audio_lead_in = value,
                            Err(_) => return Err(GeneralParseError),
                        },
                        "audiohash" => general.audio_hash = value.to_owned(),
                        "previewtime" => match value.parse::<Integer>() {
                            Ok(value) => general.preview_time = value,
                            Err(_) => return Err(GeneralParseError),
                        },
                        "countdown" => {
                            general.countdown = match value.parse::<Integer>() {
                                Ok(value) => match value.try_into() {
                                    Ok(value) => value,
                                    Err(_) => return Err(GeneralParseError),
                                },
                                Err(_) => return Err(GeneralParseError),
                            }
                        }
                        "sampleset" => match SampleSet::from_str(value) {
                            Ok(sample_set) => general.sample_set = sample_set,
                            Err(_) => return Err(GeneralParseError),
                        },
                        "stackleniency" => {
                            general.stack_leniency = match value.parse::<Decimal>() {
                                Ok(value) => value,
                                Err(_) => return Err(GeneralParseError),
                            }
                        }
                        "mode" => {
                            general.mode = match value.parse::<Integer>() {
                                Ok(value) => match value.try_into() {
                                    Ok(value) => value,
                                    Err(_) => return Err(GeneralParseError),
                                },
                                Err(_) => return Err(GeneralParseError),
                            }
                        }
                        "letterboxinbreaks" => {
                            general.letterbox_in_breaks = match value.parse() {
                                Ok(value) => value,
                                Err(_) => return Err(GeneralParseError),
                            }
                        }
                        "storyfireinfront" => {
                            general.story_fire_in_front = match value.parse() {
                                Ok(value) => value,
                                Err(_) => return Err(GeneralParseError),
                            }
                        }
                        "useskinsprites" => {
                            general.use_skin_sprites = match value.parse() {
                                Ok(value) => value,
                                Err(_) => return Err(GeneralParseError),
                            }
                        }
                        "alwaysshowplayfield" => {
                            general.always_show_playfield = match value.parse() {
                                Ok(value) => value,
                                Err(_) => return Err(GeneralParseError),
                            }
                        }
                        "overlayposition" => {
                            general.overlay_position = match OverlayPosition::from_str(value) {
                                Ok(value) => value,
                                Err(_) => return Err(GeneralParseError),
                            }
                        }
                        "skinpreference" => general.skin_preference = value.to_owned(),
                        "epilepsywarning" => {
                            general.epilepsy_warning = match value.parse() {
                                Ok(value) => value,
                                Err(_) => return Err(GeneralParseError),
                            }
                        }
                        "countdownoffset" => {
                            general.countdown_offset = match value.parse() {
                                Ok(value) => value,
                                Err(_) => return Err(GeneralParseError),
                            }
                        }
                        "specialstyle" => {
                            general.special_style = match value.parse() {
                                Ok(value) => value,
                                Err(_) => return Err(GeneralParseError),
                            }
                        }
                        "widescreenstoryboard" => {
                            general.widescreen_storyboard = match value.parse() {
                                Ok(value) => value,
                                Err(_) => return Err(GeneralParseError),
                            }
                        }
                        "samplesmatchplaybackrate" => {
                            general.samples_match_playback_rate = match value.parse() {
                                Ok(value) => value,
                                Err(_) => return Err(GeneralParseError),
                            }
                        }
                        _ => return Err(GeneralParseError),
                    }
                }
                None => return Err(GeneralParseError),
            }
        }

        Ok(Self::default())
    }
}

// TODO more specific info
pub struct GeneralParseError;

impl Display for GeneralParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "There was a problem parsing the General section")
    }
}

/// Speed of the countdown before the first hit
pub enum CountdownSpeed {
    NoCountdown,
    Normal,
    Half,
    Double,
}

impl TryFrom<i32> for CountdownSpeed {
    type Error = CountdownSpeedParseError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CountdownSpeed::NoCountdown),
            1 => Ok(CountdownSpeed::Normal),
            2 => Ok(CountdownSpeed::Half),
            3 => Ok(CountdownSpeed::Double),
            _ => Err(CountdownSpeedParseError),
        }
    }
}

pub struct CountdownSpeedParseError;

impl Display for CountdownSpeedParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error trying to parse `value` as CountdownSpeed")
    }
}

impl Default for CountdownSpeed {
    fn default() -> Self {
        Self::Normal
    }
}

/// Sample set that will be used if timing points do not override it
pub enum SampleSet {
    Normal,
    Soft,
    Drum,
}

impl Default for SampleSet {
    fn default() -> Self {
        SampleSet::Normal
    }
}

impl FromStr for SampleSet {
    type Err = SampleSetParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "normal" => Ok(SampleSet::Normal),
            "soft" => Ok(SampleSet::Soft),
            "drum" => Ok(SampleSet::Drum),
            _ => Err(SampleSetParseError),
        }
    }
}

pub struct SampleSetParseError;

impl Display for SampleSetParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error trying to parse `value` as SampleSet")
    }
}

/// Game mode of the .osu file
pub enum GameMode {
    Osu,
    Taiko,
    Catch,
    Mania,
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
            _ => Err(GameModeParseError),
        }
    }
}

pub struct GameModeParseError;

impl Display for GameModeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error trying to parse `value` as GameMode")
    }
}

/// Draw order of hit circle overlays compared to hit numbers
pub enum OverlayPosition {
    /// Use skin setting
    NoChange,
    /// Draw overlays under numbers
    Below,
    /// Draw overlays on top of numbers
    Above,
}

impl Default for OverlayPosition {
    fn default() -> Self {
        Self::NoChange
    }
}

impl FromStr for OverlayPosition {
    type Err = OverlayPositionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "nochange" => Ok(OverlayPosition::NoChange),
            "below" => Ok(OverlayPosition::Below),
            "above" => Ok(OverlayPosition::Above),
            _ => Err(OverlayPositionParseError),
        }
    }
}

pub struct OverlayPositionParseError;

impl Display for OverlayPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error trying to parse `value` as OverlayPosition")
    }
}
