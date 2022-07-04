// TODO clean up events

pub mod audio_sample;
pub mod error;
pub mod normal_event;
pub mod storyboard;

use nom::{bytes::complete::tag, combinator::rest, sequence::preceded};

use crate::{helper::trait_ext::MapOptStringNewLine, osu_file::events::storyboard::sprites};

use self::storyboard::cmds::Command;
use self::storyboard::{cmds::CommandProperties, error::ObjectParseError, sprites::Object};

use super::{types::Error, Integer, VersionedDefault, VersionedFromString, VersionedToString};

pub use self::audio_sample::*;
pub use self::error::*;
pub use self::normal_event::*;

#[derive(Default, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Events(pub Vec<Event>);

const OLD_VERSION_TIME_OFFSET: Integer = 24;

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
                    let indent = line.chars().take_while(|c| *c == ' ' || *c == '_').count();

                    // its a storyboard command
                    if indent > 0 {
                        match events.0.last_mut() {
                            Some(Event::Storyboard(sprite)) => {
                                let cmd = Error::new_from_result_into(Command::from_str(line, version), line_index)?;
                                if let Some(cmd) = cmd {
                                    Error::new_from_result_into(
                                        sprite.try_push_cmd(
                                            cmd,
                                            indent,
                                        ),
                                        line_index,
                                    )?
                                }
                            },
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
                                    // try AudioSample
                                    match line.parse() {
                                        Ok(audio_sample) => {
                                            events.0.push(Event::AudioSample(audio_sample))
                                        }
                                        Err(_) => {
                                            // its a normal event
                                            match Background::from_str(line, version) {
                                                Ok(background) => events.0.push(Event::Background(background.ok_or_else(|| Error::new(ParseError::EventNotExistOnVersion, line_index))?)),
                                                Err(err) => {
                                                    if let BackgroundParseError::WrongEventType =
                                                        err
                                                    {
                                                        match Video::from_str(line, version) {
                                                            Ok(video) => events.0.push(Event::Video(video.ok_or_else(|| Error::new(ParseError::EventNotExistOnVersion, line_index))?)),
                                                            Err(err) => if let VideoParseError::WrongEventType = err {
                                                                match Break::from_str(line, version) {
                                                                    Ok(break_) => events.0.push(Event::Break(break_.ok_or_else(|| Error::new(ParseError::EventNotExistOnVersion, line_index))?)),
                                                                    Err(err) => if let BreakParseError::WrongEventType = err {
                                                                        match ColourTransformation::from_str(line, version) {
                                                                            Ok(colour_trans) => events.0.push(Event::ColourTransformation(colour_trans.ok_or_else(|| Error::new(ParseError::EventNotExistOnVersion, line_index))?)),
                                                                            Err(err) => {
                                                                                return Err(Error::new(
                                                                                    ParseError::ColourTransformationParseError(err),
                                                                                    line_index,
                                                                                ));
                                                                            }
                                                                        }
                                                                    }
                                                                }   
                                                            }
                                                        }
                                                    } else {
                                                        return Err(Error::new(
                                                            ParseError::BackgroundParseError(err),
                                                            line_index,
                                                        ));
                                                    }
                                                }
                                            }
                                        }
                                    }
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
        let mut s = self.0.iter().map(|i| 
            i.to_string(version)
        );

        Some(s.map_string_new_line())
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
    Background(Background),
    Video(Video),
    Break(Break),
    ColourTransformation(ColourTransformation),
    Storyboard(Object),
    AudioSample(AudioSample),
}

impl VersionedToString for Event {
    fn to_string(&self, version: usize) -> Option<String> {
        match self {
            Event::Comment(comment) => Some(format!("//{comment}")),
            Event::Background(background) => background.to_string(version),
            Event::Video(video) => video.to_string(version),
            Event::Break(break_) => break_.to_string(version),
            Event::ColourTransformation(colour_trans) => colour_trans.to_string(version),
            Event::Storyboard(object) => {
                let pos_str = format!("{},{}", object.position.x, object.position.y);

                let object_str = match &object.object_type {
                    sprites::ObjectType::Sprite(sprite) => format!(
                        "Sprite,{},{},{},{}",
                        object.layer, object.origin, sprite.filepath, pos_str
                    ),
                    sprites::ObjectType::Animation(anim) => {
                        format!(
                            "Animation,{},{},{},{},{},{},{}",
                            object.layer,
                            object.origin,
                            anim.filepath,
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
            Event::AudioSample(audio_sample) => Some(audio_sample.to_string()),
        }
    }
}
