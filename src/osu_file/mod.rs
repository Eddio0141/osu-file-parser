pub mod colours;
pub mod difficulty;
pub mod editor;
pub mod events;
pub mod general;
mod helper;
pub mod hitobject;
pub mod metadata;
mod parsers;
pub mod timingpoint;

use std::fmt::Display;
use std::hash::Hash;
use std::num::ParseIntError;
use std::str::FromStr;

use nom::bytes::complete::{tag, take_till, take_until};
use nom::character::complete::char;
use nom::character::is_newline;
use nom::combinator::map_res;
use nom::multi::many0;
use nom::sequence::{delimited, tuple};
use thiserror::Error;

use self::colours::{Colours, ColoursParseError};
use self::difficulty::{Difficulty, DifficultyParseError};
use self::editor::{Editor, EditorParseError};
use self::events::{Events, EventsParseError};
use self::general::{General, GeneralParseError};
use self::hitobject::{HitObjects, HitObjectsParseError};
use self::metadata::{Metadata, MetadataParseError};

use self::parsers::{leading_ws, ws};
use self::timingpoint::{TimingPoints, TimingPointsParseError};

// TODO use the crate https://crates.io/crates/nom
/// An .osu file represented as a struct.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct OsuFile {
    /// Version of the file format.
    pub version: Integer,
    /// General information about the beatmap.
    /// - `key`: `value` pairs.
    pub general: Option<General>,
    /// Saved settings for the beatmap editor.
    /// - `key`: `value` pairs.
    pub editor: Option<Editor>,
    /// Information used to identify the beatmap.
    /// - `key`:`value` pairs.
    pub metadata: Option<Metadata>,
    /// Difficulty settings.
    /// - `key`:`value` pairs.
    pub difficulty: Option<Difficulty>,
    /// Beatmap and storyboard graphic events.
    /// Comma-separated lists.
    pub events: Option<Events>,
    /// Timing and control points.
    /// Comma-separated lists.
    pub timing_points: Option<TimingPoints>,
    /// Combo and skin colours.
    /// `key` : `value` pairs.
    pub colours: Option<Colours>,
    /// Hit objects.
    /// Comma-separated lists.
    pub hitobjects: Option<HitObjects>,
}

impl OsuFile {
    pub fn empty() -> Self {
        Self {
            version: Default::default(),
            general: None,
            editor: None,
            metadata: None,
            difficulty: None,
            events: None,
            timing_points: None,
            colours: None,
            hitobjects: None,
        }
    }
}

impl Display for OsuFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO .osb file too

        let sections = vec![
            format!("osu file format v{}", self.version),
            match &self.general {
                Some(general) => format!("[General]\r\n{}", general),
                None => String::new(),
            },
            match &self.editor {
                Some(editor) => format!("[Editor]\r\n{}", editor),
                None => String::new(),
            },
            match &self.metadata {
                Some(metadata) => format!("[Metadata]\r\n{}", metadata),
                None => String::new(),
            },
            match &self.difficulty {
                Some(difficulty) => format!("[Difficulty]\r\n{}", difficulty),
                None => String::new(),
            },
            match &self.events {
                Some(events) => format!("[Events]\r\n{}", events),
                None => String::new(),
            },
            match &self.timing_points {
                Some(timing_points) => format!("[TimingPoints]\r\n{}", timing_points),
                None => String::new(),
            },
            match &self.colours {
                Some(colours) => format!("[Colours]\r\n{}", colours),
                None => String::new(),
            },
            match &self.hitobjects {
                Some(hitobjects) => format!("[HitObjects]\r\n{}", hitobjects),
                None => String::new(),
            },
        ];

        write!(f, "{}", sections.join("\r\n"))
    }
}

impl FromStr for OsuFile {
    type Err = OsuFileParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let version_text = tag::<_, _, nom::error::Error<_>>("osu file format v");
        let version_number = map_res(take_until("\n"), |s: &str| s.parse::<Integer>());

        let section_open = char::<_, nom::error::Error<_>>('[');
        let section_close = char(']');
        let section_name_inner = take_till(|c: char| c == ']' || is_newline(c as u8));
        let section_name = delimited(section_open, section_name_inner, section_close);

        let section_until = take_till(|c| c == '[');
        let section = tuple((ws(section_name), section_until));

        // TODO errors
        let (s, (_, version)) = tuple((leading_ws(version_text), version_number))(s).unwrap();
        let (_, sections) = many0(section)(s).unwrap();

        let mut section_parsed = Vec::with_capacity(8);

        let (
            mut general,
            mut editor,
            mut metadata,
            mut difficulty,
            mut events,
            mut timing_points,
            mut colours,
            mut hitobjects,
        ) = (
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        );

