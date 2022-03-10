use std::str::FromStr;

use thiserror::Error;

#[derive(Default)]
pub struct Metadata;

impl FromStr for Metadata {
    type Err = MetadataParseError;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Error)]
#[error("")]
pub struct MetadataParseError;
