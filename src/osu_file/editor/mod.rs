pub mod error;

use std::num::ParseIntError;

use nom::{bytes::complete::take_till, combinator::map_res, multi::separated_list0, Finish};
use rust_decimal::Decimal;

use crate::parsers::{comma, get_colon_field_value_lines};

use super::Integer;
use crate::helper::macros::*;

pub use self::error::*;

versioned_field!(Bookmarks, Vec<Integer>, no_versions, |s| {
    let bookmark = map_res(take_till(|c| c == ','), |s: &str| s.parse::<Integer>());
    let mut bookmarks = separated_list0(comma::<nom::error::Error<_>>(), bookmark);

    let (_, bookmarks) = bookmarks(s).finish().map_err(|err| {
        match err.code {
            // TODO test those errors
            nom::error::ErrorKind::SeparatedList => {
                ParseError::InvalidCommaList
            }
            nom::error::ErrorKind::MapRes => {
                // get section of the input that caused the error, and re-parse to get error
                let err = err.input.parse::<Integer>().unwrap_err();
                ParseError::ParseIntError(err)
            }
            _ => unimplemented!(),
        }
    })?;

    Ok(bookmarks)
} -> ParseError, |v| { v.iter().map(|v| v.to_string())
    .collect::<Vec<_>>().join(",") },);
versioned_field!(DistanceSpacing, Decimal, no_versions, |s| { s.parse() } -> rust_decimal::Error,,);
versioned_field!(BeatDivisor, Decimal, no_versions, |s| { s.parse() } -> rust_decimal::Error,,);
versioned_field!(GridSize, Integer, no_versions, |s| { s.parse() } -> ParseIntError,,);
versioned_field!(TimelineZoom, Decimal, no_versions, |s| { s.parse() } -> rust_decimal::Error,,);

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
    ParseError
);
