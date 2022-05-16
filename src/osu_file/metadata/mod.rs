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
        let tags = self.tags.as_ref().map(|v| v.join(" "));
        let beatmap_id = self.beatmap_id.map(|v| v.to_string());
        let beatmap_set_id = self.beatmap_set_id.map(|v| v.to_string());

        let key_value = vec![
            ("Title", &self.title),
            ("TitleUnicode", &self.title),
            ("Artist", &self.artist),
            ("ArtistUnicode", &self.artist_unicode),
            ("Creator", &self.creator),
            ("Version", &self.version),
            ("Source", &self.source),
            ("Tags", &tags),
            ("BeatmapID", &beatmap_id),
            ("BeatmapSetID", &beatmap_set_id),
        ];

        write!(
            f,
            "{}",
            key_value
                .iter()
                .filter_map(|(k, v)| v.as_ref().map(|v| format!("{k}:{v}")))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}
