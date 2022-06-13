use std::num::ParseIntError;

use thiserror::Error;

pub fn pipe_vec_to_string<T>(vec: &[T]) -> String
where
    T: ToString,
{
    vec.iter()
        .map(|s| s.to_string())
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

pub fn display_colon_fields(fields: &[(&str, &Option<String>)], space_after_colon: bool) -> String {
    let mut fields_str_builder = Vec::with_capacity(fields.len());
    let space_after_colon = if space_after_colon { " " } else { "" };

    for (field_name, field) in fields {
        if let Some(field) = field {
            fields_str_builder.push(format!("{field_name}:{space_after_colon}{field}",));
        }
    }

    fields_str_builder.join("\n")
}
