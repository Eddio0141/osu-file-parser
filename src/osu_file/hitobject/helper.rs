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
