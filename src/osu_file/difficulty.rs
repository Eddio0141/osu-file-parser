use std::{error::Error, str::FromStr};

#[derive(Default)]
pub struct Difficulty;

impl FromStr for Difficulty {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
