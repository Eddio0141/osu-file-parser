use std::{str::FromStr, error::Error};


#[derive(Default)]
pub struct Colours;

impl FromStr for Colours {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}