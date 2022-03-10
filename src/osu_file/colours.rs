use std::str::FromStr;

use thiserror::Error;

#[derive(Default)]
pub struct Colours;

impl FromStr for Colours {
    type Err = ColoursParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Error)]
#[error("")]
pub struct ColoursParseError;
