pub mod colours;
pub mod difficulty;
pub mod editor;
pub mod events;
pub mod general;
pub mod hitobjects;
pub mod metadata;
pub mod osb;
pub mod timingpoints;
pub mod types;

use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::str::FromStr;

use nom::bytes::complete::{tag, take_till};
use nom::character::complete::multispace0;
use nom::combinator::map_res;
use nom::multi::many0;
use nom::sequence::{preceded, tuple};
use thiserror::Error;

use crate::parsers::square_section;

pub use colours::Colours;
pub use difficulty::Difficulty;
pub use editor::Editor;
pub use events::Events;
pub use general::General;
pub use hitobjects::HitObjects;
pub use metadata::Metadata;
pub use osb::Osb;
pub use timingpoints::TimingPoints;

pub use types::*;

/// An .osu file represented as a struct.
#[derive(Clone, Debug, Hash, PartialEq)]
#[non_exhaustive]
pub struct OsuFile {
    /// Version of the file format.
    pub version: Version,
    /// General information about the beatmap.
    /// - `key`: `value` pairs.
    pub general: Option<General>,
    /// Saved settings for the beatmap editor.
    /// - `key`: `value` pairs.
    pub editor: Option<Editor>,
    /// Contents of an .osb storyboard file.
    pub osb: Option<Osb>,
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
    /// Spacing between each section.
    pub spacing: SectionSpacing,
}

impl OsuFile {
    /// New `OsuFile` with no data.
    pub fn new(version: Version) -> Self {
        Self {
            version,
            general: None,
            editor: None,
            metadata: None,
            difficulty: None,
            events: None,
            timing_points: None,
            colours: None,
            hitobjects: None,
            osb: None,
            spacing: SectionSpacing::default(),
        }
    }

    /// Appends .osb file.
    pub fn append_osb(&mut self, s: &str) -> Result<(), Error<osb::ParseError>> {
        self.osb = Osb::from_str(s, self.version)?;

        Ok(())
    }

    /// Generates .osb file contents.
    pub fn osb_to_string(&self) -> Option<String> {
        match &self.osb {
            Some(osb) => osb.to_string(self.version),
            None => None,
        }
    }

    pub fn default(version: Version) -> OsuFile {
        OsuFile::new(version)
    }
}

impl Display for OsuFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sections = Vec::new();

        if let Some(general) = &self.general {
            if let Some(general) = general.to_string(self.version) {
                sections.push(format!(
                    "[General]\n{general}{}",
                    "\n".repeat(self.spacing.general + 1)
                ))
            }
        }
        if let Some(editor) = &self.editor {
            if let Some(editor) = editor.to_string(self.version) {
                sections.push(format!(
                    "[Editor]\n{editor}{}",
                    "\n".repeat(self.spacing.editor + 1)
                ));
            }
        }
        if let Some(metadata) = &self.metadata {
            if let Some(metadata) = metadata.to_string(self.version) {
                sections.push(format!(
                    "[Metadata]\n{metadata}{}",
                    "\n".repeat(self.spacing.metadata + 1)
                ));
            }
        }
        if let Some(difficulty) = &self.difficulty {
            if let Some(difficulty) = difficulty.to_string(self.version) {
                sections.push(format!(
                    "[Difficulty]\n{difficulty}{}",
                    "\n".repeat(self.spacing.difficulty + 1)
                ));
            }
        }
        if let Some(events) = &self.events {
            if let Some(events) = events.to_string(self.version) {
                sections.push(format!(
                    "[Events]\n{events}{}",
                    "\n".repeat(self.spacing.events + 1)
                ));
            }
        }
        if let Some(timing_points) = &self.timing_points {
            if let Some(timing_points) = timing_points.to_string(self.version) {
                sections.push(format!(
                    "[TimingPoints]\n{timing_points}{}",
                    "\n".repeat(self.spacing.timing_points + 1)
                ));
            }
        }
        if let Some(colours) = &self.colours {
            if let Some(colours) = colours.to_string(self.version) {
                sections.push(format!(
                    "[Colours]\n{colours}{}",
                    "\n".repeat(self.spacing.colours + 1)
                ));
            }
        }
        if let Some(hitobjects) = &self.hitobjects {
            if let Some(hitobjects) = hitobjects.to_string(self.version) {
                sections.push(format!(
                    "[HitObjects]\n{hitobjects}{}",
                    "\n".repeat(self.spacing.hitobjects)
                ));
            }
        }

        write!(
            f,
            "osu file format v{}\n\n{}",
            self.version,
            sections.join("")
        )?;

        Ok(())
    }
}

impl FromStr for OsuFile {
    type Err = Error<ParseError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let version_text = tag::<_, _, nom::error::Error<_>>("osu file format v");
        let version_number = map_res(take_till(|c| c == '\r' || c == '\n'), |s: &str| s.parse());

