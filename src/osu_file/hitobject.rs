use std::{str::FromStr, error::Error};

#[derive(Default)]
pub struct HitObject;

impl FromStr for HitObject {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
