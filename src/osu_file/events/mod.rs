pub mod error;
pub mod storyboard;

use std::{
    fmt::Display,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::osu_file::events::storyboard::sprites;

use self::storyboard::{
    cmds::{Command, CommandProperties},
    error::ObjectParseError,
    sprites::Object,
};

use super::{types::Error, Integer, Position};

use self::error::*;

#[derive(Default, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Events(pub Vec<Event>);

impl Display for Events {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl FromStr for Events {
    type Err = crate::osu_file::types::Error<ParseError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.lines();
        let mut events = Events::default();
        let mut line_number = 0;

        for line in s {
            if !line.trim().is_empty() {
                match line.trim().strip_prefix("//") {
                    Some(comment) => events.0.push(Event::Comment(comment.to_string())),
                    None => {
                        let mut s = line.split(',');

                        let mut header = s.next().ok_or(ParseError::MissingField("eventType"))?;
                        let header_indent = {
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
                                    sprite
                                        .try_push_cmd(
                                            line.parse().map_err(|err| {
                                                Error::from(err).combine(line_number)
                                            })?,
                                            header_indent,
                                        )
                                        .map_err(|err| Error::from(err).combine(line_number))?;
                                }
                                _ => {
                                    return Err(Error::from_err_with_line(
                                        ParseError::StoryboardCmdWithNoSprite,
                                        line_number,
                                    ))
                                }
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
                                    let start_time =
                                        s.next().ok_or(ParseError::MissingField("startTime"))?;
                                    let start_time = start_time.parse().map_err(|err| {
                                        ParseError::FieldParseError {
                                            source: Box::new(err),
                                            value: start_time.to_string(),
                                            field_name: "startTime",
                                        }
                                    })?;

                                    let event_params = EventParams::try_from((header, &mut s))
                                        .map_err(|err| {
                                            Error::from_err_with_line(
                                                ParseError::FieldParseError {
                                                    source: Box::new(err),
                                                    // TODO does this even work
                                                    value: format!(
                                                        "{}{}",
                                                        header,
                                                        s.collect::<String>()
                                                    ),
                                                    field_name: "eventParams",
                                                },
                                                line_number,
                                            )
                                        })?;

                                    events.0.push(Event::NormalEvent {
                                        start_time,
                                        event_params,
                                    })
                                } else {
                                    return Err(Error::from_err_with_line(
                                        ParseError::StoryboardObjectParseError(err),
                                        line_number,
                                    ));
                                }
                            }
                        }
                    }
                }
            }

            line_number += 1;
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

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let event_str = match self {
            Event::Comment(comment) => format!("//{comment}"),
            // TODO do normal event type's value storage such as if being 0, it will remember that for 1:1 matching
            Event::NormalEvent {
                start_time,
                event_params,
            } => {
                let position_str = |position: &Position| {
                    if position.x == 0 && position.y == 0 {
                        String::new()
                    } else {
                        format!(",{},{}", position.x, position.y)
                    }
                };

                match event_params {
                    // background by default doesn't print the shorthand for some reason
                    EventParams::Background(background) => format!(
                        "0,{start_time},{},{},{}",
                        background.filename.to_string_lossy(),
                        background.position.x,
                        background.position.y,
                    ),
                    EventParams::Video(video) => format!(
                        "1,{start_time},{}{}",
                        video.filename.to_string_lossy(),
                        position_str(&video.position),
                    ),
                    EventParams::Break(_break) => format!("2,{start_time},{}", _break.end_time),
                    EventParams::ColourTransformation(transformation) => format!(
                        "3,{start_time},{},{},{}",
                        transformation.red, transformation.green, transformation.blue
                    ),
                }
            }
            Event::Storyboard(object) => {
                let pos_str = format!("{},{}", object.position.x, object.position.y);

                let object_str = match &object.object_type {
                    sprites::ObjectType::Sprite(sprite) => format!(
                        "Sprite,{},{},{},{}",
                        object.layer,
                        object.origin,
                        // TODO make sure if the filepath has spaces, use the quotes no matter what, but don't add quotes if it already has
                        sprite.filepath.to_string_lossy(),
                        pos_str
                    ),
                    sprites::ObjectType::Animation(anim) => {
                        format!(
                            "Animation,{},{},{},{},{},{},{}",
                            object.layer,
                            object.origin,
                            anim.filepath.to_string_lossy(),
                            pos_str,
                            anim.frame_count,
                            anim.frame_delay,
                            anim.loop_type
                        )
                    }
                };

                let cmds = {
                    if object.commands.is_empty() {
                        None
                    } else {
                        Some(command_recursive_display(&object.commands, 1).join("\n"))
                    }
                };

                match cmds {
                    Some(cmds) => format!("{object_str}\n{cmds}"),
                    None => object_str,
                }
            }
        };

        write!(f, "{event_str}")
    }
}

