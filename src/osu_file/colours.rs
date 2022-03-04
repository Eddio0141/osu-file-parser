use std::str::FromStr;

use super::section_error::SectionParseError;

#[derive(Default)]
pub struct Colours;

impl FromStr for Colours {
    type Err = SectionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
