use std::{error::Error, str::FromStr};

use thiserror::Error;

use super::{Integer, Position};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Event {
    pub start_time: Integer,
    pub event_params: EventParams,
}

impl FromStr for Event {
    type Err = EventsParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(',');

        let event_type = s
            .next()
            .ok_or(EventsParseError::MissingField("eventType"))?;

        let start_time = s
            .next()
            .ok_or(EventsParseError::MissingField("startTime"))?;
        let start_time = start_time
            .parse()
            .map_err(|err| EventsParseError::FieldParseError {
                source: Box::new(err),
                value: start_time.to_string(),
                field_name: "startTime",
            })?;

        let event_params = s
            .next()
            .ok_or(EventsParseError::MissingField("eventParams"))?;

        let event_params = EventParams::try_from((event_type, event_params)).map_err(|err| {
            EventsParseError::FieldParseError {
                source: Box::new(err),
                value: format!("{}{}", event_type, event_params),
                field_name: "eventParams",
            }
        })?;

        Ok(Self {
            start_time,
            event_params,
        })
    }
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
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Background {
    pub filename: String,
    // only used to make resulting output match the input file.
    filename_has_quotes: bool,
    pub position: Position,
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
    Storyboard,
}

impl TryFrom<(&str, &str)> for EventParams {
    type Error = EventParamsParseError;

    fn try_from((event_type, event_params): (&str, &str)) -> Result<Self, Self::Error> {
        let mut event_params = event_params.split(',');
        let event_type = event_type.trim();

        match event_type {
            "0" | "1" | "Video" => {
                let filename = event_params
                    .next()
                    .ok_or(EventParamsParseError::MissingField("filename"))?
                    .trim();
                let filename_has_quotes = filename.starts_with('"') && filename.ends_with('"');
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
