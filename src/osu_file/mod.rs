pub mod colours;
pub mod difficulty;
pub mod editor;
pub mod events;
pub mod general;
pub mod hitobject;
pub mod metadata;
pub mod timingpoint;

use std::fmt::Display;
use std::hash::Hash;
use std::str::FromStr;

use nom::bytes::complete::{tag, take_till};
use nom::character::complete::char;
use nom::character::is_newline;
use nom::combinator::map_res;
use nom::multi::many0;
use nom::sequence::{delimited, tuple};
use thiserror::Error;

use crate::parsers::*;

use self::colours::{error::ColoursParseError, Colours};
use self::difficulty::{error::DifficultyParseError, Difficulty};
use self::editor::{error::EditorParseError, Editor};
use self::events::{error::EventsParseError, Events};
use self::general::error::GeneralParseError;
use self::general::General;
use self::hitobject::{HitObjects, HitObjectsParseError};
use self::metadata::{error::MetadataParseError, Metadata};

use self::timingpoint::{TimingPoints, TimingPointsParseError};

/// An .osu file represented as a struct.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[non_exhaustive]
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
    /// New `OsuFile` with no data.
    /// - Difference from `Default` is that all fields are `None` instead of Some(`Default`).
    pub fn new() -> Self {
        Self {
            version: LATEST_VERSION,
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

        let mut sections = Vec::with_capacity(9);

        sections.push(format!("osu file format v{}", self.version));

        if let Some(general) = &self.general {
            sections.push(format!("[General]\n{}", general));
        }
        if let Some(editor) = &self.editor {
            sections.push(format!("[Editor]\n{}", editor));
        }
        if let Some(metadata) = &self.metadata {
            sections.push(format!("[Metadata]\n{}", metadata));
        }
        if let Some(difficulty) = &self.difficulty {
            sections.push(format!("[Difficulty]\n{}", difficulty));
        }
        if let Some(events) = &self.events {
            sections.push(format!("[Events]\n{}", events));
        }
        if let Some(timing_points) = &self.timing_points {
            sections.push(format!("[TimingPoints]\n{}", timing_points));
        }
        if let Some(colours) = &self.colours {
            sections.push(format!("[Colours]\n{}", colours));
        }
        if let Some(hitobjects) = &self.hitobjects {
            sections.push(format!("[HitObjects]\n{}", hitobjects));
        }

        write!(f, "{}", sections.join("\n\n"))
    }
}

impl FromStr for OsuFile {
    type Err = OsuFileParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let version_text = tag::<_, _, nom::error::Error<_>>("osu file format v");
        let version_number = map_res(
            trailing_ws(take_till(|ch| ch == '\r' || ch == '\n')),
            |s: &str| s.parse::<Integer>(),
        );

        let section_open = char::<_, nom::error::Error<_>>('[');
        let section_close = char(']');
        let section_name_inner = take_till(|c: char| c == ']' || is_newline(c as u8));
        let section_name = delimited(section_open, section_name_inner, section_close);

        let section_until = take_till(|c| c == '[');
        let section = tuple((ws(section_name), section_until));

        // TODO fix this mess
        let (s, (_, version)) = match tuple((version_text, version_number))(s) {
            Ok(ok) => ok,
            Err(err) => {
                // wrong line?
                if s.starts_with('\n') || s.starts_with("\r\n") {
                    return Err(OsuFileParseError::FileVersionInWrongLine);
                } else if let nom::Err::Error(err) = err {
                    let err = match err.code {
                        nom::error::ErrorKind::Tag => {
                            Err(OsuFileParseError::FileVersionDefinedWrong)
                        }
                        nom::error::ErrorKind::MapRes => {
                            Err(OsuFileParseError::InvalidFileVersion(
                                s.lines()
                                    .next()
                                    .unwrap()
                                    .strip_prefix("osu file format v")
                                    .unwrap()
                                    .to_string(),
                            ))
                        }
                        _ => {
                            unreachable!("Not possible to have the error kind {:#?}", err.code)
                        }
                    };

                    return err;
                } else {
                    unreachable!("Not possible to reach when the errors are already handled");
                }
            }
        };

        if version > LATEST_VERSION {
            return Err(OsuFileParseError::InvalidFileVersion(version.to_string()));
        }

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

// TODO way of combining the Error types together as well as line_number being calculated
pub struct Error<E> {
    pub line_number: usize,
    pub error: E,
}

impl<E> Error<E>
where
    E: std::fmt::Display,
{
    /// Shows a pretty error message with the affected line and the error.
    /// - Expensive than showing line number and error with the `Display` trait, as this iterates over the lines of the file input string.
    pub fn display_error_with_line(
        &self,
        f: &mut std::fmt::Formatter,
        file_input: &str,
    ) -> std::fmt::Result {
        let line = file_input.lines().nth(self.line_number).unwrap_or_default();

        writeln!(f, "Line {}: {}", self.line_number, line)?;
        writeln!(f, "{}", self.error)
    }
}

impl<E> Display for Error<E>
where
    E: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Line {}", self.line_number)?;
        writeln!(f, "{}", self.error)
    }
}

#[derive(Debug, Error)]
#[non_exhaustive]
/// Error for when there's a problem parsing an .osu file.
pub enum OsuFileParseError {
    /// File version is invalid.
    // TODO multiple file versions for this crate somehow
    // TODO first file version number?
    #[error("Invalid file version, expected integer from 1 ~ {LATEST_VERSION}, got {0}")]
    InvalidFileVersion(String),
    /// File version is defined wrong.
    #[error("File version defined wrong, expected `osu file format v..` at the first line")]
    FileVersionDefinedWrong,
    /// File version not defined in line 1.
    #[error("Found file version definition, but not defined at the first line")]
    FileVersionInWrongLine,
    /// Duplicate section names defined.
    #[error("There are multiple sections defined as the same name {0}")]
    DuplicateSections(String),
    /// Unknown section name defined.
    #[error("There is an unknown section name `{0}`")]
    UnknownSectionName(String),
    /// Error used when the opening bracket for the section is missing.
    #[error("The opening bracket of the section is missing, expected `[` before {0}")]
    SectionNameNoOpenBracket(String),
    /// Error used when the closing bracket for the section is missing.
    #[error("The closing bracket of the section is missing, expected `]` after {0}")]
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

const LATEST_VERSION: Integer = 14;
// const MIN_VERSION: Integer = 3;

const SECTION_DELIMITER: &str = ":";

/// Definition of the `Integer` type.
pub type Integer = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// The position of something in `osu!pixels` with the `x` `y` form.
pub struct Position {
    /// x coordinate.
    pub x: Integer,
    /// y coordinate.
    pub y: Integer,
}

impl Default for Position {
    fn default() -> Self {
        Self { x: 256, y: 192 }
    }
}
