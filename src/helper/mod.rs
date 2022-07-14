pub mod macros;
pub mod trait_ext;

use std::num::ParseIntError;

use thiserror::Error;

use crate::osu_file::{Version, VersionedToString};

pub fn pipe_vec_to_string<T>(vec: &[T], version: Version) -> String
where
    T: VersionedToString,
{
    vec.iter()
        .map(|s| s.to_string(version).unwrap())
        .collect::<Vec<_>>()
        .join("|")
}

pub fn nth_bit_state_i64(value: i64, nth_bit: u8) -> bool {
    value >> nth_bit & 1 == 1
}

pub fn parse_zero_one_bool(value: &str) -> Result<bool, ParseZeroOneBoolError> {
    let value = value.parse()?;

    match value {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(ParseZeroOneBoolError::InvalidValue),
    }
}

#[derive(Debug, Error)]
pub enum ParseZeroOneBoolError {
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error("Error parsing value as `true` or `false`, expected value of 0 or 1")]
    InvalidValue,
}
