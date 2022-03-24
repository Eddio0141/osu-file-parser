use std::{error::Error, num::ParseIntError, str::FromStr};

use thiserror::Error;

use super::{hitobject::error::PipeVecParseErr, Integer};

pub fn pipe_vec_to_string<T>(vec: &[T]) -> String
where
    T: ToString,
{
    vec.iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .join("|")
}

pub fn str_to_pipe_vec<T>(s: &str) -> Result<Vec<T>, PipeVecParseErr>
where
    T: FromStr,
    <T as FromStr>::Err: Error + 'static,
{
    // TODO is there a better way to handle this stuff
    let s = s.split('|').collect::<Vec<_>>();
    let mut err_index = None;

    s.iter()
        .enumerate()
        .map(|(i, s)| {
            let s = s.parse();
            if s.is_err() {
                err_index = Some(i)
            }
            s
        })
        .collect::<Result<Vec<T>, _>>()
        .map_err(|err| PipeVecParseErr {
            source: Box::new(err),
            value: s[err_index.unwrap()].to_string(),
        })
}

pub fn nth_bit_state_i64(value: i64, nth_bit: u8) -> bool {
    value >> nth_bit & 1 == 1
}

pub fn parse_zero_one_bool(value: &str) -> Result<bool, ParseZeroOneBoolError> {
    let value = value
        .parse()
        .map_err(|err| ParseZeroOneBoolError::ValueParseError {
            source: err,
            value: value.to_string(),
        })?;

    match value {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(ParseZeroOneBoolError::InvalidValue(value)),
    }
}

#[derive(Debug, Error)]
pub enum ParseZeroOneBoolError {
    #[error("Error parsing {value} as an Integer")]
    ValueParseError {
        #[source]
        source: ParseIntError,
        value: String,
    },
    #[error("Error parsing {0} as `true` or `false`, expected value of 0 or 1")]
    InvalidValue(Integer),
}

pub fn display_colon_fields(
    f: &mut std::fmt::Formatter,
    fields: &[(&str, &Option<String>)],
    space_after_colon: bool,
    new_line: &str,
) -> std::fmt::Result {
    let mut fields_str_builder = Vec::with_capacity(fields.len());
    let space_after_colon = if space_after_colon { 1usize } else { 0usize };

    for (field_name, field) in fields {
        if let Some(field) = field {
            fields_str_builder.push(format!(
                "{field_name}:{}{field}",
                " ".repeat(space_after_colon)
            ));
        }
    }

    write!(f, "{}", fields_str_builder.join(new_line))
}
