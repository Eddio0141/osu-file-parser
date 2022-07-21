pub mod error;

use std::num::{IntErrorKind, ParseIntError};

use nom::{bytes::complete::take_till, combinator::map_res, multi::separated_list0, Finish};
use crate::osu_file::types::Decimal;

use crate::parsers::comma;

use super::Integer;
use crate::helper::macros::*;

pub use error::*;

versioned_field!(Bookmarks, Vec<Integer>, no_versions, |s| {
    let bookmark = map_res(take_till(|c| c == ','), |s: &str| s.parse::<Integer>());
    let mut bookmarks = separated_list0(comma::<nom::error::Error<_>>(), bookmark);
    let input_len = s.len();

    let (s, bookmarks) = bookmarks(s).finish().unwrap();

    if s.is_empty() {
        Ok(bookmarks)
    } else {
        let (_, s) = {
            let s = if s.len() < input_len {
                match s.strip_prefix(',') {
                    Some(s) => s,
                    None => s,
                }
            } else {
                s
            };

            take_till::<_, _, nom::error::Error<_>>(|c| c == ',')(s).unwrap()
        };
    
        // re-parse to get error
        let err = s.parse::<Integer>().unwrap_err();

        let err = if let IntErrorKind::Empty = err.kind() {
            ParseError::InvalidCommaList
        } else {
            ParseError::ParseIntError(err)
        };

        Err(err)
    }
} -> ParseError, |v| { v.iter().map(|v| v.to_string())
    .collect::<Vec<_>>().join(",") },);
versioned_field!(DistanceSpacing, Decimal, no_versions, |s| { s.parse() } -> (),,);
versioned_field!(BeatDivisor, Decimal, no_versions, |s| { s.parse() } -> (),,);
versioned_field!(GridSize, Integer, no_versions, |s| { s.parse() } -> ParseIntError,,);
versioned_field!(TimelineZoom, Decimal, no_versions, |s| { s.parse() } -> (),,);

general_section!(
    /// A struct representing the editor section of the .osu file.
    pub struct Editor {
        /// Time in milliseconds of bookmarks.
        pub bookmarks: Bookmarks,
        /// Distance snap multiplier.
        pub distance_spacing: DistanceSpacing,
        /// Beat snap divisor.
        pub beat_divisor: BeatDivisor,
        /// Grid size.
        pub grid_size: GridSize,
        /// Scale factor for the objecct timeline.
        pub timeline_zoom: TimelineZoom,
    },
    ParseError,
    " ",
);