fn command_recursive_display(commands: &[Command], indentation: usize) -> Vec<String> {
    let mut cmd_lines = Vec::with_capacity(commands.len());

    for cmd in commands {
        cmd_lines.push(format!("{}{cmd}", " ".repeat(indentation)));

        if let CommandProperties::Loop { commands, .. }
        | CommandProperties::Trigger { commands, .. } = &cmd.properties
        {
            for command_str in command_recursive_display(commands, indentation + 1) {
                cmd_lines.push(command_str);
            }
        }
    }

    cmd_lines
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
pub struct ColourTransformation {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum EventParams {
    Background(Background),
    Video(Video),
    Break(Break),
    ColourTransformation(ColourTransformation),
}

impl<'a, T> TryFrom<(&str, &mut T)> for EventParams
where
    T: Iterator<Item = &'a str> + Clone + std::fmt::Debug,
{
    type Error = EventParamsParseError;

    fn try_from((event_type, event_params): (&str, &mut T)) -> Result<Self, Self::Error> {
        let event_type = event_type.trim();

        match event_type {
            "0" | "1" | "Video" => {
                let filename = event_params
                    .next()
                    .ok_or(EventParamsParseError::MissingField("filename"))?;
                let position = {
                    let mut has_position = false;

                    let x = match event_params.next() {
                        Some(x) => {
                            has_position = true;
                            x
                        }
                        None => "0",
                    };
                    let x = x
                        .parse()
                        .map_err(|err| EventParamsParseError::ParseFieldError {
                            source: Box::new(err),
                            value: x.to_string(),
                            field_name: "xOffset",
                        })?;

                    let y = match event_params.next() {
                        Some(y) => {
                            if has_position {
                                y
                            } else {
                                return Err(EventParamsParseError::MissingField("xOffset"));
                            }
                        }
                        None => {
                            if has_position {
                                return Err(EventParamsParseError::MissingField("yOffset"));
                            } else {
                                "0"
                            }
                        }
                    };
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
            "3" => {
                let red = event_params
                    .next()
                    .ok_or(EventParamsParseError::MissingField("red"))?;
                let red = red
                    .parse()
                    .map_err(|err| EventParamsParseError::ParseFieldError {
                        source: Box::new(err),
                        value: red.to_string(),
                        field_name: "red",
                    })?;

                let green = event_params
                    .next()
                    .ok_or(EventParamsParseError::MissingField("green"))?;
                let green =
                    green
                        .parse()
                        .map_err(|err| EventParamsParseError::ParseFieldError {
                            source: Box::new(err),
                            value: green.to_string(),
                            field_name: "green",
                        })?;

                let blue = event_params
                    .next()
                    .ok_or(EventParamsParseError::MissingField("blue"))?;
                let blue = blue
                    .parse()
                    .map_err(|err| EventParamsParseError::ParseFieldError {
                        source: Box::new(err),
                        value: blue.to_string(),
                        field_name: "blue",
                    })?;

                Ok(EventParams::ColourTransformation(ColourTransformation {
                    red,
                    green,
                    blue,
                }))
            }
            _ => Err(EventParamsParseError::UnknownParamType(
                event_type.to_string(),
            )),
        }
    }
}
