pub mod storyboard;

use std::{error::Error, str::FromStr};

use thiserror::Error;

use self::storyboard::Object;

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
                        let mut count = 0usize;
                        while let Some(header_no_indent) = header.strip_prefix(" ") {
                            header = header_no_indent;
                            count += 1;
                        }
                        count
                    };
                    let header = header;

                    // its a storyboard
                    if header_indent > 0 {
                        match events.0.last_mut() {
                            Some(sprite) => {
                                if let Event::Storyboard(sprite) = sprite {
                                    sprite.push_cmd(
                                        header.parse().map_err(|_err| {
                                            EventsParseError::StoryboardParseError
                                        })?,
                                    );
                                } else {
                                    return Err(EventsParseError::StoryboardCmdWithNoSprite);
                                }
                            }
                            None => return Err(EventsParseError::StoryboardCmdWithNoSprite),
                        }
                    }

                    let event_type = header;

                    let start_time = s
                        .next()
                        .ok_or(EventsParseError::MissingField("startTime"))?;
                    let start_time =
                        start_time
                            .parse()
                            .map_err(|err| EventsParseError::FieldParseError {
                                source: Box::new(err),
                                value: start_time.to_string(),
                                field_name: "startTime",
                            })?;

                    let event_params =
                        EventParams::try_from((event_type, &mut s)).map_err(|err| {
                            EventsParseError::FieldParseError {
                                source: Box::new(err),
                                // TODO does this even work
                                value: format!("{}{}", event_type, s.collect::<String>()),
                                field_name: "eventParams",
                            }
                        })?;

                    events.0.push(Event::NormalEvent {
                        start_time,
                        event_params,
                    })
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
    // TODO more specific error
    #[error("A storyboard command was used without defined sprite or animation sprite")]
    StoryboardCmdWithNoSprite,
    /// There was a problem parsing some `storyboard` element.
    // TODO more specific error
    #[error("There was a problem parsing a storyboard element")]
    StoryboardParseError,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Background {
    pub filename: String,
    // only used to make resulting output match the input file.
    filename_has_quotes: bool,
    pub position: Position,
}

impl Background {
    pub fn new(filename: String, position: Position) -> Self {
        let (filename, filename_has_quotes) =
            if filename.starts_with('"') && filename.ends_with('"') {
                (&filename[1..filename.len()], true)
            } else {
                (filename.as_str(), false)
            };

        Self {
            filename: filename.to_string(),
            filename_has_quotes,
            position,
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Video {
    pub filename: String,
    // only used to make resulting output match the input file.
    filename_has_quotes: bool,
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
                let filename_has_quotes = filename.starts_with('"') && filename.ends_with('"');
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

                let filename = filename.to_string();

                if event_type == "0" {
                    Ok(EventParams::Background(Background {
                        filename,
                        filename_has_quotes,
                        position,
                    }))
                } else {
                    Ok(EventParams::Video(Video {
                        filename,
                        filename_has_quotes,
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
