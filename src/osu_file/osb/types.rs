use nom::{
    bytes::{complete::take_till, streaming::tag},
    combinator::rest,
    error::context,
    sequence::{preceded, tuple},
};

use super::*;
use crate::osu_file::VersionedFromStr;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Variable {
    pub name: String,
    pub value: String,
}

impl VersionedFromStr for Variable {
    type Err = ParseVariableError;

    fn from_str(s: &str, _: Version) -> Result<Option<Self>, Self::Err> {
        let header = tag("$");
        let value_name = take_till(|c| c == '=' || c == '\n');
        let equals = tag("=");
        let value = rest;

        let (_, (name, value)) = tuple((
            preceded(
                context(ParseVariableError::MissingHeader.into(), header),
                value_name,
            ),
            preceded(
                context(ParseVariableError::MissingEquals.into(), equals),
                value,
            ),
        ))(s)?;

        Ok(Some(Variable {
            name: name.to_string(),
            value: value.to_string(),
        }))
    }
}

impl VersionedToString for Variable {
    fn to_string(&self, _: Version) -> Option<String> {
        Some(format!("${}={}", self.name, self.value))
    }
}