        for (section_name, section) in sections {
            if section_parsed.contains(&section_name) {
                return Err(OsuFileParseError::DuplicateSections(
                    section_name.to_string(),
                ));
            }

            match section_name {
                "General" => general = Some(section.parse()?),
                "Editor" => editor = Some(section.parse()?),
                "Metadata" => metadata = Some(section.parse()?),
                "Difficulty" => difficulty = Some(section.parse()?),
                "Events" => events = Some(section.parse()?),
                "TimingPoints" => timing_points = Some(section.parse()?),
                "Colours" => colours = Some(section.parse()?),
                "HitObjects" => hitobjects = Some(section.parse()?),
                _ => {
                    return Err(OsuFileParseError::UnknownSectionName(
                        section_name.to_string(),
                    ))
                }
            }

            section_parsed.push(section_name);
        }

        Ok(OsuFile {
            version,
            general,
            editor,
            metadata,
            difficulty,
            events,
            timing_points,
            colours,
            hitobjects,
        })
    }
}

impl Default for OsuFile {
    fn default() -> Self {
        Self {
            version: LATEST_VERSION,
            general: Some(Default::default()),
            editor: Some(Default::default()),
            metadata: Some(Default::default()),
            difficulty: Some(Default::default()),
            events: Some(Default::default()),
            timing_points: Some(Default::default()),
            colours: Some(Default::default()),
            hitobjects: Some(Default::default()),
        }
    }
}

#[derive(Debug, Error)]
/// Error for when there's a problem parsing an .osu file.
pub enum OsuFileParseError {
    /// File version is invalid.
    // TODO multiple file versions for this crate somehow
    #[error("Invalid file version, expected {LATEST_VERSION}, got {0}")]
    InvalidFileVersion(Integer),
    /// File version parsing failed.
    #[error("Invalid file version, expected version in an `Integer` form, got {value}")]
    FileVersionParseError {
        #[source]
        source: ParseIntError,
        value: String,
    },
    /// No file version defined.
    #[error("No file version defined, expected `osu file format v..` at the first line")]
    NoFileVersion,
    /// More than 1 file version defined.
    #[error("Multiple file versions defined, only requires one file version: {0}")]
    MultipleFileVersions(String),
    /// File version not defined in line 1.
    #[error("Found file version definition, but in the line {0}, expected to be in line 1")]
    FileVersionInWrongLine(usize),
    /// Duplicate section names defined.
    #[error("There are multiple sections defined as the same name {0}")]
    DuplicateSections(String),
    /// Unknown section name defined.
    #[error("There is an unknown section name `{0}`")]
    UnknownSectionName(String),
    /// Error used when the opening bracket for the section is missing.
    #[error("The opening bracket of the section is missing, expected `{SECTION_OPEN}` before {0}")]
    SectionNameNoOpenBracket(String),
    /// Error used when the closing bracket for the section is missing.
    #[error("The closing bracket of the section is missing, expected `{SECTION_CLOSE}` after {0}")]
    SectionNameNoCloseBracket(String),
    /// Error parsing the general section.
    #[error(transparent)]
    GeneralParseError {
        #[from]
        source: GeneralParseError,
    },
    /// Error parsing the editor section.
    #[error(transparent)]
    EditorParseError {
        #[from]
        source: EditorParseError,
    },
    /// Error parsing the metadata section.
    #[error(transparent)]
    MetadataParseError {
        #[from]
        source: MetadataParseError,
    },
    /// Error parsing the difficulty section.
    #[error(transparent)]
    DifficultyParseError {
        #[from]
        source: DifficultyParseError,
    },
    /// Error parsing the events section.
    #[error(transparent)]
    EventsParseError {
        #[from]
        source: EventsParseError,
    },
    /// Error parsing the timingpoints section.
    #[error(transparent)]
    TimingPointsParseError {
        #[from]
        source: TimingPointsParseError,
    },
    /// Error parsing the colours section.
    #[error(transparent)]
    ColoursParseError {
        #[from]
        source: ColoursParseError,
    },
    /// Error parsing the hitobjects section.
    #[error(transparent)]
    HitObjectsParseError {
        #[from]
        source: HitObjectsParseError,
    },
}

/// Latest file version.
const LATEST_VERSION: Integer = 14;

/// Delimiter for the `key: value` pair.
const SECTION_DELIMITER: &str = ":";
/// Section name open bracket.
const SECTION_OPEN: char = '[';
/// Section name close bracket.
const SECTION_CLOSE: char = ']';

/// Definition of the `Integer` type.
type Integer = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// The position of something in `osu!pixels` with the `x` `y` form.
pub struct Position {
    /// x coordinate.
    pub x: Integer,
    /// y coordinate
    pub y: Integer,
}

impl Default for Position {
    fn default() -> Self {
        Self { x: 256, y: 192 }
    }
}
