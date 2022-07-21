pub mod error;
pub mod types;

use nom::multi::many0;

use crate::parsers::square_section;

use super::{Error, Events, Version, VersionedFromStr, VersionedToString};

pub use error::*;
pub use types::*;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Osb {
    pub events: Option<Events>,
    pub variables: Option<Vec<Variable>>,
}

impl VersionedFromStr for Osb {
    type Err = Error<ParseError>;

    fn from_str(s: &str, version: Version) -> std::result::Result<Option<Self>, Self::Err> {
        if version < 14 {
            return Ok(None);
        }

        // we get sections
        // only valid sections currently are [Variables] [Events]
        let (_, sections) = many0::<_, _, nom::error::Error<_>, _>(square_section())(s).unwrap();

        let mut section_parsed = Vec::with_capacity(2);
        let mut line_number = 0;

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
                    events =
                        Error::processing_line(Events::from_str(section, version), line_number)?;
                }
                _ => return Err(Error::new(ParseError::UnknownSection, section_name_line)),
            }

            section_parsed.push(section_name);
            line_number += section.lines().count();
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
                sections.push((
                    "Variables",
                    variables
                        .iter()
                        .map(|v| v.to_string(version).unwrap())
                        .collect::<Vec<_>>()
                        .join("\n"),
                ));
            }
            if let Some(events) = &self.events {
                // Events existed longer than storyboards I think
                sections.push(("Events", events.to_string(version).unwrap()))
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
