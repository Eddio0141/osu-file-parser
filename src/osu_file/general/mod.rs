pub mod error;

use std::fmt::Display;
use std::path::PathBuf;
use std::{fmt::Debug, str::FromStr};

use lazy_static::lazy_static;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use strum_macros::{Display, EnumString, FromRepr};
use thiserror::Error;

use crate::parsers::get_colon_field_value_lines;
use crate::{helper, versioned_field};

use crate::osu_file::Integer;

pub use self::error::*;

use super::{types::Error, Version};
use super::{FieldVersion, LATEST_VERSION, MIN_VERSION};

versioned_field!(
    AudioFilename,
    PathBuf,
    |s: &str, _: usize| { Ok(Some(PathBuf::from(s))) },
    (),
    |_: usize| { Some(PathBuf::from("")) }
);

/// A struct representing the general section of the .osu file.
#[derive(PartialEq, Debug, Clone, Eq, Hash)]
#[non_exhaustive]
pub struct General {
    /// Location of the audio file relative to the current folder.
    pub audio_filename: Option<AudioFilename>,
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

// TODO:
// idea for General versioning:
// have a const table of [field_name, valid_versions, [methods for parsing in each valid version], [methods for display in each valid version]]
// introduce Type for the table?

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

#[derive(Debug)]
enum GeneralFieldType {
    String(String),
    Integer(Integer),
    Decimal(Decimal),
    Bool(bool),
    PathBuf(PathBuf),
    GameMode(GameMode),
    CountdownSpeed(CountdownSpeed),
}

// TODO clean all of this up
macro_rules! impl_try_from_for_enum_type {
    ($enum_type:ident, $enum_variant:ident, $type:ty) => {
        impl TryFrom<$enum_type> for $type {
            type Error = ();

            fn try_from(value: $enum_type) -> Result<Self, Self::Error> {
                if let $enum_type::$enum_variant(value) = value {
                    Ok(value)
                } else {
                    Err(())
                }
            }
        }
    };
}

impl_try_from_for_enum_type!(GeneralFieldType, Integer, Integer);
impl_try_from_for_enum_type!(GeneralFieldType, String, String);
impl_try_from_for_enum_type!(GeneralFieldType, Decimal, Decimal);
impl_try_from_for_enum_type!(GeneralFieldType, Bool, bool);
impl_try_from_for_enum_type!(GeneralFieldType, PathBuf, PathBuf);
impl_try_from_for_enum_type!(GeneralFieldType, GameMode, GameMode);
impl_try_from_for_enum_type!(GeneralFieldType, CountdownSpeed, CountdownSpeed);

#[derive(Debug, Error)]
enum GeneralFieldParseError {
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error(transparent)]
    ParseDecimalError(#[from] rust_decimal::Error),
    #[error(transparent)]
    ParseBoolError(#[from] helper::ParseZeroOneBoolError),
    #[error(transparent)]
    ParseGameModeError(#[from] ParseGameModeError),
    #[error(transparent)]
    ParseCountdownSpeedError(#[from] ParseCountdownSpeedError),
}

impl From<GeneralFieldParseError> for ParseError {
    fn from(err: GeneralFieldParseError) -> Self {
        err.into()
    }
}

impl Display for GeneralFieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GeneralFieldType::String(s) => s.to_string(),
                GeneralFieldType::Integer(i) => i.to_string(),
                GeneralFieldType::Decimal(d) => d.to_string(),
                GeneralFieldType::Bool(b) => (*b as u8).to_string(),
                GeneralFieldType::PathBuf(p) => p.display().to_string(),
                GeneralFieldType::GameMode(gm) => gm.to_string_v3().unwrap(),
                GeneralFieldType::CountdownSpeed(cs) => cs.to_string_v3().unwrap(),
            }
        )
    }
}

