use std::{str::FromStr, error::Error};


#[derive(Default)]
pub struct Events;

impl FromStr for Events {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}