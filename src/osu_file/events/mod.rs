pub mod error;
pub mod storyboard;

use std::path::{Path, PathBuf};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    combinator::{cut, eof, fail, rest},
    error::context,
    sequence::{preceded, terminated, tuple},
    Finish, Parser,
};

use crate::{osu_file::events::storyboard::sprites, parsers::*};

use self::storyboard::{cmds::CommandProperties, error::ObjectParseError, sprites::Object};

use super::{
    types::Error, Integer, Position, VersionedDefault, VersionedFromString, VersionedToString,
};

pub use self::error::*;

#[derive(Default, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Events(pub Vec<Event>);

const OLD_VERSION_TIME_OFFSET: Integer = 24;

impl Events {
    pub fn append_osb(&mut self, osb: &str, version: usize) -> Result<(), OsbParseError> {
        // we do a check if its properly starting with [Events]
        let (osb, _) = context(OsbParseError::MissingEventsHeader.into(), tag("[Events]"))(osb)?;

        let events = Events::from_str(osb, version)?.unwrap();

        // TODO do we sort?
        self.0.extend(events.0);

        Ok(())
    }
}

impl VersionedFromString for Events {
    type ParseError = Error<ParseError>;

    fn from_str(s: &str, version: usize) -> std::result::Result<Option<Self>, Self::ParseError> {
        let mut events = Events(Vec::new());

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
                                if let ObjectParseError::UnknownObjectType = err {
                                    // TODO clean this trash up
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

                                    let event_params = Error::new_from_result_into(
                                        EventParams::from_str(line, version),
                                        line_index,
                                    )?
                                    .unwrap();

                                    events.0.push(Event::NormalEvent {
                                        // TODO does start_time has the version offset in storyboard?
                                        start_time: if (3..=4).contains(&version)
                                            && !matches!(event_params, EventParams::Background(_))
                                        {
                                            start_time + OLD_VERSION_TIME_OFFSET
                                        } else {
                                            start_time
                                        },
                                        event_params,
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
}

impl VersionedToString for Events {
    fn to_string(&self, version: usize) -> Option<String> {
        let s = self.0.iter().map(|i| {
            if let Event::NormalEvent {
                start_time,
                event_params,
            } = i
            {
                if matches!(event_params, EventParams::Background(_)) {
                    i.to_string(version)
                } else {
                    // TODO check out whats going on in v5, seems inconsistent
                    if (3..=4).contains(&version) {
                        Event::NormalEvent {
                            start_time: *start_time,
                            event_params: {
                                let mut event_params = event_params.clone();
                                if let EventParams::Break(b) = &mut event_params {
                                    b.end_time -= OLD_VERSION_TIME_OFFSET
                                }
                                event_params
                            },
                        }
                        .to_string(version)
                    } else {
                        i.to_string(version)
                    }
                }
            } else {
                i.to_string(version)
            }
        });

        // BUG making Vec<Option<String>> into Option<Vec<String>> will lose the Some values
        // solution, filter out the None values
        Some(s.into_iter().flatten().collect::<Vec<_>>().join("\n"))
    }
}

impl VersionedDefault for Events {
    fn default(_: usize) -> Option<Self> {
        Some(Events(Vec::new()))
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[non_exhaustive]
pub enum Event {
    Comment(String),
    NormalEvent {
        start_time: Integer,
        event_params: EventParams,
    },
    Storyboard(Object),
}

impl VersionedToString for Event {
    fn to_string(&self, version: usize) -> Option<String> {
        match self {
            Event::Comment(comment) => Some(format!("//{comment}")),
            Event::NormalEvent {
                start_time,
                event_params,
            } => {
                let position_str = |position: &Option<Position>| match position {
                    Some(position) => format!(",{},{}", position.x, position.y),
                    None => String::new(),
                };
                let mut start_time = *start_time;

                if (3..=4).contains(&version) && !matches!(event_params, EventParams::Background(_))
                {
                    start_time -= OLD_VERSION_TIME_OFFSET;
                }

                match event_params {
                    EventParams::Background(background) => Some(format!(
                        "0,{start_time},{}{}",
                        background.filename.to_string_lossy(),
                        position_str(&background.position),
                    )),
                    EventParams::Video(video) => Some(format!(
                        "{},{start_time},{}{}",
                        if video.short_hand { "1" } else { "Video" },
                        video.filename.to_string_lossy(),
                        position_str(&video.position),
                    )),
                    EventParams::Break(_break) => Some(format!(
                        "{},{start_time},{}",
                        if _break.short_hand { "2" } else { "Break" },
                        _break.end_time
                    )),
                    EventParams::ColourTransformation(transformation) => {
                        if version < 14 {
                            Some(format!(
                                "3,{start_time},{},{},{}",
                                transformation.red, transformation.green, transformation.blue
                            ))
                        } else {
                            None
                        }
                    }
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
                        let mut builder = Vec::new();
                        let mut indentation = 1usize;

                        for cmd in &object.commands {
                            builder.push(format!("{}{cmd}", " ".repeat(indentation)));

                            if let CommandProperties::Loop { commands, .. }
                            | CommandProperties::Trigger { commands, .. } = &cmd.properties
                            {
                                if commands.is_empty() {
                                    continue;
                                }

                                let starting_indentation = indentation;
                                indentation += 1;

                                let mut current_cmds = commands;
                                let mut current_index = 0;
                                // stack of commands, index, and indentation
                                let mut cmds_stack = Vec::new();

                                loop {
                                    let cmd = &current_cmds[current_index];
                                    current_index += 1;

                                    builder.push(format!("{}{cmd}", " ".repeat(indentation)));
                                    match &cmd.properties {
                                        CommandProperties::Loop { commands, .. }
                                        | CommandProperties::Trigger { commands, .. }
                                            if !commands.is_empty() =>
                                        {
                                            // save the current cmds and index
                                            // ignore if index is already at the end of the current cmds
                                            if current_index < current_cmds.len() {
                                                cmds_stack.push((
                                                    current_cmds,
                                                    current_index,
                                                    indentation,
                                                ));
                                            }

                                            current_cmds = commands;
                                            current_index = 0;
                                            indentation += 1;
                                        }
                                        _ => {
                                            if current_index >= current_cmds.len() {
                                                // check for end of commands
                                                match cmds_stack.pop() {
                                                    Some((
                                                        last_cmds,
                                                        last_index,
                                                        last_indentation,
                                                    )) => {
                                                        current_cmds = last_cmds;
                                                        current_index = last_index;
                                                        indentation = last_indentation;
                                                    }
                                                    None => break,
                                                }
                                            }
                                        }
                                    }
                                }

                                indentation = starting_indentation;
                            }
                        }

                        Some(builder.join("\n"))
                    }
                };

                match cmds {
                    Some(cmds) => Some(format!("{object_str}\n{cmds}")),
                    None => Some(object_str),
                }
            }
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Background {
    pub filename: PathBuf,
    pub position: Option<Position>,
}

impl Background {
    pub fn new(filename: &Path, position: Option<Position>) -> Self {
        Self {
            filename: filename.to_path_buf(),
            position,
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Video {
    pub filename: PathBuf,
    pub position: Option<Position>,
    short_hand: bool,
}

impl Video {
    pub fn new(filename: PathBuf, position: Option<Position>) -> Self {
        Self {
            filename,
            position,
            short_hand: true,
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Break {
    pub end_time: Integer,
    short_hand: bool,
}

impl Break {
    pub fn new(end_time: Integer) -> Self {
        Self {
            end_time,
            short_hand: true,
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ColourTransformation {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[non_exhaustive]
pub enum EventParams {
    Background(Background),
    Video(Video),
    Break(Break),
    ColourTransformation(ColourTransformation),
}

impl VersionedFromString for EventParams {
    type ParseError = EventParamsParseError;

    fn from_str(s: &str, version: usize) -> std::result::Result<Option<Self>, Self::ParseError> {
        let start_time = comma_field;
        let file_name = || comma_field().map(PathBuf::from);
        let coordinates = || {
            alt((
                eof.map(|_| None),
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
                ))
                .map(|(x, y)| Some((x, y))),
            ))
            .map(|position| position.map(|(x, y)| Position { x, y }))
        };
        let file_name_and_coordinates = || tuple((file_name(), coordinates()));
        let end_time = consume_rest_type().map(|end_time| {
            if (3..=4).contains(&version) {
                end_time + OLD_VERSION_TIME_OFFSET
            } else {
                end_time
            }
        });

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
        let video = tuple((
            terminated(
                alt((tag("1").map(|_| true), tag("Video").map(|_| false))),
                cut(tuple((
                    context(EventParamsParseError::MissingStartTime.into(), comma()),
                    start_time(),
                    context(EventParamsParseError::MissingFileName.into(), comma()),
                ))),
            ),
            cut(file_name_and_coordinates()),
        ))
        .map(|(short_hand, (filename, position))| {
            EventParams::Video(Video {
                filename,
                position,
                short_hand,
            })
        });
        let break_ = tuple((
            terminated(
                alt((tag("2").map(|_| true), tag("Break").map(|_| false))),
                cut(tuple((
                    context(EventParamsParseError::MissingStartTime.into(), comma()),
                    start_time(),
                    context(EventParamsParseError::MissingEndTime.into(), comma()),
                ))),
            ),
            cut(context(
                EventParamsParseError::InvalidEndTime.into(),
                end_time,
            )),
        ))
        .map(|(short_hand, end_time)| {
            EventParams::Break(Break {
                end_time,
                short_hand,
            })
        });
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
                    context(
                        EventParamsParseError::InvalidGreen.into(),
                        comma_field_type(),
                    ),
                ),
                preceded(
                    context(EventParamsParseError::MissingBlue.into(), comma()),
                    context(
                        EventParamsParseError::InvalidBlue.into(),
                        consume_rest_type(),
                    ),
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

        Ok(Some(result.1))
    }
}
