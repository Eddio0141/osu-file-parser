pub mod error;

pub use error::*;
use nom::multi::many0;

use crate::parsers::square_section;

use super::{events::storyboard::sprites::Object, Error, VersionedFromString};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Osb {
    pub objects: Vec<Object>,
    pub variables: Vec<Variable>,
}

impl VersionedFromString for Osb {
    type ParseError = Error<ParseError>;

    fn from_str(s: &str, version: usize) -> std::result::Result<Option<Self>, Self::ParseError> {
        // we get sections
        // only valid sections currently are [Variables] [Events]
        let (_, sections) = many0::<_, _, nom::error::Error<_>, _>(square_section())(s).unwrap();

        let mut section_parsed = Vec::with_capacity(2);
        let mut line_number = 0;

        let (mut objects, mut variables) = (Vec::new(), Vec::new());

        for (ws, section_name, ws2, section) in sections {
            line_number += ws.lines().count();

            if section_parsed.contains(&section_name) {
                return Err(Error::new(ParseError::DuplicateSections, line_number));
            }

            let section_name_line = line_number;
            line_number += ws2.lines().count();

            match section_name {
                "Variables" => {
                    Error::processing_line(Variables::from_str(section, version), line_number)?
                }
                "Events" => todo!(),
                _ => return Err(Error::new(ParseError::UnknownSection, section_name_line)),
            }

            section_parsed.push(section_name);
            line_number += section.lines().count();
        }

        Ok(Some(Osb {
            objects,
            variables: Variables(variables),
        }))
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Variable;
