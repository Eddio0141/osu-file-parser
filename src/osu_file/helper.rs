use std::{error::Error, str::FromStr, num::ParseIntError};

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

pub fn parse_zero_one_bool(value: &str) -> Result<bool, ParseBoolError> {
    let value = value
        .parse()
        .map_err(|err| ParseBoolError::ValueParseError {
            source: err,
            value: value.to_string(),
        })?;

    match value {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(ParseBoolError::InvalidValue(value)),
    }
}

#[derive(Debug, Error)]
pub enum ParseBoolError {
    #[error("Error parsing {value} as an Integer")]
    ValueParseError {
        #[source]
        source: ParseIntError,
        value: String,
    },
    #[error("Error parsing {0} as `true` or `false`, expected value of 0 or 1")]
    InvalidValue(Integer),
}
