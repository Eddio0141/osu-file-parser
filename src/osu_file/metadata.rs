use std::{error::Error, str::FromStr};

#[derive(Default)]
pub struct Metadata;

impl FromStr for Metadata {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
