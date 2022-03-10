use std::str::FromStr;

use thiserror::Error;

#[derive(Default)]
pub struct Difficulty;

impl FromStr for Difficulty {
    type Err = DifficultyParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Error)]
#[error("")]
pub struct DifficultyParseError;
