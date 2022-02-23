use std::{str::FromStr, error::Error};


#[derive(Default)]
pub struct TimingPoint;

impl FromStr for TimingPoint {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}