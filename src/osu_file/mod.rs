pub mod colours;
pub mod difficulty;
pub mod editor;
pub mod events;
pub mod general;
pub mod hitobject;
pub mod metadata;
pub mod timingpoint;

use std::hash::Hash;
use std::num::ParseIntError;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    str::FromStr,
};

use regex::Regex;
use thiserror::Error;

use self::colours::Colours;
use self::difficulty::Difficulty;
use self::editor::Editor;
use self::events::Events;
use self::general::General;
use self::hitobject::{try_parse_hitobject, HitObjectWrapper};
use self::metadata::Metadata;

use self::timingpoint::TimingPoint;

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

/// An .osu file represented as a struct.
pub struct OsuFile {
    /// Version of the file format.
    pub version: Integer,
    /// General information about the beatmap.
    /// - `key`: `value` pairs.
    pub general: General,
    /// Saved settings for the beatmap editor.
    /// - `key`: `value` pairs.
    pub editor: Editor,
    /// Information used to identify the beatmap.
    /// - `key`:`value` pairs.
    pub metadata: Metadata,
    /// Difficulty settings.
    /// - `key`:`value` pairs.
    pub difficulty: Difficulty,
    /// Beatmap and storyboard graphic events.
    /// Comma-separated lists.
    pub events: Events,
    /// Timing and control points.
    /// Comma-separated lists.
    pub timing_points: Vec<TimingPoint>,
    /// Combo and skin colours.
    /// `key` : `value` pairs.
    pub colours: Colours,
    /// Hit objects.
    /// Comma-separated lists.
    pub hitobjects: Vec<HitObjectWrapper>,
}

impl FromStr for OsuFile {
    type Err = OsuFileParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let version_text = "osu file format v";

        let mut lines = s.lines().peekable();

        let version = match lines.next() {
            Some(version) => match version.trim().strip_prefix(version_text) {
                Some(version) => match version.parse() {
                    Ok(version) => {
                        if version > LATEST_VERSION {
                            return Err(OsuFileParseError::InvalidFileVersion(version));
                        } else {
                            version
                        }
                    }
                    Err(err) => {
                        return Err(OsuFileParseError::FileVersionParseError {
                            source: err,
                            value: version.to_string(),
                        })
                    }
                },
                None => {
                    // check what type of error it is
                    let mut version_index = None;

                    for (i, line) in s.lines().enumerate() {
                        if line.trim().starts_with(version_text) {
                            version_index = Some(i);
                            break;
                        }
                    }

                    match version_index {
                        Some(version_index) => {
                            return Err(OsuFileParseError::FileVersionInWrongLine(version_index))
                        }
                        None => return Err(OsuFileParseError::NoFileVersion),
                    }
                }
            },
            None => return Err(OsuFileParseError::NoFileVersion),
        };

        let mut lines_no_version = lines.clone();

