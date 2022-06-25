use nom::{IResult, Parser, error::ParseError};

use crate::{osu_file::{FilePath, Position}, parsers::comma_field};

pub fn start_time(error: &str) -> impl FnMut(&'a str) -> IResult<&str, &str, E>
where
    E: ParseError<&'a str>,
{
    comma_field()
}

pub fn file_name() -> impl FnMut(&'a str) -> IResult<&str, FilePath, E> {
    comma_field().map(|f| f.into())
}

pub fn file_name_and_coordinates() -> impl FnMut(&'a str) -> IResult<&str, (FilePath, Position), E> 
where E: ParseError<&'a str>{
    comma_field().map(|f| f.into())

}
//         let coordinates = || {
//             alt((
//                 eof.map(|_| None),
//                 tuple((
//                     preceded(
//                         context(NormalEventParamsParseError::MissingXOffset.into(), comma()),
//                         context(
//                             NormalEventParamsParseError::InvalidXOffset.into(),
//                             comma_field_type(),
//                         ),
//                     ),
//                     preceded(
//                         context(NormalEventParamsParseError::MissingYOffset.into(), comma()),
//                         context(
//                             NormalEventParamsParseError::InvalidYOffset.into(),
//                             consume_rest_type(),
//                         ),
//                     ),
//                 ))
//                 .map(|(x, y)| Some((x, y))),
//             ))
//             .map(|position| position.map(|(x, y)| Position { x, y }))
//         };
//         let file_name_and_coordinates = || tuple((file_name(), coordinates()));
//         let end_time = consume_rest_type().map(|end_time| {
//             if (3..=4).contains(&version) {
//                 end_time + OLD_VERSION_TIME_OFFSET
//             } else {
//                 end_time
//             }
//         });
