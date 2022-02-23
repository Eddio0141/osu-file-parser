pub mod colours;
pub mod difficulty;
pub mod editor;
pub mod events;
pub mod general;
pub mod hitobject;
pub mod metadata;
pub mod section_error;
pub mod timingpoint;

// use core::hash::Hash;
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
use self::hitobject::HitObject;
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

/// An .osu file represented as a struct
pub struct OsuFile {
    version: u64,
    general: General,
    editor: Editor,
    metadata: Metadata,
    difficulty: Difficulty,
    events: Events,
    timing_points: Vec<TimingPoint>,
    colours: Colours,
    hitobjects: Vec<HitObject>,
}

impl FromStr for OsuFile {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let version_text = "osu file format v";

        let mut lines = s.lines();

        let file_version = match lines.next() {
            Some(version) => match version.strip_prefix(version_text) {
                Some(version) => match version.parse::<u64>() {
                    Ok(version) => version,
                    Err(_) => return Err(Box::new(OsuFileParseError::InvalidFileVersion)),
                },
                None => return Err(Box::new(OsuFileParseError::NoFileVersion)),
            },
            None => return Err(Box::new(OsuFileParseError::NoFileVersion)),
        };

        let s = lines.collect::<String>();

        // no defining more than 1 file version
        if s.find(version_text).is_some() {
            return Err(Box::new(OsuFileParseError::MultipleFileVersions));
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
            None => return Err(Box::new(OsuFileParseError::NoSectionsFound)),
        };

        if !has_unique_elements(&section_names) {
            return Err(Box::new(OsuFileParseError::DuplicateSectionNames));
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

        for (k, v) in section_map.iter() {
            match **k {
                "General" => general = v.parse()?,
                "Editor" => editor = v.parse()?,
                "Metadata" => metadata = v.parse()?,
                "Difficulty" => difficulty = v.parse()?,
                "Events" => events = v.parse()?,
                "TimingPoints" => {
                    timing_points = v
                        .lines()
                        .map(|line| line.parse::<TimingPoint>())
                        .collect::<Result<Vec<_>, _>>()?
                }
                "Colours" => colours = v.parse()?,
                "HitObjects" => {
                    hitobjects = v
                        .lines()
                        .map(|line| line.parse::<HitObject>())
                        .collect::<Result<Vec<_>, _>>()?
                }
                _ => return Err(Box::new(OsuFileParseError::InvalidSectionName)),
            }
        }

        Ok(OsuFile {
            version: file_version,
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
pub enum OsuFileParseError {
    InvalidFileVersion,
    NoFileVersion,
    MultipleFileVersions,
    NoSectionsFound,
    DuplicateSectionNames,
    InvalidSectionName,
}

impl Display for OsuFileParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_text = match self {
            OsuFileParseError::InvalidFileVersion => "Invalid file version",
            OsuFileParseError::NoFileVersion => "No file version defined",
            OsuFileParseError::MultipleFileVersions => "Multiple file versions defined",
            OsuFileParseError::NoSectionsFound => "No sections defined",
            OsuFileParseError::DuplicateSectionNames => "Duplicate sections defined",
            OsuFileParseError::InvalidSectionName => "Invalid section name",
        };

        write!(f, "{}", error_text)
    }
}

impl Error for OsuFileParseError {}

const DELIMITER: char = ':';

type Integer = i32;
type Decimal = f32;