        if let Some(version_index) =
            lines_no_version.position(|s| s.trim().starts_with(version_text))
        {
            let mut all_version_defs = lines_no_version
                .enumerate()
                .filter_map(|(i, s)| {
                    if s.trim().starts_with(version_text) {
                        Some(i.to_string())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            all_version_defs.insert(0, version_index.to_string());

            return Err(OsuFileParseError::MultipleFileVersions(format!(
                "lines {}",
                all_version_defs.join(", ")
            )));
        }

        let (section_names, sections) = {
            let mut section_names = Vec::new();
            let mut sections = Vec::new();
    
            let mut in_section = false;
    
            for line in lines {
                if in_section {
                    sections.push(line);
    
                    if let Some(next_line) = lines.peek() {
                        if next_line.trim().starts_with(SECTION_OPEN) {
                            in_section = false;
                        }
                    }
                } else {
                    let line = line.trim();
    
                    if line.starts_with(SECTION_OPEN) {
                        if line.ends_with(SECTION_CLOSE) {
                            section_names.push(&line[1..section_names.len()]);
                        } else {
                            return Err(OsuFileParseError::SectionNameNoCloseBracket(
                                line.to_string(),
                            ));
                        }
                    } else {
                        return Err(OsuFileParseError::SectionNameNoOpenBracket(
                            line.to_string(),
                        ));
                    }
    
                    in_section = true;
                }
            }

            (section_names, sections)
        };

        if !has_unique_elements(&section_names) {
            return Err(OsuFileParseError::DuplicateSections);
        }

        let section_map: HashMap<_, _> =
            HashMap::from_iter(section_names.iter().zip(sections.iter()));

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

        // maybe temporary
        // used to check what sections are missing
        let mut sections_to_include = vec![
            "General",
            "Editor",
            "Metadata",
            "Difficulty",
            "Events",
            "TimingPoints",
            "Colours",
            "HitObjects",
        ];

        // TODO clean this up
        for (k, v) in section_map.iter() {
            match **k {
                "General" => {
                    general = v
                        .parse()
                        .map_err(|err| OsuFileParseError::SectionParseError {
                            source: Box::new(err),
                        })?;
                    sections_to_include.remove(
                        sections_to_include
                            .iter()
                            .position(|section| *section == "General")
                            .unwrap(),
                    );
                }
                "Editor" => {
                    editor = v
                        .parse()
                        .map_err(|err| OsuFileParseError::SectionParseError {
                            source: Box::new(err),
                        })?;
                    sections_to_include.remove(
                        sections_to_include
                            .iter()
                            .position(|section| *section == "Editor")
                            .unwrap(),
                    );
                }
                "Metadata" => {
                    metadata = v
                        .parse()
                        .map_err(|err| OsuFileParseError::SectionParseError {
                            source: Box::new(err),
                        })?;
                    sections_to_include.remove(
                        sections_to_include
                            .iter()
                            .position(|section| *section == "Metadata")
                            .unwrap(),
                    );
                }
                "Difficulty" => {
                    difficulty = v
                        .parse()
                        .map_err(|err| OsuFileParseError::SectionParseError {
                            source: Box::new(err),
                        })?;
                    sections_to_include.remove(
                        sections_to_include
                            .iter()
                            .position(|section| *section == "Difficulty")
                            .unwrap(),
                    );
                }
                "Events" => {
                    events = v
                        .parse()
                        .map_err(|err| OsuFileParseError::SectionParseError {
                            source: Box::new(err),
                        })?;
                    sections_to_include.remove(
                        sections_to_include
                            .iter()
                            .position(|section| *section == "Events")
                            .unwrap(),
                    );
                }
                "TimingPoints" => {
                    timing_points = v
                        .lines()
                        .map(|line| line.parse::<TimingPoint>())
                        .collect::<Result<Vec<_>, _>>()
                        .map_err(|err| OsuFileParseError::SectionParseError {
                            source: Box::new(err),
                        })?;

                    sections_to_include.remove(
                        sections_to_include
                            .iter()
                            .position(|section| *section == "TimingPoints")
                            .unwrap(),
                    );
                }
                "Colours" => {
                    colours = v
                        .parse()
                        .map_err(|err| OsuFileParseError::SectionParseError {
                            source: Box::new(err),
                        })?;
                    sections_to_include.remove(
                        sections_to_include
                            .iter()
                            .position(|section| *section == "Colours")
                            .unwrap(),
                    );
                }
                "HitObjects" => {
                    hitobjects = v
                        .lines()
                        .map(try_parse_hitobject)
                        .collect::<Result<Vec<_>, _>>()
                        .map_err(|err| OsuFileParseError::SectionParseError {
                            source: Box::new(err),
                        })?;

                    sections_to_include.remove(
                        sections_to_include
                            .iter()
                            .position(|section| *section == "HitObjects")
                            .unwrap(),
                    );
                }
                _ => return Err(OsuFileParseError::InvalidSectionName(k.to_string())),
            }
        }

        if !sections_to_include.is_empty() {
            return Err(OsuFileParseError::MissingSections(
                sections_to_include.iter().map(|s| s.to_string()).collect(),
            ));
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
            version: 14,
            general: Default::default(),
            editor: Default::default(),
            metadata: Default::default(),
            difficulty: Default::default(),
            events: Default::default(),
            timing_points: Default::default(),
            colours: Default::default(),
            hitobjects: Default::default(),
        }
    }
}

// TODO refine error

#[derive(Debug, Error)]
/// Error for when there's a problem parsing an .osu file.
pub enum OsuFileParseError {
    /// File version is invalid.
    #[error("Invalid file version, expected {LATEST_VERSION}, got {0}")]
    InvalidFileVersion(Integer),
    /// File version parsing failed
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
    /// File contains no sections.
    #[error("No sections found")]
    NoSectionsFound,
    /// There are sections missing from the file.
    #[error("Missing sections: {0}")]
    MissingSections(String),
    /// Duplicate section names defined.
    #[error("There are duplicate sections: {0}")]
    DuplicateSections(String),
    /// Invalid section name defined.
    #[error("There is an invalid section name `{0}`")]
    InvalidSectionName(String),
    /// Error parsing a section.
    #[error(transparent)]
    SectionParseError {
        #[from]
        source: Box<dyn Error>,
    },
    /// Error used when the opening bracket for the section is missing.
    #[error("The opening bracket of the section is missing, expected `{SECTION_OPEN}` before {0}")]
    SectionNameNoOpenBracket(String),
    /// Error used when the closing bracket for the section is missing.
    #[error("The closing bracket of the section is missing, expected `{SECTION_CLOSE}` after {0}")]
    SectionNameNoCloseBracket(String),
}

/// Latest file version.
const LATEST_VERSION: Integer = 14;

/// Delimiter for the `key: value` pair.
const SECTION_DELIMITER: char = ':';
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
