pub mod storyboard;

use std::{
    error::Error,
    path::{Path, PathBuf},
    str::FromStr,
};

use thiserror::Error;

use self::storyboard::{CommandParseError, CommandPushError, Object, ObjectParseError};

use super::{Integer, Position};

#[derive(Default, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Events(pub Vec<Event>);

impl FromStr for Events {
    type Err = EventsParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.lines();
        let mut events = Events::default();

        for line in s {
            if line.trim().is_empty() {
                continue;
            }

            match line.trim().strip_prefix("//") {
                Some(comment) => events.0.push(Event::Comment(comment.to_string())),
                None => {
                    let mut s = line.split(',');

                    let mut header = s
                        .next()
                        .ok_or(EventsParseError::MissingField("eventType"))?;
                    let header_indent = {
                        // TODO find out if u can alternate indentation by _ and space
                        let mut count = 0usize;
                        while let Some(header_no_indent) = header.strip_prefix([' ', '_']) {
                            header = header_no_indent;
                            count += 1;
                        }
                        count
                    };
                    let header = header;

                    // its a storyboard command
                    if header_indent > 0 {
                        match events.0.last_mut() {
                            Some(Event::Storyboard(sprite)) => {
                                sprite.try_push_cmd(line.parse()?, header_indent)?;
                            }
                            _ => return Err(EventsParseError::StoryboardCmdWithNoSprite),
                        }
                        continue;
                    }

                    // is it a storyboard object?
                    match line.parse() {
                        Ok(object) => {
                            events.0.push(Event::Storyboard(object));
                            continue;
                        }
                        Err(err) => {
                            if let ObjectParseError::UnknownObjectType(_) = err {
                                let start_time = s
                                    .next()
                                    .ok_or(EventsParseError::MissingField("startTime"))?;
                                let start_time = start_time.parse().map_err(|err| {
                                    EventsParseError::FieldParseError {
                                        source: Box::new(err),
                                        value: start_time.to_string(),
                                        field_name: "startTime",
                                    }
                                })?;

                                let event_params = EventParams::try_from((header, &mut s))
                                    .map_err(|err| {
                                        EventsParseError::FieldParseError {
                                            source: Box::new(err),
                                            // TODO does this even work
                                            value: format!("{}{}", header, s.collect::<String>()),
                                            field_name: "eventParams",
                                        }
                                    })?;

                                events.0.push(Event::NormalEvent {
                                    start_time,
                                    event_params,
                                })
                            } else {
                                return Err(EventsParseError::StoryboardObjectParseError(err));
                            }
                        }
                    }
                }
            }
        }

        Ok(events)
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Event {
    Comment(String),
    NormalEvent {
        start_time: Integer,
        event_params: EventParams,
    },
    Storyboard(Object),
}

/// Errors used when there was a problem parsing an [`Event`] from a `str`.
#[derive(Debug, Error)]
pub enum EventsParseError {
    /// A field is missing from the [`Event`].
    #[error("The field {0} is missing")]
    MissingField(&'static str),
    /// There was an error attempting to parse `str` as a field type.
    #[error("There was a problem parsing the {field_name} field from a `str`")]
    FieldParseError {
        #[source]
        source: Box<dyn Error>,
        value: String,
        field_name: &'static str,
    },
    /// The input has nothing or whitespace.
    #[error("The input is empty")]
    EmptyString,
    /// A `storyboard` `command` was used without defined sprite or animation sprite.
    #[error("A storyboard command was used without defined sprite or animation sprite")]
    StoryboardCmdWithNoSprite,
    #[error(transparent)]
    CommandPushError(#[from] CommandPushError),
    #[error(transparent)]
    CommandParseError(#[from] CommandParseError),
    /// There was a problem parsing some `storyboard` element.
    #[error(transparent)]
    StoryboardObjectParseError(#[from] ObjectParseError),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Background {
    pub filename: PathBuf,
    pub position: Position,
}

impl Background {
    pub fn new(filename: &Path, position: Position) -> Self {
        Self {
            filename: filename.to_path_buf(),
            position,
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Video {
    pub filename: PathBuf,
    pub position: Position,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Break {
    pub end_time: Integer,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum EventParams {
    Background(Background),
    Video(Video),
    Break(Break),
}

impl<'a, T> TryFrom<(&str, &mut T)> for EventParams
where
    T: Iterator<Item = &'a str>,
{
    type Error = EventParamsParseError;

    fn try_from((event_type, event_params): (&str, &mut T)) -> Result<Self, Self::Error> {
        let event_type = event_type.trim();

        match event_type {
            "0" | "1" | "Video" => {
                let filename = event_params
                    .next()
                    .ok_or(EventParamsParseError::MissingField("filename"))?
                    .trim();
                let filename_has_quotes =
                    filename.len() > 1 && filename.starts_with('"') && filename.ends_with('"');
                let filename = if filename_has_quotes {
                    &filename[1..filename.len()]
                } else {
                    filename
                };
                let position = {
                    let x = event_params
                        .next()
                        .ok_or(EventParamsParseError::MissingField("xOffset"))?;
                    let x = x
                        .parse()
                        .map_err(|err| EventParamsParseError::ParseFieldError {
                            source: Box::new(err),
                            value: x.to_string(),
                            field_name: "xOffset",
                        })?;

                    let y = event_params
                        .next()
                        .ok_or(EventParamsParseError::MissingField("yOffset"))?;
                    let y = y
                        .parse()
                        .map_err(|err| EventParamsParseError::ParseFieldError {
                            source: Box::new(err),
                            value: y.to_string(),
                            field_name: "yOffset",
                        })?;

                    Position { x, y }
                };

                let filename = Path::new(filename);

                if event_type == "0" {
                    Ok(EventParams::Background(Background {
                        filename: filename.to_path_buf(),
                        position,
                    }))
                } else {
                    Ok(EventParams::Video(Video {
                        filename: filename.to_path_buf(),
                        position,
                    }))
                }
            }
            "2" | "Break" => {
                let end_time = event_params
                    .next()
                    .ok_or(EventParamsParseError::MissingField("endTime"))?;
                let end_time =
                    end_time
                        .parse()
                        .map_err(|err| EventParamsParseError::ParseFieldError {
                            source: Box::new(err),
                            value: end_time.to_string(),
                            field_name: "endTime",
                        })?;

                Ok(EventParams::Break(Break { end_time }))
            }
            _ => Err(EventParamsParseError::UnknownParamType(
                event_type.to_string(),
            )),
        }
    }
}

#[derive(Debug, Error)]
pub enum EventParamsParseError {
    #[error("The field {0} is missing from the event parameter")]
    MissingField(&'static str),
    #[error("The field {field_name} failed to parse from a `str`")]
    ParseFieldError {
        #[source]
        source: Box<dyn Error>,
        value: String,
        field_name: &'static str,
    },
    #[error("Unknown field `{0}`")]
    UnknownParamType(String),
}
