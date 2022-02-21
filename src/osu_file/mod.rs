pub mod general;

use std::{error::Error, fmt::Display, str::FromStr};

use self::general::General;

/// An .osu file represented as a struct
pub struct OsuFile {
    version: u64,
    general: General,
    editor: Editor,
    metadata: Metadata,
    difficulty: Difficulty,
    events: Events,
    timing_points: TimingPoints,
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

        // split sections
        

        Ok(OsuFile {
            version: file_version,
            general: todo!(),
            editor: todo!(),
            metadata: todo!(),
            difficulty: todo!(),
            events: todo!(),
            timing_points: todo!(),
            colours: todo!(),
            hitobjects: todo!(),
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
}

impl Display for OsuFileParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_text = match self {
            OsuFileParseError::InvalidFileVersion => "Invalid file version",
            OsuFileParseError::NoFileVersion => "No file version defined",
            OsuFileParseError::MultipleFileVersions => "Multiple file versions defined",
        };

        write!(f, "{}", error_text)
    }
}

impl Error for OsuFileParseError {}

const DELIMITER: char = ':';

type Integer = i32;
type Decimal = f32;

#[derive(Default)]
pub struct Editor;

#[derive(Default)]
pub struct Metadata;

#[derive(Default)]
pub struct Difficulty;

#[derive(Default)]
pub struct Events;

#[derive(Default)]
pub struct TimingPoints;

#[derive(Default)]
pub struct Colours;

#[derive(Default)]
pub struct HitObject;
