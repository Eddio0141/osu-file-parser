pub mod error;

use nom::{
    bytes::complete::{tag, take_until},
    multi::separated_list0,
    Parser,
};

use crate::parsers::get_colon_field_value_lines;

use super::{Error, Integer, Version};

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

impl Version for Metadata {
    type ParseError = Error<ParseError>;

    // TODO versions
    fn from_str_v3(s: &str) -> std::result::Result<Option<Self>, Self::ParseError>
    where
        Self: Sized,
    {
        let mut metadata = Metadata::default();

        let (s, fields) = get_colon_field_value_lines(s).unwrap();

        if !s.trim().is_empty() {
            // line count from fields
            let line_count = { fields.iter().map(|(_, _, ws)| ws.lines().count()).sum() };

            return Err(Error::new(ParseError::InvalidColonSet, line_count));
        }

        let tags = separated_list0(tag::<_, _, nom::error::Error<_>>(" "), take_until(" "))
            .map(|tags: Vec<&str>| tags.iter().map(|tag| tag.to_string()).collect());

        let mut line_count = 0;

        for (name, value, ws) in fields {
            let new_into_int = move |err| Error::new_into(err, line_count);

            match name {
                "Title" => metadata.title = Some(value.to_owned()),
                "TitleUnicode" => metadata.title_unicode = Some(value.to_owned()),
                "Artist" => metadata.artist = Some(value.to_owned()),
                "ArtistUnicode" => metadata.artist_unicode = Some(value.to_owned()),
                "Creator" => metadata.creator = Some(value.to_owned()),
                "Version" => metadata.version = Some(value.to_owned()),
                "Source" => metadata.source = Some(value.to_owned()),
                "Tags" => metadata.tags = Some(tags.parse(value).unwrap().1),
                "BeatmapID" => metadata.beatmap_id = Some(value.parse().map_err(new_into_int)?),
                "BeatmapSetID" => {
                    metadata.beatmap_set_id = Some(value.parse().map_err(new_into_int)?)
                }
                _ => return Err(Error::new(ParseError::InvalidKey, line_count)),
            }

            line_count += ws.lines().count();
        }

        Ok(Some(metadata))
    }

    fn to_string_v3(&self) -> String {
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

        key_value
            .iter()
            .filter_map(|(k, v)| v.as_ref().map(|v| format!("{k}:{v}")))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
