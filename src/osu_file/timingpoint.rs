use std::str::FromStr;

use thiserror::Error;

#[derive(Default)]
pub struct TimingPoint;

impl FromStr for TimingPoint {
    type Err = TimingPointParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Error)]
#[error("")]
pub struct TimingPointParseError;
