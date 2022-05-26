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
    pub version: u8,
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
            let general = match &self.version {
                3 => general.to_string_v3(),
                4 => general.to_string_v4(),
                5 => general.to_string_v5(),
                6 => general.to_string_v6(),
                7 => general.to_string_v7(),
                8 => general.to_string_v8(),
                9 => general.to_string_v9(),
                10 => general.to_string_v10(),
                11 => general.to_string_v11(),
                12 => general.to_string_v12(),
                13 => general.to_string_v13(),
                14 => general.to_string_v14(),
                _ => unreachable!(),
            };

            if let Some(general) = general {
                sections.push(format!("[General]\n{general}",))
            }
        }
        if let Some(editor) = &self.editor {
            let editor = match &self.version {
                3 => editor.to_string_v3(),
                4 => editor.to_string_v4(),
                5 => editor.to_string_v5(),
                6 => editor.to_string_v6(),
                7 => editor.to_string_v7(),
                8 => editor.to_string_v8(),
                9 => editor.to_string_v9(),
                10 => editor.to_string_v10(),
                11 => editor.to_string_v11(),
                12 => editor.to_string_v12(),
                13 => editor.to_string_v13(),
                14 => editor.to_string_v14(),
                _ => unreachable!(),
            };

            if let Some(editor) = editor {
                sections.push(format!("[Editor]\n{editor}"));
            }
        }
        if let Some(metadata) = &self.metadata {
            let metadata = match &self.version {
                3 => metadata.to_string_v3(),
                4 => metadata.to_string_v4(),
                5 => metadata.to_string_v5(),
                6 => metadata.to_string_v6(),
                7 => metadata.to_string_v7(),
                8 => metadata.to_string_v8(),
                9 => metadata.to_string_v9(),
                10 => metadata.to_string_v10(),
                11 => metadata.to_string_v11(),
                12 => metadata.to_string_v12(),
                13 => metadata.to_string_v13(),
                14 => metadata.to_string_v14(),
                _ => unreachable!(),
            };

            if let Some(metadata) = metadata {
                sections.push(format!("[Metadata]\n{metadata}"));
            }
        }
        if let Some(difficulty) = &self.difficulty {
            let difficulty = match &self.version {
                3 => difficulty.to_string_v3(),
                4 => difficulty.to_string_v4(),
                5 => difficulty.to_string_v5(),
                6 => difficulty.to_string_v6(),
                7 => difficulty.to_string_v7(),
                8 => difficulty.to_string_v8(),
                9 => difficulty.to_string_v9(),
                10 => difficulty.to_string_v10(),
                11 => difficulty.to_string_v11(),
                12 => difficulty.to_string_v12(),
                13 => difficulty.to_string_v13(),
                14 => difficulty.to_string_v14(),
                _ => unreachable!(),
            };

            if let Some(difficulty) = difficulty {
                sections.push(format!("[Difficulty]\n{difficulty}"));
            }
        }
        if let Some(events) = &self.events {
            let events = match &self.version {
                3 => events.to_string_v3(),
                4 => events.to_string_v4(),
                5 => events.to_string_v5(),
                6 => events.to_string_v6(),
                7 => events.to_string_v7(),
                8 => events.to_string_v8(),
                9 => events.to_string_v9(),
                10 => events.to_string_v10(),
                11 => events.to_string_v11(),
                12 => events.to_string_v12(),
                13 => events.to_string_v13(),
                14 => events.to_string_v14(),
                _ => unreachable!(),
            };

            if let Some(events) = events {
                sections.push(format!("[Events]\n{events}"));
            }
        }
        if let Some(timing_points) = &self.timing_points {
            let timing_points = match &self.version {
                3 => timing_points.to_string_v3(),
                4 => timing_points.to_string_v4(),
                5 => timing_points.to_string_v5(),
                6 => timing_points.to_string_v6(),
                7 => timing_points.to_string_v7(),
                8 => timing_points.to_string_v8(),
                9 => timing_points.to_string_v9(),
                10 => timing_points.to_string_v10(),
                11 => timing_points.to_string_v11(),
                12 => timing_points.to_string_v12(),
                13 => timing_points.to_string_v13(),
                14 => timing_points.to_string_v14(),
                _ => unreachable!(),
            };

            if let Some(timing_points) = timing_points {
                let section = format!("[TimingPoints]\n{timing_points}");

                // for some reason below v14, theres an extra new line at the end
                if self.version < 14 {
                    sections.push(format!("{section}\n"));
                } else {
                    sections.push(section);
                }
            }
        }
        if let Some(colours) = &self.colours {
            let colours = match &self.version {
                3 => colours.to_string_v3(),
                4 => colours.to_string_v4(),
                5 => colours.to_string_v5(),
                6 => colours.to_string_v6(),
                7 => colours.to_string_v7(),
                8 => colours.to_string_v8(),
                9 => colours.to_string_v9(),
                10 => colours.to_string_v10(),
                11 => colours.to_string_v11(),
                12 => colours.to_string_v12(),
                13 => colours.to_string_v13(),
                14 => colours.to_string_v14(),
                _ => unreachable!(),
            };

            if let Some(colours) = colours {
                sections.push(format!("[Colours]\n{colours}"));
            }
        }
        if let Some(hitobjects) = &self.hitobjects {
            let hitobjects = match &self.version {
                3 => hitobjects.to_string_v3(),
                4 => hitobjects.to_string_v4(),
                5 => hitobjects.to_string_v5(),
                6 => hitobjects.to_string_v6(),
                7 => hitobjects.to_string_v7(),
                8 => hitobjects.to_string_v8(),
                9 => hitobjects.to_string_v9(),
                10 => hitobjects.to_string_v10(),
                11 => hitobjects.to_string_v11(),
                12 => hitobjects.to_string_v12(),
                13 => hitobjects.to_string_v13(),
                14 => hitobjects.to_string_v14(),
                _ => unreachable!(),
            };

            if let Some(hitobjects) = hitobjects {
                sections.push(format!("[HitObjects]\n{hitobjects}"));
            }
        }

        write!(f, "{}", sections.join("\n\n"))?;
        // for some reason below v14 theres another new line at the end
        if self.version < 14 {
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl FromStr for OsuFile {
    type Err = Error<ParseError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let version_text = tag::<_, _, nom::error::Error<_>>("osu file format v");
        let version_number = map_res(take_till(|c| c == '\r' || c == '\n'), |s: &str| {
            s.parse::<u8>()
        });

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
                    general = Error::processing_line(
                        match version {
                            14 => General::from_str_v14(section),
                            13 => General::from_str_v13(section),
                            12 => General::from_str_v12(section),
                            11 => General::from_str_v11(section),
                            10 => General::from_str_v10(section),
                            9 => General::from_str_v9(section),
                            8 => General::from_str_v8(section),
                            7 => General::from_str_v7(section),
                            6 => General::from_str_v6(section),
                            5 => General::from_str_v5(section),
                            4 => General::from_str_v4(section),
                            3 => General::from_str_v3(section),
                            _ => unreachable!("version {} not implemented", version),
                        },
                        line_number,
                    )?
                }
                "Editor" => {
                    editor = Error::processing_line(
                        match version {
                            14 => Editor::from_str_v14(section),
                            13 => Editor::from_str_v13(section),
                            12 => Editor::from_str_v12(section),
                            11 => Editor::from_str_v11(section),
                            10 => Editor::from_str_v10(section),
                            9 => Editor::from_str_v9(section),
                            8 => Editor::from_str_v8(section),
                            7 => Editor::from_str_v7(section),
                            6 => Editor::from_str_v6(section),
                            5 => Editor::from_str_v5(section),
                            4 => Editor::from_str_v4(section),
                            3 => Editor::from_str_v3(section),
                            _ => unreachable!("version {} not implemented", version),
                        },
                        line_number,
                    )?
                }
                "Metadata" => {
                    metadata = Error::processing_line(
                        match version {
                            14 => Metadata::from_str_v14(section),
                            13 => Metadata::from_str_v13(section),
                            12 => Metadata::from_str_v12(section),
                            11 => Metadata::from_str_v11(section),
                            10 => Metadata::from_str_v10(section),
                            9 => Metadata::from_str_v9(section),
                            8 => Metadata::from_str_v8(section),
                            7 => Metadata::from_str_v7(section),
                            6 => Metadata::from_str_v6(section),
                            5 => Metadata::from_str_v5(section),
                            4 => Metadata::from_str_v4(section),
                            3 => Metadata::from_str_v3(section),
                            _ => unreachable!("version {} not implemented", version),
                        },
                        line_number,
                    )?
                }
                "Difficulty" => {
                    difficulty = Error::processing_line(
                        match version {
                            14 => Difficulty::from_str_v14(section),
                            13 => Difficulty::from_str_v13(section),
                            12 => Difficulty::from_str_v12(section),
                            11 => Difficulty::from_str_v11(section),
                            10 => Difficulty::from_str_v10(section),
                            9 => Difficulty::from_str_v9(section),
                            8 => Difficulty::from_str_v8(section),
                            7 => Difficulty::from_str_v7(section),
                            6 => Difficulty::from_str_v6(section),
                            5 => Difficulty::from_str_v5(section),
                            4 => Difficulty::from_str_v4(section),
                            3 => Difficulty::from_str_v3(section),
                            _ => unreachable!("version {} not implemented", version),
                        },
                        line_number,
                    )?
                }
                "Events" => {
                    events = Error::processing_line(
                        match version {
                            14 => Events::from_str_v14(section),
                            13 => Events::from_str_v13(section),
                            12 => Events::from_str_v12(section),
                            11 => Events::from_str_v11(section),
                            10 => Events::from_str_v10(section),
                            9 => Events::from_str_v9(section),
                            8 => Events::from_str_v8(section),
                            7 => Events::from_str_v7(section),
                            6 => Events::from_str_v6(section),
                            5 => Events::from_str_v5(section),
                            4 => Events::from_str_v4(section),
                            3 => Events::from_str_v3(section),
                            _ => unreachable!("version {} not implemented", version),
                        },
                        line_number,
                    )?
                }
                "TimingPoints" => {
                    timing_points = Error::processing_line(
                        match version {
                            14 => TimingPoints::from_str_v14(section),
                            13 => TimingPoints::from_str_v13(section),
                            12 => TimingPoints::from_str_v12(section),
                            11 => TimingPoints::from_str_v11(section),
                            10 => TimingPoints::from_str_v10(section),
                            9 => TimingPoints::from_str_v9(section),
                            8 => TimingPoints::from_str_v8(section),
                            7 => TimingPoints::from_str_v7(section),
                            6 => TimingPoints::from_str_v6(section),
                            5 => TimingPoints::from_str_v5(section),
                            4 => TimingPoints::from_str_v4(section),
                            3 => TimingPoints::from_str_v3(section),
                            _ => unreachable!("version {} not implemented", version),
                        },
                        line_number,
                    )?
                }
                "Colours" => {
                    colours = Error::processing_line(
                        match version {
                            14 => Colours::from_str_v14(section),
                            13 => Colours::from_str_v13(section),
                            12 => Colours::from_str_v12(section),
                            11 => Colours::from_str_v11(section),
                            10 => Colours::from_str_v10(section),
                            9 => Colours::from_str_v9(section),
                            8 => Colours::from_str_v8(section),
                            7 => Colours::from_str_v7(section),
                            6 => Colours::from_str_v6(section),
                            5 => Colours::from_str_v5(section),
                            4 => Colours::from_str_v4(section),
                            3 => Colours::from_str_v3(section),
                            _ => unreachable!("version {} not implemented", version),
                        },
                        line_number,
                    )?
                }
                "HitObjects" => {
                    hitobjects = Error::processing_line(
                        match version {
                            14 => HitObjects::from_str_v14(section),
                            13 => HitObjects::from_str_v13(section),
                            12 => HitObjects::from_str_v12(section),
                            11 => HitObjects::from_str_v11(section),
                            10 => HitObjects::from_str_v10(section),
                            9 => HitObjects::from_str_v9(section),
                            8 => HitObjects::from_str_v8(section),
                            7 => HitObjects::from_str_v7(section),
                            6 => HitObjects::from_str_v6(section),
                            5 => HitObjects::from_str_v5(section),
                            4 => HitObjects::from_str_v4(section),
                            3 => HitObjects::from_str_v3(section),
                            _ => unreachable!("version {} not implemented", version),
                        },
                        line_number,
                    )?
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
