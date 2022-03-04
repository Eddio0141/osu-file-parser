use std::str::FromStr;

use super::section_error::SectionParseError;

#[derive(Default)]
pub struct Difficulty;

impl FromStr for Difficulty {
    type Err = SectionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