lazy_static! {
    static ref GENERAL_FIELD_VERSIONS: Vec<FieldVersion<GeneralFieldType, GeneralFieldParseError>> = {
        let parse_string = |s: &str| -> Result<GeneralFieldType, GeneralFieldParseError> {
            Ok(GeneralFieldType::String(s.to_string()))
        };
        let parse_integer = |s: &str| -> Result<GeneralFieldType, GeneralFieldParseError> {
            Ok(GeneralFieldType::Integer(s.parse()?))
        };
        let parse_decimal = |s: &str| -> Result<GeneralFieldType, GeneralFieldParseError> {
            Ok(GeneralFieldType::Decimal(s.parse()?))
        };
        let parse_bool = |s: &str| -> Result<GeneralFieldType, GeneralFieldParseError> {
            Ok(GeneralFieldType::Bool(helper::parse_zero_one_bool(s)?))
        };
        let parse_pathbuf = |s: &str| -> Result<GeneralFieldType, GeneralFieldParseError> {
            Ok(GeneralFieldType::PathBuf(PathBuf::from(s)))
        };
        let parse_game_mode = |s: &str| -> Result<GeneralFieldType, GeneralFieldParseError> {
            Ok(GeneralFieldType::GameMode(s.parse()?))
        };
        let parse_countdown_speed = |s: &str| -> Result<GeneralFieldType, GeneralFieldParseError> {
            Ok(GeneralFieldType::CountdownSpeed(s.parse()?))
        };

        let field_to_string = |s: &GeneralFieldType| s.to_string();

        vec![
            FieldVersion {
                field: "AudioFilename",
                functions: vec![(
                    Some((parse_pathbuf, field_to_string)),
                    MIN_VERSION..=LATEST_VERSION,
                )],
            },
            FieldVersion {
                field: "AudioLeadIn",
                functions: vec![(
                    Some((parse_integer, field_to_string)),
                    MIN_VERSION..=LATEST_VERSION,
                )],
            },
            FieldVersion {
                field: "AudioHash",
                functions: vec![(
                    Some((parse_string, field_to_string)),
                    MIN_VERSION..=LATEST_VERSION,
                )],
            },
            FieldVersion {
                field: "PreviewTime",
                functions: vec![(
                    Some((parse_integer, field_to_string)),
                    MIN_VERSION..=LATEST_VERSION,
                )],
            },
            FieldVersion {
                field: "Countdown",
                functions: vec![(
                    Some((parse_countdown_speed, field_to_string)),
                    MIN_VERSION..=LATEST_VERSION,
                )],
            },
            FieldVersion {
                field: "StackLeniency",
                functions: vec![(
                    Some((parse_decimal, field_to_string)),
                    MIN_VERSION..=LATEST_VERSION,
                )],
            },
            FieldVersion {
                field: "Mode",
                functions: vec![(
                    Some((parse_game_mode, field_to_string)),
                    MIN_VERSION..=LATEST_VERSION,
                )],
            },
            FieldVersion {
                field: "LetterboxInBreaks",
                functions: vec![(
                    Some((parse_bool, field_to_string)),
                    MIN_VERSION..=LATEST_VERSION,
                )],
            },
            FieldVersion {
                field: "StoryFireInFront",
                functions: vec![(
                    Some((parse_bool, field_to_string)),
                    MIN_VERSION..=LATEST_VERSION,
                )],
            },
            FieldVersion {
                field: "UseSkinSprites",
                functions: vec![(
                    Some((parse_bool, field_to_string)),
                    MIN_VERSION..=LATEST_VERSION,
                )],
            },
            FieldVersion {
                field: "AlwaysShowPlayfield",
                functions: vec![(
                    Some((parse_bool, field_to_string)),
                    MIN_VERSION..=LATEST_VERSION,
                )],
            },
            FieldVersion {
                field: "SkinPreference",
                functions: vec![(
                    Some((parse_string, field_to_string)),
                    MIN_VERSION..=LATEST_VERSION,
                )],
            },
            FieldVersion {
                field: "EpilepsyWarning",
                functions: vec![(
                    Some((parse_bool, field_to_string)),
                    MIN_VERSION..=LATEST_VERSION,
                )],
            },
            FieldVersion {
                field: "CountdownOffset",
                functions: vec![(
                    Some((parse_integer, field_to_string)),
                    MIN_VERSION..=LATEST_VERSION,
                )],
            },
            FieldVersion {
                field: "SpecialStyle",
                functions: vec![(
                    Some((parse_bool, field_to_string)),
                    MIN_VERSION..=LATEST_VERSION,
                )],
            },
            FieldVersion {
                field: "WidescreenStoryboard",
                functions: vec![(
                    Some((parse_bool, field_to_string)),
                    MIN_VERSION..=LATEST_VERSION,
                )],
            },
            FieldVersion {
                field: "SamplesMatchPlaybackRate",
                functions: vec![(
                    Some((parse_bool, field_to_string)),
                    MIN_VERSION..=LATEST_VERSION,
                )],
            },
        ]
    };
}

