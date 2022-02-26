use std::{str::FromStr, error::Error};

use super::section_error::SectionParseError;

#[derive(Default)]
pub struct HitObject;

impl FromStr for HitObject {
    type Err = SectionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
