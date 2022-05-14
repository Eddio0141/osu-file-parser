pub mod colours;
pub mod difficulty;
pub mod editor;
pub mod events;
pub mod general;
pub mod hitobject;
pub mod metadata;
pub mod timingpoint;
pub mod types;

use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::str::FromStr;

use nom::bytes::complete::{tag, take_till};
use nom::character::complete::{char, multispace0};
use nom::combinator::map_res;
use nom::multi::many0;
use nom::sequence::{delimited, tuple};
use thiserror::Error;

use crate::parsers::*;

use self::colours::Colours;
use self::difficulty::Difficulty;
use self::editor::Editor;
use self::events::Events;
use self::general::General;
use self::hitobject::HitObjects;
use self::metadata::Metadata;
use self::timingpoint::TimingPoints;

use self::types::*;

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
    type Err = Error<ParseError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let version_text = tag::<_, _, nom::error::Error<_>>("osu file format v");
        let version_number = map_res(
            trailing_ws(take_till(|ch| ch == '\r' || ch == '\n')),
            |s: &str| s.parse::<Integer>(),
        );

        let section_open = char::<_, nom::error::Error<_>>('[');
        let section_close = char(']');
        let section_name_inner = take_till(|c: char| c == ']' || c == '\r' || c == '\n');
        let section_name = delimited(section_open, section_name_inner, section_close);

        let section_until = take_till(|c| c == '[');
        let section = tuple((multispace0, section_name, multispace0, section_until));

        let (s, (_, version)) = match tuple((version_text, version_number))(s) {
            Ok(ok) => ok,
            Err(err) => {
                // wrong line?
                let err = if s.starts_with('\n') || s.starts_with("\r\n") {
                    ParseError::FileVersionInWrongLine
                } else if let nom::Err::Error(err) = err {
                    // TODO fix this mess
                    match err.code {
                        nom::error::ErrorKind::Tag => ParseError::FileVersionDefinedWrong,
                        nom::error::ErrorKind::MapRes => ParseError::InvalidFileVersion,
                        _ => {
                            unreachable!("Not possible to have the error kind {:#?}", err.code)
                        }
                    }
                } else {
                    unreachable!("Not possible to reach when the errors are already handled");
                };

                return Err(Error {
                    line_index: 0,
                    error: err,
                });
            }
        };

        if version > LATEST_VERSION || version < MIN_VERSION {
            return Err(Error {
                line_index: 0,
                error: ParseError::InvalidFileVersion,
            });
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
        ) = (None, None, None, None, None, None, None, None);

        let mut line_number = 1;

        // TODO eventually remove this
        fn parse_error_to_error<P>(
            result: Result<P, <P as FromStr>::Err>,
            line_number: usize,
        ) -> Result<P, Error<ParseError>>
        where
            P: FromStr,
            ParseError: From<P::Err>,
        {
            match result {
                Ok(ok) => Ok(ok),
                Err(err) => Err(Error {
                    line_index: line_number,
                    error: err.into(),
                }),
            }
        }

        for (ws, section_name, ws2, section) in sections {
            line_number += ws.lines().count();

            if section_parsed.contains(&section_name) {
                return Err(Error {
                    line_index: line_number,
                    error: ParseError::DuplicateSections,
                });
            }

            let section_name_line = line_number;
            line_number += ws2.lines().count();

            match section_name {
                "General" => {
                    general = Some(Error::combine_result(section.parse(), line_number + 1)?)
                }
                "Editor" => editor = Some(parse_error_to_error(section.parse(), line_number + 1)?),
                "Metadata" => {
                    metadata = Some(parse_error_to_error(section.parse(), line_number + 1)?)
                }
                "Difficulty" => {
                    difficulty = Some(parse_error_to_error(section.parse(), line_number + 1)?)
                }
                "Events" => events = Some(Error::combine_result(section.parse(), line_number + 1)?),
                "TimingPoints" => {
                    timing_points = Some(parse_error_to_error(section.parse(), line_number + 1)?)
                }
                "Colours" => {
                    colours = Some(parse_error_to_error(section.parse(), line_number + 1)?)
                }
                "HitObjects" => {
                    hitobjects = Some(parse_error_to_error(section.parse(), line_number + 1)?)
                }
                _ => {
                    return Err(Error {
                        line_index: section_name_line,
                        error: ParseError::UnknownSection,
                    })
                }
            }

            section_parsed.push(section_name);
            line_number += section.lines().count();
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
#[non_exhaustive]
/// Error for when there's a problem parsing an .osu file.
pub enum ParseError {
    /// File version is invalid.
    // TODO redesign this error
    // TODO multiple file versions for this crate somehow
    #[error("Invalid file version, expected versions from {MIN_VERSION} ~ {LATEST_VERSION}")]
    InvalidFileVersion,
    /// File version is defined wrong.
    #[error("File version defined wrong, expected `osu file format v..` at the first line")]
    FileVersionDefinedWrong,
    /// File version not defined in line 1.
    #[error("Found file version definition, but not defined at the first line")]
    FileVersionInWrongLine,
    /// Duplicate section names defined.
    #[error("There are multiple sections defined as the same name")]
    DuplicateSections,
    /// Unknown section name defined.
    #[error("There is an unknown section")]
    UnknownSection,
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
        source: general::ParseError,
    },
    /// Error parsing the editor section.
    #[error(transparent)]
    EditorParseError {
        #[from]
        source: editor::ParseError,
    },
    /// Error parsing the metadata section.
    #[error(transparent)]
    MetadataParseError {
        #[from]
        source: metadata::MetadataParseError,
    },
    /// Error parsing the difficulty section.
    #[error(transparent)]
    DifficultyParseError {
        #[from]
        source: difficulty::DifficultyParseError,
    },
    /// Error parsing the events section.
    #[error(transparent)]
    EventsParseError {
        #[from]
        source: events::ParseError,
    },
    /// Error parsing the timingpoints section.
    #[error(transparent)]
    TimingPointsParseError {
        #[from]
        source: timingpoint::ParseError,
    },
    /// Error parsing the colours section.
    #[error(transparent)]
    ColoursParseError {
        #[from]
        source: colours::ColoursParseError,
    },
    /// Error parsing the hitobjects section.
    #[error(transparent)]
    HitObjectsParseError {
        #[from]
        source: hitobject::HitObjectsParseError,
    },
}

const LATEST_VERSION: Integer = 14;
const MIN_VERSION: Integer = 3;

const SECTION_DELIMITER: &str = ":";