impl General {
    /// Creates an empty `General` instance, with all fields being `None`.
    pub fn new() -> Self {
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

    pub fn from_str(s: &str, version: usize) -> Result<Option<General>, Error<ParseError>> {
        let mut general = General::new();

        let (s, fields) = get_colon_field_value_lines(s).unwrap();

        if !s.trim().is_empty() {
            // line count from fields
            let line_count = { fields.iter().map(|(_, _, ws)| ws.lines().count()).sum() };

            return Err(Error::new(ParseError::InvalidColonSet, line_count));
        }

        let mut line_count = 0;
        let mut parsed_fields = Vec::new();

        for (name, value, ws) in fields {
            let new_into_strum_parse_error = move |err| Error::new_into(err, line_count);

            if parsed_fields.contains(&name) {
                return Err(Error::new(ParseError::DuplicateField, line_count));
            }

            match GENERAL_FIELD_VERSIONS.iter().find(|v| v.field == name) {
                Some(v) => {
                    let value = v
                        .parse(version, value)
                        .ok_or(Error::new(ParseError::InvalidVersion, line_count))?
                        .map_err(|err| Error::new_into(err, line_count))?;

                    match name {
                        "AudioFilename" => general.audio_filename = AudioFilename::from_str(value, version)?,
                        "AudioLeadIn" => general.audio_lead_in = Some(value.try_into().unwrap()),
                        "AudioHash" => general.audio_hash = Some(value.try_into().unwrap()),
                        "PreviewTime" => general.preview_time = Some(value.try_into().unwrap()),
                        "Countdown" => general.countdown = Some(value.try_into().unwrap()),
                        "StackLeniency" => general.stack_leniency = Some(value.try_into().unwrap()),
                        "Mode" => general.mode = Some(value.try_into().unwrap()),
                        "LetterboxInBreaks" => {
                            general.letterbox_in_breaks = Some(value.try_into().unwrap())
                        }
                        "StoryFireInFront" => {
                            general.story_fire_in_front = Some(value.try_into().unwrap())
                        }
                        "UseSkinSprites" => {
                            general.use_skin_sprites = Some(value.try_into().unwrap())
                        }
                        "AlwaysShowPlayfield" => {
                            general.always_show_playfield = Some(value.try_into().unwrap())
                        }
                        "SkinPreference" => {
                            general.skin_preference = Some(value.try_into().unwrap())
                        }
                        "EpilepsyWarning" => {
                            general.epilepsy_warning = Some(value.try_into().unwrap())
                        }
                        "CountdownOffset" => {
                            general.countdown_offset = Some(value.try_into().unwrap())
                        }
                        "SpecialStyle" => general.special_style = Some(value.try_into().unwrap()),
                        "WidescreenStoryboard" => {
                            general.widescreen_storyboard = Some(value.try_into().unwrap())
                        }
                        "SamplesMatchPlaybackRate" => {
                            general.samples_match_playback_rate = Some(value.try_into().unwrap())
                        }
                        _ => return Err(Error::new(ParseError::InvalidKey, line_count)),
                    }
                }
                None => {
                    match name {
                        "SampleSet" => {
                            // TODO probably change the Version trait to have a method for value and version
                            // or use FieldVersion and get rid of Version trait
                            general.sample_set = match version {
                                3 => SampleSet::from_str_v3(value)
                                    .map_err(new_into_strum_parse_error)?,
                                4 => SampleSet::from_str_v4(value)
                                    .map_err(new_into_strum_parse_error)?,
                                5 => SampleSet::from_str_v5(value)
                                    .map_err(new_into_strum_parse_error)?,
                                6 => SampleSet::from_str_v6(value)
                                    .map_err(new_into_strum_parse_error)?,
                                7 => SampleSet::from_str_v7(value)
                                    .map_err(new_into_strum_parse_error)?,
                                8 => SampleSet::from_str_v8(value)
                                    .map_err(new_into_strum_parse_error)?,
                                9 => SampleSet::from_str_v9(value)
                                    .map_err(new_into_strum_parse_error)?,
                                10 => SampleSet::from_str_v10(value)
                                    .map_err(new_into_strum_parse_error)?,
                                11 => SampleSet::from_str_v11(value)
                                    .map_err(new_into_strum_parse_error)?,
                                12 => SampleSet::from_str_v12(value)
                                    .map_err(new_into_strum_parse_error)?,
                                13 => SampleSet::from_str_v13(value)
                                    .map_err(new_into_strum_parse_error)?,
                                14 => SampleSet::from_str_v14(value)
                                    .map_err(new_into_strum_parse_error)?,
                                _ => None,
                            };
                        }
                        "OverlayPosition" => {
                            general.overlay_position = match version {
                                3 => OverlayPosition::from_str_v3(value)
                                    .map_err(new_into_strum_parse_error)?,
                                4 => OverlayPosition::from_str_v4(value)
                                    .map_err(new_into_strum_parse_error)?,
                                5 => OverlayPosition::from_str_v5(value)
                                    .map_err(new_into_strum_parse_error)?,
                                6 => OverlayPosition::from_str_v6(value)
                                    .map_err(new_into_strum_parse_error)?,
                                7 => OverlayPosition::from_str_v7(value)
                                    .map_err(new_into_strum_parse_error)?,
                                8 => OverlayPosition::from_str_v8(value)
                                    .map_err(new_into_strum_parse_error)?,
                                9 => OverlayPosition::from_str_v9(value)
                                    .map_err(new_into_strum_parse_error)?,
                                10 => OverlayPosition::from_str_v10(value)
                                    .map_err(new_into_strum_parse_error)?,
                                11 => OverlayPosition::from_str_v11(value)
                                    .map_err(new_into_strum_parse_error)?,
                                12 => OverlayPosition::from_str_v12(value)
                                    .map_err(new_into_strum_parse_error)?,
                                13 => OverlayPosition::from_str_v13(value)
                                    .map_err(new_into_strum_parse_error)?,
                                14 => OverlayPosition::from_str_v14(value)
                                    .map_err(new_into_strum_parse_error)?,
                                _ => None,
                            };
                        }
                        _ => return Err(Error::new(ParseError::InvalidKey, line_count)),
                    }
                }
            }
            // .ok_or(Error::new(ParseError::InvalidKey, line_count))?;

            line_count += ws.lines().count();
            parsed_fields.push(name);
        }

        // in case future versions don't have general, we use option
        Ok(Some(general))
    }

    pub fn to_string(&self, version: usize) -> Option<String> {
        let mut fields = Vec::new();

        let pathbuf = |name, value: &Option<PathBuf>, fields: &mut Vec<_>| {
            if let Some(value) = value {
                let value = value.to_owned();
                if let Some(value) = GENERAL_FIELD_VERSIONS
                    .iter()
                    .find(|v| v.field == name)
                    .unwrap()
                    .display(version, &GeneralFieldType::PathBuf(value))
                {
                    fields.push((name, value));
                }
            }
        };
        let string = |name, value: &Option<String>, fields: &mut Vec<_>| {
            if let Some(value) = value {
                let value = value.to_owned();
                if let Some(value) = GENERAL_FIELD_VERSIONS
                    .iter()
                    .find(|v| v.field == name)
                    .unwrap()
                    .display(version, &GeneralFieldType::String(value))
                {
                    fields.push((name, value));
                }
            }
        };
        let int = |name, value: &Option<Integer>, fields: &mut Vec<_>| {
            if let Some(value) = value {
                let value = value.to_owned();
                if let Some(value) = GENERAL_FIELD_VERSIONS
                    .iter()
                    .find(|v| v.field == name)
                    .unwrap()
                    .display(version, &GeneralFieldType::Integer(value))
                {
                    fields.push((name, value));
                }
            }
        };
        let bool = |name, value: &Option<bool>, fields: &mut Vec<_>| {
            if let Some(value) = value {
                let value = value.to_owned();
                if let Some(value) = GENERAL_FIELD_VERSIONS
                    .iter()
                    .find(|v| v.field == name)
                    .unwrap()
                    .display(version, &GeneralFieldType::Bool(value))
                {
                    fields.push((name, value));
                }
            }
        };
        let decimal = |name, value: &Option<Decimal>, fields: &mut Vec<_>| {
            if let Some(value) = value {
                let value = value.to_owned();
                if let Some(value) = GENERAL_FIELD_VERSIONS
                    .iter()
                    .find(|v| v.field == name)
                    .unwrap()
                    .display(version, &GeneralFieldType::Decimal(value))
                {
                    fields.push((name, value));
                }
            }
        };

        pathbuf("AudioFilename", &self.audio_filename, &mut fields);
        int("AudioLeadIn", &self.audio_lead_in, &mut fields);
        string("AudioHash", &self.audio_hash, &mut fields);
        int("PreviewTime", &self.preview_time, &mut fields);
        if let Some(countdown) = &self.countdown {
            let countdown = match version {
                3 => countdown.to_string_v3(),
                4 => countdown.to_string_v4(),
                5 => countdown.to_string_v5(),
                6 => countdown.to_string_v6(),
                7 => countdown.to_string_v7(),
                8 => countdown.to_string_v8(),
                9 => countdown.to_string_v9(),
                10 => countdown.to_string_v10(),
                11 => countdown.to_string_v11(),
                12 => countdown.to_string_v12(),
                13 => countdown.to_string_v13(),
                14 => countdown.to_string_v14(),
                _ => None,
            };

            if let Some(countdown) = countdown {
                fields.push(("Countdown", countdown));
            }
        }
        if let Some(sample_set) = &self.sample_set {
            let countdown = match version {
                3 => sample_set.to_string_v3(),
                4 => sample_set.to_string_v4(),
                5 => sample_set.to_string_v5(),
                6 => sample_set.to_string_v6(),
                7 => sample_set.to_string_v7(),
                8 => sample_set.to_string_v8(),
                9 => sample_set.to_string_v9(),
                10 => sample_set.to_string_v10(),
                11 => sample_set.to_string_v11(),
                12 => sample_set.to_string_v12(),
                13 => sample_set.to_string_v13(),
                14 => sample_set.to_string_v14(),
                _ => None,
            };

            if let Some(countdown) = countdown {
                fields.push(("SampleSet", countdown));
            }
        }
        decimal("StackLeniency", &self.stack_leniency, &mut fields);
        if let Some(mode) = &self.mode {
            let mode = mode.to_owned();
            if let Some(value) = GENERAL_FIELD_VERSIONS
                .iter()
                .find(|v| v.field == "Mode")
                .unwrap()
                .display(version, &GeneralFieldType::GameMode(mode))
            {
                fields.push(("Mode", value));
            }
        }
        bool("LetterboxInBreaks", &self.letterbox_in_breaks, &mut fields);
        bool("StoryFireInFront", &self.story_fire_in_front, &mut fields);
        bool("UseSkinSprites", &self.use_skin_sprites, &mut fields);
        bool(
            "AlwaysShowPlayfield",
            &self.always_show_playfield,
            &mut fields,
        );
        if let Some(overlay_position) = &self.overlay_position {
            let countdown = match version {
                3 => overlay_position.to_string_v3(),
                4 => overlay_position.to_string_v4(),
                5 => overlay_position.to_string_v5(),
                6 => overlay_position.to_string_v6(),
                7 => overlay_position.to_string_v7(),
                8 => overlay_position.to_string_v8(),
                9 => overlay_position.to_string_v9(),
                10 => overlay_position.to_string_v10(),
                11 => overlay_position.to_string_v11(),
                12 => overlay_position.to_string_v12(),
                13 => overlay_position.to_string_v13(),
                14 => overlay_position.to_string_v14(),
                _ => None,
            };

            if let Some(countdown) = countdown {
                fields.push(("OverlayPosition", countdown));
            }
        }
        string("SkinPreference", &self.skin_preference, &mut fields);
        bool("EpilepsyWarning", &self.epilepsy_warning, &mut fields);
        int("CountdownOffset", &self.countdown_offset, &mut fields);
        bool("SpecialStyle", &self.special_style, &mut fields);
        bool(
            "WidescreenStoryboard",
            &self.widescreen_storyboard,
            &mut fields,
        );
        bool(
            "SamplesMatchPlaybackRate",
            &self.samples_match_playback_rate,
            &mut fields,
        );

        Some(
            fields
                .iter()
                .map(|(name, value)| format!("{}: {}", name, value))
                .collect::<Vec<_>>()
                .join("\n"),
        )
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
    type Err = ParseCountdownSpeedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        CountdownSpeed::from_repr(s.parse()?).ok_or(ParseCountdownSpeedError::UnknownType)
    }
}

impl Default for CountdownSpeed {
    fn default() -> Self {
        Self::Normal
    }
}

impl Version for CountdownSpeed {
    type ParseError = ParseCountdownSpeedError;

