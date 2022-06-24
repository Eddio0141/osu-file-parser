pub mod error;

use std::{fmt::Display, str::FromStr};

pub use error::*;
use nom::{
    bytes::{complete::take_till, streaming::tag},
    combinator::rest,
    error::context,
    multi::many0,
    sequence::{preceded, tuple},
};

use crate::parsers::square_section;

use super::{events::storyboard::sprites::Object, Error, VersionedFromString, VersionedToString};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Osb {
    pub objects: Option<Vec<Object>>,
    pub variables: Option<Vec<Variable>>,
}

impl VersionedFromString for Osb {
    type ParseError = Error<ParseError>;

    fn from_str(s: &str, version: usize) -> std::result::Result<Option<Self>, Self::ParseError> {
        if version < 14 {
            Ok(None)
        } else {
            // we get sections
            // only valid sections currently are [Variables] [Events]
            let (_, sections) =
                many0::<_, _, nom::error::Error<_>, _>(square_section())(s).unwrap();

            let mut section_parsed = Vec::with_capacity(2);
            let mut line_number = 0;

            let (mut objects, mut variables) = (None, None);

            for (ws, section_name, ws2, section) in sections {
                line_number += ws.lines().count();

                if section_parsed.contains(&section_name) {
                    return Err(Error::new(ParseError::DuplicateSections, line_number));
                }

                let section_name_line = line_number;
                line_number += ws2.lines().count();

                match section_name {
                    "Variables" => {
                        let mut vars = Vec::new();
                        for (i, line) in section.lines().enumerate() {
                            if !line.is_empty() {
                                let variable =
                                    Error::new_from_result_into(line.parse(), line_number + i)?;

                                vars.push(variable);
                            }
                        }
                        variables = Some(vars);
                    }
                    "Events" => {
                        let mut objs = Vec::new();
                        for (i, line) in section.lines().enumerate() {
                            if !line.is_empty() {
                                let object =
                                    Error::new_from_result_into(line.parse(), line_number + i)?;

                                objs.push(object);
                            }
                        }
                        objects = Some(objs);
                    }
                    _ => return Err(Error::new(ParseError::UnknownSection, section_name_line)),
                }

                section_parsed.push(section_name);
                line_number += section.lines().count();
            }

            Ok(Some(Osb { objects, variables }))
        }
    }
}

impl VersionedToString for Osb {
    fn to_string(&self, version: usize) -> Option<String> {
        if version < 14 {
            None
        } else {
            let mut sections = Vec::new();

            if let Some(variables) = &self.variables {
                sections.push((
                    "Variables",
                    variables
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join("\n"),
                ));
            }
            if let Some(objects) = &self.objects {
                sections.push((
                    "Events",
                    objects
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join("\n"),
                ))
            }

            Some(
                sections
                    .iter()
                    .map(|(name, section)| format!("[{name}]\n{section}"))
                    .collect::<Vec<_>>()
                    .join("\n\n"),
            )
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Variable {
    pub name: String,
    pub value: String,
}

impl FromStr for Variable {
    type Err = VariableParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let header = tag("$");
        let value_name = take_till(|c| c == '=');
        let equals = tag("=");
        let value = rest;

        let (_, (name, value)) = tuple((
            preceded(
                context(VariableParseError::MissingHeader.into(), header),
                value_name,
            ),
            preceded(
                context(VariableParseError::MissingEquals.into(), equals),
                value,
            ),
        ))(s)?;

        Ok(Variable {
            name: name.to_string(),
            value: value.to_string(),
        })
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${}={}", self.name, self.value)
    }
}