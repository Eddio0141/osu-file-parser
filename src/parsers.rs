use std::num::ParseIntError;

use nom::{
    bytes::complete::{is_not, take_while},
    character::complete::char,
    character::complete::multispace0,
    combinator::{map, map_res, opt},
    error::{FromExternalError, ParseError},
    multi::{many0, many_till},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

// pub fn leading_ws<'a, F: 'a, O, E: ParseError<&'a str>>(
//     inner: F,
// ) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
// where
//     F: Fn(&'a str) -> IResult<&'a str, O, E>,
// {
//     preceded(multispace0, inner)
// }

pub fn trailing_ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    terminated(inner, multispace0)
}

pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

pub fn get_colon_field_value_lines(s: &str) -> IResult<&str, Vec<(&str, &str)>> {
    let field_name = is_not::<_, _, nom::error::Error<_>>(": ");
    let field_separator = ws(char(':'));
    let field_value = is_not("\n\r");
    let field = tuple((
        terminated(field_name, field_separator),
        trailing_ws(field_value),
    ));

    many0(field)(s)
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

pub fn comma_field_i32<'a, E>() -> impl FnMut(&'a str) -> IResult<&str, i32, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    map_res(comma_field(), |s| s.parse::<i32>())
}
