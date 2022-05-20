use std::num::ParseIntError;

use nom::{
    bytes::complete::take_while,
    character::complete::char,
    character::complete::multispace0,
    combinator::{map, map_res, opt},
    error::{FromExternalError, ParseError},
    multi::{many0, many_till},
    sequence::{preceded, terminated, tuple},
    IResult,
};

// pub fn leading_ws<'a, F: 'a, O, E: ParseError<&'a str>>(
//     inner: F,
// ) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
// where
//     F: FnMut(&'a str) -> IResult<&'a str, O, E>,
// {
//     preceded(multispace0, inner)
// }

pub fn trailing_ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    terminated(inner, multispace0)
}

// pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
//     inner: F,
// ) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
// where
//     F: FnMut(&'a str) -> IResult<&'a str, O, E>,
// {
//     delimited(multispace0, inner, multispace0)
// }

pub fn get_colon_field_value_lines(s: &str) -> IResult<&str, Vec<(&str, &str, &str)>> {
    let field_name = take_while(|c| c != ':' && c != '\n');
    let field_separator = char(':');
    let field_value = take_while(|c| c != '\n');
    // we keep whitespace information that can contain newlines
    let field_line = tuple((
        trailing_ws(terminated(field_name, field_separator)),
        field_value,
        multispace0,
    ));

    many0(field_line)(s)
}

pub fn pipe_vec<'a, O, E, M, E2>(mapper: M) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<O>, E>
where
    E: ParseError<&'a str> + nom::error::FromExternalError<&'a str, E2>,
    M: Fn(&str) -> Result<O, E2> + 'a,
{
    let comma = char(',');
    let item = preceded(opt(char('|')), take_while(|c: char| c != '|' && c != ','));
    let item_map = map_res(item, mapper);
    map(many_till(item_map, comma), |(v, _)| v)
}

pub fn comma<'a, E>() -> impl FnMut(&'a str) -> IResult<&'a str, char, E>
where
    E: ParseError<&'a str>,
{
    char(',')
}

pub fn comma_field<'a, E>() -> impl FnMut(&'a str) -> IResult<&str, &str, E>
where
    E: ParseError<&'a str>,
{
    take_while(|c: char| c != ',')
}

pub fn comma_field_u8<'a, E>() -> impl FnMut(&'a str) -> IResult<&str, u8, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    map_res(comma_field(), |s| s.parse::<u8>())
}

pub fn comma_field_i32<'a, E>() -> impl FnMut(&'a str) -> IResult<&str, i32, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    map_res(comma_field(), |s| s.parse::<i32>())
}

/// Consumes the rest of the input.
pub fn consume_rest<'a, E>() -> impl FnMut(&'a str) -> IResult<&str, &str, E>
where
    E: ParseError<&'a str>,
{
    take_while(|_| true)
}

/// Consumes the rest of the input and parses it as i32.
pub fn consume_rest_i32<'a, E>() -> impl FnMut(&'a str) -> IResult<&str, i32, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    map_res(consume_rest(), |s| s.parse::<i32>())
}

/// Consumes the rest of the input and parses as u8.
pub fn consume_rest_u8<'a, E>() -> impl FnMut(&'a str) -> IResult<&str, u8, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    map_res(consume_rest(), |s| s.parse::<u8>())
}
