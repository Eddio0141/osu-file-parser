use std::{error::Error, str::FromStr};

#[derive(Default)]
pub struct Editor;

impl FromStr for Editor {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