    // TODO investigate versions
    fn from_str_v3(s: &str) -> std::result::Result<Option<Self>, Self::ParseError>
    where
        Self: Sized,
    {
        s.parse().map(Some)
    }

    fn to_string_v3(&self) -> Option<String> {
        Some((*self as usize).to_string())
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

impl Version for SampleSet {
    type ParseError = strum::ParseError;

    // TODO investigate versions
    fn from_str_v3(_: &str) -> std::result::Result<Option<Self>, Self::ParseError>
    where
        Self: Sized,
    {
        Ok(None)
    }

    fn to_string_v3(&self) -> Option<String> {
        None
    }

    fn from_str_v5(s: &str) -> std::result::Result<Option<Self>, Self::ParseError>
    where
        Self: Sized,
    {
        s.parse().map(Some)
    }

    fn to_string_v5(&self) -> Option<String> {
        // I dont think we have to revert to None
        Some(self.to_string())
    }

    fn from_str_v14(s: &str) -> std::result::Result<Option<Self>, Self::ParseError>
    where
        Self: Sized,
    {
        let mut sample_set = s.parse()?;

        if let SampleSet::None = sample_set {
            sample_set = SampleSet::Normal;
        };

        Ok(Some(sample_set))
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
    type Err = ParseGameModeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        GameMode::from_repr(s.parse()?).ok_or(ParseGameModeError::UnknownType)
    }
}

impl From<GameMode> for Integer {
    fn from(value: GameMode) -> Self {
        value as Integer
    }
}

impl Version for GameMode {
    type ParseError = ParseGameModeError;

    // TODO check what gamemodes exist in versions
    fn from_str_v3(s: &str) -> std::result::Result<Option<Self>, Self::ParseError>
    where
        Self: Sized,
    {
        s.parse().map(Some)
    }

    fn to_string_v3(&self) -> Option<String> {
        Some((*self as usize).to_string())
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

impl Version for OverlayPosition {
    type ParseError = strum::ParseError;

    // TODO investigate versions
    fn from_str_v3(s: &str) -> std::result::Result<Option<Self>, Self::ParseError>
    where
        Self: Sized,
    {
        s.parse().map(Some)
    }

    fn to_string_v3(&self) -> Option<String> {
        Some(self.to_string())
    }
}

// TODO separate types in types.rs
