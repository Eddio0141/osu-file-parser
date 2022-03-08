use std::{error::Error, str::FromStr};

use super::error::PipeVecParseErr;

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
    s.split('|')
        .map(|s| s.parse())
        .collect::<Result<Vec<T>, _>>()
        .map_err(|err| PipeVecParseErr::new(Box::new(err)))
}

pub fn nth_bit_state_i64(value: i64, nth_bit: u8) -> bool {
    value >> nth_bit & 1 == 1
}
