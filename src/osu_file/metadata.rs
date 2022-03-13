use std::fmt::Display;
use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

use super::{Integer, SECTION_DELIMITER};

/// A struct representing the metadata section of the .osu file.
#[derive(Default, Clone, Hash, PartialEq, Eq, Debug)]
pub struct Metadata {
    /// Romanised song title.
    pub title: String,
    /// Song title.
    pub title_unicode: String,
    /// ROmanised song artist.
    pub artist: String,
    /// Song artist.
    pub artist_unicode: String,
    /// Beatmap creator.
    pub creator: String,
    /// Difficulty name.
    pub version: String,
    /// Original media the song was produced for.
    pub source: String,
    /// Search terms.
    pub tags: Vec<String>,
    /// Difficulty ID.
    pub beatmap_id: Integer,
    /// Beatmap ID.
    pub beatmap_set_id: Integer,
}

impl FromStr for Metadata {
    type Err = MetadataParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut metadata = Metadata::default();

        let s = s.trim();

        for line in s.lines() {
            match line.split_once(SECTION_DELIMITER) {
                Some((key, value)) => {
                    match key.trim() {
                        "Title" => metadata.title = value.to_owned(),
                        "TitleUnicode" => metadata.title_unicode = value.to_owned(),
                        "Artist" => metadata.artist = value.to_owned(),
                        "ArtistUnicode" => metadata.artist_unicode = value.to_owned(),
                        "Creator" => metadata.creator = value.to_owned(),
                        "Version" => metadata.version = value.to_owned(),
                        "Source" => metadata.source = value.to_owned(),
                        "Tags" => {
                            metadata.tags = value
                                .split_whitespace()
                                .map(|value| value.to_string())
                                .collect()
                        }
                        "BeatmapID" => {
                            metadata.beatmap_id = value.parse().map_err(|err| {
                                MetadataParseError::SectionParseError {
                                    source: err,
                                    name: "BeatmapID",
                                }
                            })?
                        }
                        "BeatmapSetID" => {
                            metadata.beatmap_set_id = value.parse().map_err(|err| {
                                MetadataParseError::SectionParseError {
                                    source: err,
                                    name: "BeatmapSetID",
                                }
                            })?
                        }
                        _ => return Err(MetadataParseError::InvalidKey(line.to_string())),
                    }
                }
                None => return Err(MetadataParseError::MissingValue(line.to_owned())),
            }
        }

        Ok(metadata)
    }
}

impl Display for Metadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut key_value = Vec::new();

        key_value.push(format!("Title:{}", self.title));
        key_value.push(format!("TitleUnicode:{}", self.title));
        key_value.push(format!("Artist:{}", self.artist));
        key_value.push(format!("ArtistUnicode:{}", self.artist_unicode));
        key_value.push(format!("Creator:{}", self.creator));
        key_value.push(format!("Version:{}", self.version));
        key_value.push(format!("Source:{}", self.source));
        key_value.push(format!("Tags:{}", self.tags.join(" ")));
        key_value.push(format!("BeatmapID:{}", self.beatmap_id));
        key_value.push(format!("BeatmapSetID:{}", self.beatmap_set_id));

        write!(f, "{}", key_value.join("\r\n"))
    }
}

#[derive(Debug, Error)]
/// Error used when there was a problem parsing the `Metadata` section.
pub enum MetadataParseError {
    #[error("There was a problem parsing the `{name}` property from a `str` to an `Integer`")]
    /// A section in `Metadata` failed to parse.
    SectionParseError {
        #[source]
        source: ParseIntError,
        name: &'static str,
    },
    #[error("The key {0} doesn't exist in `Metadata`")]
    /// Invalid key name was used.
    InvalidKey(String),
    #[error("The key {0} has no value set")]
    /// The value is missing from the `key:value` set.
    MissingValue(String),
}
