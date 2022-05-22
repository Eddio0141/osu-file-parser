pub mod error;
pub mod storyboard;

use std::{
    fmt::Display,
    path::{Path, PathBuf},
    str::FromStr,
};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    combinator::{cut, eof, fail, rest},
    error::context,
    sequence::{preceded, tuple},
    Finish, Parser,
};

use crate::{osu_file::events::storyboard::sprites, parsers::*};

use self::storyboard::{
    cmds::{Command, CommandProperties},
    error::ObjectParseError,
    sprites::Object,
};

use super::{types::Error, Integer, Position, Version};

pub use self::error::*;

#[derive(Default, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Events(pub Vec<Event>);

impl Version for Events {
    type ParseError = Error<ParseError>;

    // TODO check versions
    fn from_str_v3(s: &str) -> std::result::Result<Option<Self>, Self::ParseError>
    where
        Self: Sized,
    {
        let mut events = Events::default();

        let mut comment = preceded::<_, _, _, nom::error::Error<_>, _, _>(tag("//"), rest);

        for (line_index, line) in s.lines().enumerate() {
            if !line.trim().is_empty() {
                if let Ok((_, comment)) = comment(line) {
                    events.0.push(Event::Comment(comment.to_string()));
                } else {
                    let indent = take_while::<_, _, nom::error::Error<_>>(|c| c == ' ' || c == '_');

                    let indent = indent(line).unwrap().1.len();

                    // its a storyboard command
                    if indent > 0 {
                        match events.0.last_mut() {
                            Some(Event::Storyboard(sprite)) => Error::new_from_result_into(
                                sprite.try_push_cmd(
                                    Error::new_from_result_into(line.parse(), line_index)?,
                                    indent,
                                ),
                                line_index,
                            )?,
                            _ => {
                                return Err(Error::new(
                                    ParseError::StoryboardCmdWithNoSprite,
                                    line_index,
                                ))
                            }
                        }
                    } else {
                        // is it a storyboard object?

                        match line.parse() {
                            Ok(object) => {
                                events.0.push(Event::Storyboard(object));
                            }
                            Err(err) => {
                                if let ObjectParseError::UnknownObjectType(_) = err {
                                    let (_, (_, _, start_time, _)) = tuple((
                                        comma_field(),
                                        context("missing_start_time", comma()),
                                        context("invalid_end_time", comma_field_type()),
                                        rest,
                                    ))(
                                        line
                                    )
                                    .finish()
                                    .map_err(|err: nom::error::VerboseError<_>| {
                                        for (_, err) in err.errors {
                                            if let nom::error::VerboseErrorKind::Context(context) =
                                                err
                                            {
                                                let err = match context {
                                                    "missing_start_time" => {
                                                        ParseError::MissingStartTime
                                                    }
                                                    "invalid_end_time" => {
                                                        ParseError::ParseStartTime
                                                    }
                                                    _ => unreachable!(),
                                                };

                                                return Error::new(err, line_index);
                                            }
                                        }

                                        unimplemented!("I somehow forgot to implement a context");
                                    })?;

                                    events.0.push(Event::NormalEvent {
                                        start_time,
                                        event_params: Error::new_from_result_into(
                                            line.parse(),
                                            line_index,
                                        )?,
                                    })
                                } else {
                                    return Err(Error::new(
                                        ParseError::StoryboardObjectParseError(err),
                                        line_index,
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(Some(events))
    }

    fn to_string_v3(&self) -> String {
        self.0
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join("\n")
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

impl FromStr for EventParams {
    type Err = EventParamsParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let start_time = || comma_field();
        let file_name = || comma_field().map(|f| PathBuf::from(f));
        let coordinates = || {
            alt((
                eof.map(|_| (0, 0)),
                tuple((
                    preceded(
                        context(EventParamsParseError::MissingXOffset.into(), comma()),
                        context(
                            EventParamsParseError::InvalidXOffset.into(),
                            comma_field_type(),
                        ),
                    ),
                    preceded(
                        context(EventParamsParseError::MissingYOffset.into(), comma()),
                        context(
                            EventParamsParseError::InvalidYOffset.into(),
                            consume_rest_type(),
                        ),
                    ),
                )),
            ))
            .map(|(x, y)| Position { x, y })
        };
        let file_name_and_coordinates = || tuple((file_name(), coordinates()));
        let end_time = consume_rest_type();

        let background = preceded(
            tuple((
                tag("0"),
                cut(tuple((
                    context(EventParamsParseError::MissingStartTime.into(), comma()),
                    start_time(),
                    context(EventParamsParseError::MissingFileName.into(), comma()),
                ))),
            )),
            cut(file_name_and_coordinates()),
        )
        .map(|(filename, position)| EventParams::Background(Background { filename, position }));
        let video = preceded(
            tuple((
                alt((tag("1"), tag("Video"))),
                cut(tuple((
                    context(EventParamsParseError::MissingStartTime.into(), comma()),
                    start_time(),
                    context(EventParamsParseError::MissingFileName.into(), comma()),
                ))),
            )),
            cut(file_name_and_coordinates()),
        )
        .map(|(filename, position)| EventParams::Video(Video { filename, position }));
        let break_ = preceded(
            tuple((
                alt((tag("2"), tag("Break"))),
                cut(tuple((
                    context(EventParamsParseError::MissingStartTime.into(), comma()),
                    start_time(),
                    context(EventParamsParseError::MissingEndTime.into(), comma()),
                ))),
            )),
            cut(context(
                EventParamsParseError::InvalidEndTime.into(),
                end_time,
            )),
        )
        .map(|end_time| EventParams::Break(Break { end_time }));
        let colour_transformation = preceded(
            tuple((
                tag("3"),
                cut(tuple((
                    context(EventParamsParseError::MissingStartTime.into(), comma()),
                    start_time(),
                    context(EventParamsParseError::MissingRed.into(), comma()),
                ))),
            )),
            cut(tuple((
                context(EventParamsParseError::InvalidRed.into(), comma_field_type()),
                preceded(
                    context(EventParamsParseError::MissingGreen.into(), comma()),
                    context(EventParamsParseError::InvalidGreen.into(), comma_field_type()),
                ),
                preceded(
                    context(EventParamsParseError::MissingBlue.into(), comma()),
                    context(EventParamsParseError::InvalidBlue.into(), consume_rest_type()),
                ),
            ))),
        )
        .map(|(red, green, blue)| {
            EventParams::ColourTransformation(ColourTransformation { red, green, blue })
        });

        let result = alt((
            background,
            video,
            break_,
            colour_transformation,
            context(EventParamsParseError::UnknownEventType.into(), fail),
        ))(s)?;

        Ok(result.1)
    }
}
