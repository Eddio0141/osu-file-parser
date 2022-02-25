use std::{error::Error, str::FromStr, fmt::Display};

#[derive(Default)]
pub struct Editor;

impl FromStr for Editor {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug)]
pub struct EditorParseError;

impl Display for EditorParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "There was a problem parsing the `Editor` section")
    }
}
