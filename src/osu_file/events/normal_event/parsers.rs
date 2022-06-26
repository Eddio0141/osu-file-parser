use nom::{
    branch::alt,
    combinator::eof,
    error::context,
    sequence::{preceded, tuple},
    IResult, Parser,
};

use crate::{
    osu_file::{events::OLD_VERSION_TIME_OFFSET, FilePath, Integer, Position},
    parsers::*,
};

pub fn file_name_and_position<'a>(
    missing_x_offset: &'static str,
    invalid_x_offset: &'static str,
    missing_y_offset: &'static str,
    invalid_y_offset: &'static str,
) -> impl FnMut(
    &'a str,
) -> IResult<&'a str, (FilePath, Option<Position>), nom::error::VerboseError<&'a str>> {
    let file_name = comma_field().map(|f| f.into());
    let coordinates = alt((
        eof.map(|_| None),
        tuple((
            preceded(
                context(missing_x_offset, comma()),
                context(invalid_x_offset, comma_field_type()),
            ),
            preceded(
                context(missing_y_offset, comma()),
                context(invalid_y_offset, consume_rest_type()),
            ),
        ))
        .map(|(x, y)| Some((x, y))),
    ))
    .map(|position| position.map(|(x, y)| Position { x, y }));

    tuple((file_name, coordinates))
}

pub fn end_time<'a>(
    error: &'static str,
    version: usize,
) -> impl FnMut(&'a str) -> IResult<&str, Integer, nom::error::VerboseError<&'a str>> {
    context(
        error,
        consume_rest_type().map(move |end_time| {
            if (3..=4).contains(&version) {
                end_time + OLD_VERSION_TIME_OFFSET
            } else {
                end_time
            }
        }),
    )
}
