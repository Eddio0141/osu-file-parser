pub mod error;
pub mod types;

use nom::multi::many0;

use crate::parsers::square_section;

use super::{Error, Events, Version, VersionedFromStr, VersionedToString};

pub use error::*;
pub use types::*;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Osb {
    pub variables: Option<Vec<Variable>>,
    pub events: Option<Events>,
}

impl VersionedFromStr for Osb {
    type Err = Error<ParseError>;

    fn from_str(s: &str, version: Version) -> std::result::Result<Option<Self>, Self::Err> {
        if version < 14 {
            return Ok(None);
        }

        let pre_section_count = s
            .lines()
            .take_while(|s| {
                let s = s.trim();
                !s.trim().starts_with('[') && !s.trim().ends_with(']')
            })
            .count();

        for (i, line) in s.lines().take(pre_section_count).enumerate() {
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            if line.starts_with("//") {
                continue;
            }

            return Err(Error::new(ParseError::UnexpectedLine, i));
        }

        let s = s
            .lines()
            .skip(pre_section_count)
            .collect::<Vec<_>>()
            .join("\n");

        // we get sections
        // only valid sections currently are [Variables] [Events]
        let (_, sections) = many0(square_section())(&s).unwrap();

        let mut section_parsed = Vec::with_capacity(2);
        let mut line_number = pre_section_count;

        let (mut events, mut variables) = (None, None);

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
                        if line.trim().is_empty() {
                            continue;
                        }

                        let variable = Error::new_from_result_into(
                            Variable::from_str(line, version).map(|v| v.unwrap()),
                            line_number + i,
                        )?;

                        vars.push(variable);
                    }
                    variables = Some(vars);
                }
                "Events" => {
                    events = Error::processing_line(
                        Events::from_str_variables(
                            section,
                            version,
                            variables.as_ref().unwrap_or(&Vec::new()),
                        ),
                        line_number,
                    )?;
                }
                _ => return Err(Error::new(ParseError::UnknownSection, section_name_line)),
            }

            section_parsed.push(section_name);
            line_number += section.lines().count().saturating_sub(1);
        }

        Ok(Some(Osb { events, variables }))
    }
}

impl VersionedToString for Osb {
    fn to_string(&self, version: Version) -> Option<String> {
        if version < 14 {
            None
        } else {
            let mut sections = Vec::new();

            if let Some(variables) = &self.variables {
                sections.push(format!(
                    "[Variables]\n{}",
                    variables
                        .iter()
                        .map(|v| v.to_string(version).unwrap())
                        .collect::<Vec<_>>()
                        .join("\n"),
                ));
            }
            if let Some(events) = &self.events {
                // Events existed longer than storyboards I think
                sections.push(format!(
                    "[Events]\n{}",
                    events
                        .to_string_variables(
                            version,
                            self.variables.as_ref().unwrap_or(&Vec::new()),
                        )
                        .unwrap()
                ))
            }

            Some(sections.join("\n\n"))
        }
    }
}
