use std::str::FromStr;

use super::section_error::SectionParseError;

#[derive(Default)]
pub struct TimingPoint;

impl FromStr for TimingPoint {
    type Err = SectionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
