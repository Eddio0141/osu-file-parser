pub mod colours;
pub mod difficulty;
pub mod editor;
pub mod events;
pub mod general;
pub mod hitobjects;
pub mod metadata;
pub mod timingpoints;
pub mod types;

use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::str::FromStr;

use nom::bytes::complete::{tag, take_till};
use nom::character::complete::multispace0;
use nom::combinator::map_res;
use nom::multi::many0;
use nom::sequence::{delimited, tuple};
use thiserror::Error;

pub use self::colours::Colours;
pub use self::difficulty::Difficulty;
pub use self::editor::Editor;
pub use self::events::Events;
pub use self::general::General;
pub use self::hitobjects::HitObjects;
pub use self::metadata::Metadata;
pub use self::timingpoints::TimingPoints;

pub use self::types::*;

/// An .osu file represented as a struct.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[non_exhaustive]
pub struct OsuFile {
    /// Version of the file format.
    pub version: usize,
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

impl Default for OsuFile {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for OsuFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO .osb file too

        let mut sections = Vec::with_capacity(9);

        sections.push(format!("osu file format v{}", self.version));

        if let Some(general) = &self.general {
            if let Some(general) = general.to_string(self.version) {
                sections.push(format!("[General]\n{general}",))
            }
        }
        if let Some(editor) = &self.editor {
            if let Some(editor) = editor.to_string(self.version) {
                sections.push(format!("[Editor]\n{editor}"));
            }
        }
        if let Some(metadata) = &self.metadata {
            if let Some(metadata) = metadata.to_string(self.version) {
                sections.push(format!("[Metadata]\n{metadata}"));
            }
        }
        if let Some(difficulty) = &self.difficulty {
            if let Some(difficulty) = difficulty.to_string(self.version) {
                sections.push(format!("[Difficulty]\n{difficulty}"));
            }
        }
        if let Some(events) = &self.events {
            if let Some(events) = events.to_string(self.version) {
                sections.push(format!("[Events]\n{events}"));
            }
        }
        if let Some(timing_points) = &self.timing_points {
            if let Some(timing_points) = timing_points.to_string(self.version) {
                let section = format!("[TimingPoints]\n{timing_points}");

                // for some reason theres an extra new line at the end in some versions
                if self.version == 3 || (6..=13).contains(&self.version) {
                    sections.push(format!("{section}\n"));
                } else {
                    sections.push(section);
                }
            }
        }
        if let Some(colours) = &self.colours {
            if let Some(colours) = colours.to_string(self.version) {
                sections.push(format!("[Colours]\n{colours}"));
            }
        }
        if let Some(hitobjects) = &self.hitobjects {
            if let Some(hitobjects) = hitobjects.to_string(self.version) {
                sections.push(format!("[HitObjects]\n{hitobjects}"));
            }
        }

        write!(f, "{}", sections.join("\n\n"))?;
        // for some reason below v14 theres another new line at the end
        if self.version < 14 {
            writeln!(f)?;
        }

        Ok(())
    }
}

impl FromStr for OsuFile {
    type Err = Error<ParseError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let version_text = tag::<_, _, nom::error::Error<_>>("osu file format v");
        let version_number = map_res(take_till(|c| c == '\r' || c == '\n'), |s: &str| s.parse());

        let section_open = tag::<_, _, nom::error::Error<_>>("[");
        let section_close = tag("]");
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

                return Err(err.into());
            }
        };

        if !(MIN_VERSION..=LATEST_VERSION).contains(&version) {
            return Err(ParseError::InvalidFileVersion.into());
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

        let mut line_number = 0;

        for (ws, section_name, ws2, section) in sections {
            line_number += ws.lines().count();

            if section_parsed.contains(&section_name) {
                return Err(Error::new(ParseError::DuplicateSections, line_number));
            }

            let section_name_line = line_number;
            line_number += ws2.lines().count();

            match section_name {
                "General" => {
                    general =
                        Error::processing_line(General::from_str(section, version), line_number)?
                }
                "Editor" => {
                    editor =
                        Error::processing_line(Editor::from_str(section, version), line_number)?
                }
                "Metadata" => {
                    metadata =
                        Error::processing_line(Metadata::from_str(section, version), line_number)?
                }
                "Difficulty" => {
                    difficulty =
                        Error::processing_line(Difficulty::from_str(section, version), line_number)?
                }
                "Events" => {
                    events =
                        Error::processing_line(Events::from_str(section, version), line_number)?
                }
                "TimingPoints" => {
                    timing_points = Error::processing_line(
                        TimingPoints::from_str(section, version),
                        line_number,
                    )?
                }
                "Colours" => {
                    colours =
                        Error::processing_line(Colours::from_str(section, version), line_number)?
                }
                "HitObjects" => {
                    hitobjects =
                        Error::processing_line(HitObjects::from_str(section, version), line_number)?
                }
                _ => return Err(Error::new(ParseError::UnknownSection, section_name_line)),
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

#[derive(Debug, Error)]
#[non_exhaustive]
/// Error for when there's a problem parsing an .osu file.
pub enum ParseError {
    /// File version is invalid.
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
        source: metadata::ParseError,
    },
    /// Error parsing the difficulty section.
    #[error(transparent)]
    DifficultyParseError {
        #[from]
        source: difficulty::ParseError,
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
        source: timingpoints::ParseError,
    },
    /// Error parsing the colours section.
    #[error(transparent)]
    ColoursParseError {
        #[from]
        source: colours::ParseError,
    },
    /// Error parsing the hitobjects section.
    #[error(transparent)]
    HitObjectsParseError {
        #[from]
        source: hitobjects::ParseError,
    },
}
