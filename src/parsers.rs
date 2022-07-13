use std::str::FromStr;

use nom::{
    bytes::complete::{tag, take_till, take_while},
    character::complete::multispace0,
    character::complete::{char, space0},
    combinator::{eof, map_res, rest},
    error::{FromExternalError, ParseError},
    multi::{many0, separated_list0},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

use crate::osu_file::VersionedFromString;

// pub fn leading_ws<'a, F: 'a, O, E: ParseError<&'a str>>(
//     inner: F,
// ) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
// where
//     F: FnMut(&'a str) -> IResult<&'a str, O, E>,
// {
//     preceded(multispace0, inner)
// }

// pub fn trailing_ws<'a, F: 'a, O, E: ParseError<&'a str>>(
//     inner: F,
// ) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
// where
//     F: FnMut(&'a str) -> IResult<&'a str, O, E>,
// {
//     terminated(inner, multispace0)
// }

// pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
//     inner: F,
// ) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
// where
//     F: FnMut(&'a str) -> IResult<&'a str, O, E>,
// {
//     delimited(multispace0, inner, multispace0)
// }

/// Parses fields that has a structure of `key: value`, returning in form of `(key, ws, value, ws)`.
pub fn get_colon_field_value_lines(s: &str) -> IResult<&str, Vec<(&str, &str, &str, &str)>> {
    let field_name = take_till(|c| c == ':' || c == '\n');
    let field_separator = char(':');
    let field_value = take_till(|c| c == '\r' || c == '\n');
    // we keep whitespace information that can contain newlines
    let field_line = tuple((
        terminated(field_name, field_separator),
        space0,
        field_value,
        multispace0,
    ));

    many0(field_line)(s)
}

pub fn pipe_vec_map<'a, E, T>() -> impl FnMut(&'a str) -> IResult<&'a str, Vec<T>, E>
where
    E: ParseError<&'a str> + nom::error::FromExternalError<&'a str, <T as FromStr>::Err>,
    T: FromStr,
{
    let item = take_while(|c: char| !['|', ',', '\r', '\n'].contains(&c));
    let item = map_res(item, |s: &str| s.parse());

    separated_list0(tag("|"), item)
}

pub fn comma<'a, E>() -> impl FnMut(&'a str) -> IResult<&'a str, &str, E>
where
    E: ParseError<&'a str>,
{
    tag(",")
}

pub fn comma_field<'a, E>() -> impl FnMut(&'a str) -> IResult<&str, &str, E>
where
    E: ParseError<&'a str>,
{
    take_while(|c: char| c != ',')
}

pub fn comma_field_type<'a, E, T>() -> impl FnMut(&'a str) -> IResult<&str, T, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, <T as FromStr>::Err>,
    T: FromStr,
{
    map_res(comma_field(), |i| i.parse())
}

pub fn comma_field_versioned_type<'a, E, T>(
    version: usize,
) -> impl FnMut(&'a str) -> IResult<&str, T, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, <T as VersionedFromString>::ParseError>,
    T: VersionedFromString,
{
    map_res(comma_field(), move |i| {
        T::from_str(i, version).map(|i| i.unwrap())
    })
}

pub fn consume_rest_type<'a, E, T>() -> impl FnMut(&'a str) -> IResult<&str, T, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, <T as FromStr>::Err>,
    T: FromStr,
{
    map_res(rest, |s: &str| s.parse())
}

pub fn consume_rest_versioned_type<'a, E, T>(
    version: usize,
) -> impl FnMut(&'a str) -> IResult<&str, T, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, <T as VersionedFromString>::ParseError>,
    T: VersionedFromString,
{
    map_res(rest, move |s: &str| {
        T::from_str(s, version).map(|s| s.unwrap())
    })
}

pub fn nothing<'a, E>() -> impl FnMut(&'a str) -> IResult<&str, &str, E>
where
    E: ParseError<&'a str>,
{
    preceded(space0, eof)
}

pub fn square_section<'a, E>(
) -> impl FnMut(&'a str) -> IResult<&'a str, (&str, &str, &str, &str), E>
where
    E: ParseError<&'a str>,
{
    let section_open = tag("[");
    let section_close = tag("]");
    let section_name_inner = take_till(|c: char| c == ']' || c == '\r' || c == '\n');
    let section_name = delimited(section_open, section_name_inner, section_close);
    let section_until = take_till(|c| c == '[');
    tuple((multispace0, section_name, multispace0, section_until))
}
