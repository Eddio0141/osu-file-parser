pub mod colours;
pub mod difficulty;
pub mod editor;
pub mod events;
pub mod general;
pub mod hitobject;
pub mod metadata;
pub mod section_error;
pub mod timingpoint;

use std::hash::Hash;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::Display,
    str::FromStr,
};

use regex::Regex;

use self::colours::Colours;
use self::difficulty::Difficulty;
use self::editor::Editor;
use self::events::Events;
use self::general::General;
use self::hitobject::{parse_hitobject, HitObjectWrapper};
use self::metadata::Metadata;
use self::section_error::SectionParseError;
use self::timingpoint::TimingPoint;

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

/// An .osu file represented as a struct
pub struct OsuFile {
    /// Version of the file format
    pub version: u64,
    /// General information about the beatmap
    /// - `key`: `value` pairs
    pub general: General,
    /// Saved settings for the beatmap editor
    /// - `key`: `value` pairs
    pub editor: Editor,
    /// Information used to identify the beatmap
    /// - `key`:`value` pairs
    pub metadata: Metadata,
    /// Difficulty settings
    /// - `key`:`value` pairs
    pub difficulty: Difficulty,
    /// Beatmap and storyboard graphic events
    /// Comma-separated lists
    pub events: Events,
    /// Timing and control points
    /// Comma-separated lists
    pub timing_points: Vec<TimingPoint>,
    /// Combo and skin colours
    /// `key` : `value` pairs
    pub colours: Colours,
    /// Hit objects
    /// Comma-separated lists
    pub hitobjects: Vec<HitObjectWrapper>,
}

impl FromStr for OsuFile {
    type Err = OsuFileParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let version_text = "osu file format v";

        let mut lines = s.lines();

        let version = match lines.next() {
            Some(version) => match version.strip_prefix(version_text) {
                Some(version) => match version.parse::<u64>() {
                    Ok(version) => version,
                    Err(_) => return Err(OsuFileParseError::InvalidFileVersion),
                },
                None => return Err(OsuFileParseError::NoFileVersion),
            },
            None => return Err(OsuFileParseError::NoFileVersion),
        };

        let s = lines.collect::<String>();

        // no defining more than 1 file version
        if s.contains(version_text) {
            return Err(OsuFileParseError::MultipleFileVersions);
        }

        let (section_open, section_close) = ('[', ']');

        let section_match = format!("\\{section_open}\\w*\\{section_close}[^{section_open}]*");
        let section_match = Regex::new(&section_match).unwrap();

        // split sections
        // TODO are sections required
        let (section_names, sections) = match section_match.captures(&s) {
            Some(section_match) => {
                let section_name_match = format!(
                    "(?!\\{section_open})[^\\{section_open}\\{section_close}]*(?={section_close})"
                );
                let section_name_match = Regex::new(&section_name_match).unwrap();

                let (names, sections): (Vec<_>, Vec<_>) = section_match
                    .iter()
                    .filter_map(|section| {
                        if let Some(section) = section {
                            let name = section_name_match
                                .captures(section.as_str().trim())
                                .unwrap()
                                .get(0)
                                .unwrap()
                                .as_str();

                            let section = section
                                .as_str()
                                .trim()
                                .strip_prefix(&format!("[{name}]"))
                                .unwrap();

                            Some((name, section))
                        } else {
                            None
                        }
                    })
                    .unzip();

                (names, sections)
            }
            None => return Err(OsuFileParseError::NoSectionsFound),
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
                    general = v.parse()?;
                    sections_to_include.remove(
                        sections_to_include
                            .iter()
                            .position(|section| *section == "General")
                            .unwrap(),
                    );
                }
                "Editor" => {
                    editor = v.parse()?;
                    sections_to_include.remove(
                        sections_to_include
                            .iter()
                            .position(|section| *section == "Editor")
                            .unwrap(),
                    );
                }
                "Metadata" => {
                    metadata = v.parse()?;
                    sections_to_include.remove(
                        sections_to_include
                            .iter()
                            .position(|section| *section == "Metadata")
                            .unwrap(),
                    );
                }
                "Difficulty" => {
                    difficulty = v.parse()?;
                    sections_to_include.remove(
                        sections_to_include
                            .iter()
                            .position(|section| *section == "Difficulty")
                            .unwrap(),
                    );
                }
                "Events" => {
                    events = v.parse()?;
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
                        .collect::<Result<Vec<_>, _>>()?;

                    sections_to_include.remove(
                        sections_to_include
                            .iter()
                            .position(|section| *section == "TimingPoints")
                            .unwrap(),
                    );
                }
                "Colours" => {
                    colours = v.parse()?;
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
                        .map(parse_hitobject)
                        .collect::<Result<Vec<_>, _>>()?;

                    sections_to_include.remove(
                        sections_to_include
                            .iter()
                            .position(|section| *section == "HitObjects")
                            .unwrap(),
                    );
                }
                _ => return Err(OsuFileParseError::InvalidSectionName),
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

#[derive(Debug)]
/// Error for when there's a problem parsing an .osu file
pub enum OsuFileParseError {
    /// File version is invalid
    InvalidFileVersion,
    /// No file version defined
    NoFileVersion,
    /// More than 1 file version defined
    MultipleFileVersions,
    /// File contains no sections
    NoSectionsFound,
    /// There are sections missing from the file
    MissingSections(Vec<String>),
    /// Duplicate section names defined
    DuplicateSections,
    /// Invalid section name defined
    InvalidSectionName,
    /// Error parsing a section
    SectionParseError { source: Box<dyn Error> },
}

impl From<SectionParseError> for OsuFileParseError {
    fn from(err: SectionParseError) -> Self {
        OsuFileParseError::SectionParseError {
            source: Box::new(err),
        }
    }
}

impl Display for OsuFileParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_text = match self {
            OsuFileParseError::InvalidFileVersion => "Invalid file version".to_string(),
            OsuFileParseError::NoFileVersion => "No file version defined".to_string(),
            OsuFileParseError::MultipleFileVersions => "Multiple file versions defined".to_string(),
            OsuFileParseError::NoSectionsFound => "No sections defined".to_string(),
            OsuFileParseError::DuplicateSections => "Duplicate sections defined".to_string(),
            OsuFileParseError::InvalidSectionName => "Invalid section name".to_string(),
            OsuFileParseError::SectionParseError { .. } => "Error parsing a section".to_string(),
            OsuFileParseError::MissingSections(sections) => {
                format!("Missing sections: {}", sections.join(", "))
            }
        };

        write!(f, "{}", error_text)
    }
}

impl Error for OsuFileParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            OsuFileParseError::SectionParseError { source } => Some(source.as_ref()),
            _ => None,
        }
    }
}

const DELIMITER: char = ':';

type Integer = i32;
