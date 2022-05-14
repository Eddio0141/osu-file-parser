pub mod error;

use std::fmt::Display;
use std::str::FromStr;

use super::{Integer, SECTION_DELIMITER};

pub use self::error::*;

/// A struct representing the metadata section of the .osu file.
#[derive(Default, Clone, Hash, PartialEq, Eq, Debug)]
pub struct Metadata {
    /// Romanised song title.
    pub title: Option<String>,
    /// Song title.
    pub title_unicode: Option<String>,
    /// ROmanised song artist.
    pub artist: Option<String>,
    /// Song artist.
    pub artist_unicode: Option<String>,
    /// Beatmap creator.
    pub creator: Option<String>,
    /// Difficulty name.
    pub version: Option<String>,
    /// Original media the song was produced for.
    pub source: Option<String>,
    /// Search terms.
    pub tags: Option<Vec<String>>,
    /// Difficulty ID.
    pub beatmap_id: Option<Integer>,
    /// Beatmap ID.
    pub beatmap_set_id: Option<Integer>,
}

impl FromStr for Metadata {
    type Err = MetadataParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut metadata = Metadata::default();

        let s = s.trim();

        for line in s.lines() {
            match line.split_once(SECTION_DELIMITER) {
                Some((key, value)) => match key.trim() {
                    "Title" => metadata.title = Some(value.to_owned()),
                    "TitleUnicode" => metadata.title_unicode = Some(value.to_owned()),
                    "Artist" => metadata.artist = Some(value.to_owned()),
                    "ArtistUnicode" => metadata.artist_unicode = Some(value.to_owned()),
                    "Creator" => metadata.creator = Some(value.to_owned()),
                    "Version" => metadata.version = Some(value.to_owned()),
                    "Source" => metadata.source = Some(value.to_owned()),
                    "Tags" => {
                        metadata.tags = Some(
                            value
                                .split_whitespace()
                                .map(|value| value.to_string())
                                .collect(),
                        )
                    }
                    "BeatmapID" => {
                        metadata.beatmap_id = Some(value.parse().map_err(|err| {
                            MetadataParseError::SectionParseError {
                                source: err,
                                name: "BeatmapID",
                            }
                        })?)
                    }
                    "BeatmapSetID" => {
                        metadata.beatmap_set_id = Some(value.parse().map_err(|err| {
                            MetadataParseError::SectionParseError {
                                source: err,
                                name: "BeatmapSetID",
                            }
                        })?)
                    }
                    _ => return Err(MetadataParseError::InvalidKey(line.to_string())),
                },
                None => return Err(MetadataParseError::MissingValue(line.to_owned())),
            }
        }

        Ok(metadata)
    }
}

impl Display for Metadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut key_value = Vec::new();

        key_value.push(("Title", &self.title));
        key_value.push(("TitleUnicode", &self.title));
        key_value.push(("Artist", &self.artist));
        key_value.push(("ArtistUnicode", &self.artist_unicode));
        key_value.push(("Creator", &self.creator));
        key_value.push(("Version", &self.version));
        key_value.push(("Source", &self.source));
        let tags = self.tags.as_ref().map(|v| v.join(" "));
        key_value.push(("Tags", &tags));
        let beatmap_id = self.beatmap_id.map(|v| v.to_string());
        key_value.push(("BeatmapID", &beatmap_id));
        let beatmap_set_id = self.beatmap_set_id.map(|v| v.to_string());
        key_value.push(("BeatmapSetID", &beatmap_set_id));

        write!(
            f,
            "{}",
            key_value
                .iter()
                .filter_map(|(k, v)| if let Some(v) = v {
                    Some(format!("{k}:{v}"))
                } else {
                    None
                })
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}
