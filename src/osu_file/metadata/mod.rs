pub mod error;

use nom::{
    bytes::complete::{tag, take_till},
    multi::separated_list0,
    Parser,
};

use crate::parsers::get_colon_field_value_lines;

use super::{Error, Integer};
use crate::helper::macros::*;
use crate::osu_file::types::Version;

pub use self::error::*;

versioned_field!(Title, String, no_versions, |s| { Ok(s.to_string()) } -> (),,);
versioned_field!(TitleUnicode, String, no_versions, |s| { Ok(s.to_string()) } -> (),,);
versioned_field!(Artist, String, no_versions, |s| { Ok(s.to_string()) } -> (),,);
versioned_field!(ArtistUnicode, String, no_versions, |s| { Ok(s.to_string()) } -> (),,);
versioned_field!(Creator, String, no_versions, |s| { Ok(s.to_string()) } -> (),,);
versioned_field!(VersionName, String, no_versions, |s| { Ok(s.to_string()) } -> (),,);
versioned_field!(Source, String, no_versions, |s| { Ok(s.to_string()) } -> (),,);
versioned_field!(Tags, Vec<String>, no_versions,
    |s| {
        let mut space_separated_list = separated_list0(
            tag::<_, _, nom::error::Error<_>>(" "),
            take_till(|c| c == ' '),
        )
        .map(|tags: Vec<&str>| tags.iter().map(|tag| tag.to_string()).collect());

        Ok(space_separated_list.parse(s).unwrap().1)
    } -> (),
    |v| { v.join(" ") }, Vec::new()
);
versioned_field!(BeatmapID, Integer, no_versions, |s| { Ok(s.parse::<Integer>().unwrap()) } -> (),,);
versioned_field!(BeatmapSetID, Integer, no_versions, |s| { Ok(s.parse::<Integer>().unwrap()) } -> (),,);

general_section!(
    /// A struct representing the metadata section of an osu file.
    pub struct Metadata {
        /// Romanised song title.
        pub title: Title,
        /// Song title.
        pub title_unicode: TitleUnicode,
        /// ROmanised song artist.
        pub artist: Artist,
        /// Song artist.
        pub artist_unicode: ArtistUnicode,
        /// Beatmap creator.
        pub creator: Creator,
        /// Difficulty name.
        pub version: VersionName,
        /// Original media the song was produced for.
        pub source: Source,
        /// Search terms.
        pub tags: Tags,
        /// Difficulty ID.
        pub beatmap_id: BeatmapID,
        /// Beatmap ID.
        pub beatmap_set_id: BeatmapSetID,
    },
    ParseError
);