        let (s, (trailing_ws, version)) = match tuple((
            multispace0,
            preceded(version_text, version_number),
        ))(s)
        {
            Ok(ok) => ok,
            Err(err) => {
                // wrong line?
                let err = if let nom::Err::Error(err) = err {
                    // can find out error by checking the error type
                    match err.code {
                        nom::error::ErrorKind::Tag => ParseError::FileVersionDefinedWrong,
                        nom::error::ErrorKind::MapRes => ParseError::InvalidFileVersion,
                        _ => {
                            unreachable!("Not possible to have the error kind {:#?}", err.code)
                        }
                    }
                } else {
                    unreachable!("Not possible to reach when the errors are already handled, error type is {:#?}", err)
                };

                return Err(err.into());
            }
        };

        if !(MIN_VERSION..=LATEST_VERSION).contains(&version) {
            return Err(ParseError::InvalidFileVersion.into());
        }

        let (_, sections) = many0(square_section())(s).unwrap();

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

        let mut spacing = SectionSpacing::default();

        let mut line_number = trailing_ws.lines().count();

        for (i, (ws, section_name, ws2, section)) in sections.iter().enumerate() {
            line_number += ws.lines().count();

            if section_parsed.contains(&section_name) {
                return Err(Error::new(ParseError::DuplicateSections, line_number));
            }

            let section_name_line = line_number;
            line_number += ws2.lines().count();

            // count empty lines in section from reverse
            let empty_line_count = || {
                let mut empty_lines = section
                    .chars()
                    .rev()
                    .take_while(|c| c.is_whitespace())
                    .filter(|c| *c == '\n')
                    .count();

                if let Some((ws, _, _, _)) = sections.get(i + 1) {
                    if ws.lines().count() == 0 {
                        empty_lines -= 1;
                    }
                }

                empty_lines
            };

            match *section_name {
                "General" => {
                    general =
                        Error::processing_line(General::from_str(section, version), line_number)?;
                    spacing.general = empty_line_count();
                }
                "Editor" => {
                    editor =
                        Error::processing_line(Editor::from_str(section, version), line_number)?;
                    spacing.editor = empty_line_count();
                }
                "Metadata" => {
                    metadata =
                        Error::processing_line(Metadata::from_str(section, version), line_number)?;
                    spacing.metadata = empty_line_count();
                }
                "Difficulty" => {
                    difficulty = Error::processing_line(
                        Difficulty::from_str(section, version),
                        line_number,
                    )?;
                    spacing.difficulty = empty_line_count();
                }
                "Events" => {
                    events =
                        Error::processing_line(Events::from_str(section, version), line_number)?;
                    spacing.events = empty_line_count();
                }
                "TimingPoints" => {
                    timing_points = Error::processing_line(
                        TimingPoints::from_str(section, version),
                        line_number,
                    )?;
                    spacing.timing_points = empty_line_count();
                }
                "Colours" => {
                    colours =
                        Error::processing_line(Colours::from_str(section, version), line_number)?;
                    spacing.colours = empty_line_count();
                }
                "HitObjects" => {
                    hitobjects = Error::processing_line(
                        HitObjects::from_str(section, version),
                        line_number,
                    )?;
                    spacing.hitobjects = empty_line_count();
                }
                _ => return Err(Error::new(ParseError::UnknownSection, section_name_line)),
            }

            section_parsed.push(section_name);
            line_number += section.lines().count() - 1;
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
            osb: None,
            spacing,
        })
    }
}

/// Contains information on how much new lines are in between sections.
#[derive(Clone, Debug, Hash, PartialEq)]
pub struct SectionSpacing {
    pub general: usize,
    pub editor: usize,
    pub metadata: usize,
    pub difficulty: usize,
    pub events: usize,
    pub timing_points: usize,
    pub colours: usize,
    pub hitobjects: usize,
}

impl Default for SectionSpacing {
    fn default() -> Self {
        Self {
            general: 1,
            editor: 1,
            metadata: 1,
            difficulty: 1,
            events: 1,
            timing_points: 1,
            colours: 1,
            hitobjects: 0,
        }
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
    #[error("File version defined wrong, expected `osu file format v..` at the start")]
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
    ParseGeneralError {
        #[from]
        source: general::ParseError,
    },
    /// Error parsing the editor section.
    #[error(transparent)]
    ParseEditorError {
        #[from]
        source: editor::ParseError,
    },
    /// Error parsing the metadata section.
    #[error(transparent)]
    ParseMetadataError {
        #[from]
        source: metadata::ParseError,
    },
    /// Error parsing the difficulty section.
    #[error(transparent)]
    ParseDifficultyError {
        #[from]
        source: difficulty::ParseError,
    },
    /// Error parsing the events section.
    #[error(transparent)]
    ParseEventsError {
        #[from]
        source: events::ParseError,
    },
    /// Error parsing the timingpoints section.
    #[error(transparent)]
    ParseTimingPointsError {
        #[from]
        source: timingpoints::ParseError,
    },
    /// Error parsing the colours section.
    #[error(transparent)]
    ParseColoursError {
        #[from]
        source: colours::ParseError,
    },
    /// Error parsing the hitobjects section.
    #[error(transparent)]
    ParseHitObjectsError {
        #[from]
        source: hitobjects::ParseError,
    },
}
