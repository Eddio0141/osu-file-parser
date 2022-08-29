pub mod audio_sample;
pub mod error;
pub mod normal_event;
pub mod storyboard;

use nom::branch::alt;
use nom::combinator::{cut, eof, peek, success};
use nom::sequence::tuple;
use nom::Parser;
use nom::{bytes::complete::tag, combinator::rest, sequence::preceded};

use crate::events::storyboard::cmds::CommandProperties;
use crate::helper::trait_ext::MapOptStringNewLine;
use crate::osb::Variable;
use crate::parsers::comma;

use self::storyboard::cmds::Command;
use self::storyboard::error::CommandPushError;
use self::storyboard::{error::ParseObjectError, sprites::Object};

use super::Version;
use super::{types::Error, Integer, VersionedDefault, VersionedFromStr, VersionedToString};

pub use audio_sample::*;
pub use error::*;
pub use normal_event::*;

#[derive(Default, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Events(pub Vec<Event>);

const OLD_VERSION_TIME_OFFSET: Integer = 24;

impl VersionedFromStr for Events {
    type Err = Error<ParseError>;

    fn from_str(s: &str, version: Version) -> std::result::Result<Option<Self>, Self::Err> {
        Events::from_str_variables(s, version, &[])
    }
}

impl Events {
    pub fn from_str_variables(
        s: &str,
        version: Version,
        variables: &[Variable],
    ) -> std::result::Result<Option<Self>, Error<ParseError>> {
        let mut events = Events(Vec::new());

        #[derive(Clone)]
        enum NormalEventType {
            Background,
            Video,
            Break,
            ColourTransformation,
            Event4,
            Other,
        }

        let mut comment = preceded::<_, _, _, nom::error::Error<_>, _, _>(tag("//"), rest);
        let background = || {
            peek(tuple((
                tag::<_, _, nom::error::Error<_>>(normal_event::BACKGROUND_HEADER),
                cut(alt((eof, comma()))),
            )))
            .map(|_| NormalEventType::Background)
        };
        let video = || {
            peek(tuple((
                alt((
                    tag(normal_event::VIDEO_HEADER),
                    tag(normal_event::VIDEO_HEADER_LONG),
                )),
                cut(alt((eof, comma()))),
            )))
            .map(|_| NormalEventType::Video)
        };
        let break_ = || {
            peek(tuple((
                alt((
                    tag(normal_event::BREAK_HEADER),
                    tag(normal_event::BREAK_HEADER_LONG),
                )),
                cut(alt((eof, comma()))),
            )))
            .map(|_| NormalEventType::Break)
        };
        let colour_transformation = || {
            peek(tuple((
                tag(normal_event::COLOUR_TRANSFORMATION_HEADER),
                cut(alt((eof, comma()))),
            )))
            .map(|_| NormalEventType::ColourTransformation)
        };
        let event4 = || {
            peek(tuple((
                tag(normal_event::EVENT4_HEADER),
                cut(alt((eof, comma()))),
            )))
            .map(|_| NormalEventType::Event4)
        };

        for (line_index, line) in s.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }

            if let Ok((_, comment)) = comment(line) {
                events.0.push(Event::Comment(comment.to_string()));
                continue;
            }

            let indent = line.chars().take_while(|c| *c == ' ' || *c == '_').count();

            // its a storyboard command
            if indent > 0 {
                let cmd_parse = || {
                    let line_without_header = match line.chars().position(|c| c == ',') {
                        Some(i) => &line[i + 1..],
                        None => line,
                    };

                    let mut line_with_variable: Option<String> = None;
                    for variable in variables {
                        let variable_full = format!("${}", variable.name);

                        if line_without_header.contains(&variable_full) {
                            let new_line = match line_with_variable {
                                Some(line_with_variable) => {
                                    line_with_variable.replace(&variable_full, &variable.value)
                                }
                                None => line.replace(&variable_full, &variable.value),
                            };

                            line_with_variable = Some(new_line);
                        }
                    }

                    match line_with_variable {
                        Some(line_with_variable) => Error::new_from_result_into(
                            Command::from_str(&line_with_variable, version),
                            line_index,
                        ),
                        None => Error::new_from_result_into(
                            Command::from_str(line, version),
                            line_index,
                        ),
                    }
                };

                match events.0.last_mut() {
                    Some(event) => match event {
                        Event::Background(bg) => {
                            if let Some(cmd) = cmd_parse()? {
                                Error::new_from_result_into(
                                    bg.try_push_cmd(cmd, indent),
                                    line_index,
                                )?
                            }
                        }
                        Event::Video(video) => {
                            if let Some(cmd) = cmd_parse()? {
                                Error::new_from_result_into(
                                    video.try_push_cmd(cmd, indent),
                                    line_index,
                                )?
                            }
                        }
                        Event::Event4(event4) => {
                            if let Some(cmd) = cmd_parse()? {
                                Error::new_from_result_into(
                                    event4.try_push_cmd(cmd, indent),
                                    line_index,
                                )?
                            }
                        }
                        Event::StoryboardObject(obj) => {
                            if let Some(cmd) = cmd_parse()? {
                                Error::new_from_result_into(
                                    obj.try_push_cmd(cmd, indent),
                                    line_index,
                                )?
                            }
                        }
                        _ => {
                            return Err(Error::new(
                                ParseError::StoryboardCmdWithNoSprite,
                                line_index,
                            ))
                        }
                    },
                    _ => {
                        return Err(Error::new(
                            ParseError::StoryboardCmdWithNoSprite,
                            line_index,
                        ))
                    }
                }
                continue;
            }

            // normal event trying
            let (_, type_) = alt((
                background(),
                video(),
                break_(),
                colour_transformation(),
                event4(),
                success(NormalEventType::Other),
            ))(line)
            .unwrap();

            let res = match type_ {
                NormalEventType::Background => Background::from_str(line, version)
                    .map(|e| e.map(Event::Background))
                    .map_err(ParseError::ParseBackgroundError),
                NormalEventType::Video => Video::from_str(line, version)
                    .map(|e| e.map(Event::Video))
                    .map_err(ParseError::ParseVideoError),
                NormalEventType::Break => Break::from_str(line, version)
                    .map(|e| e.map(Event::Break))
                    .map_err(ParseError::ParseBreakError),
                NormalEventType::ColourTransformation => {
                    ColourTransformation::from_str(line, version)
                        .map(|e| e.map(Event::ColourTransformation))
                        .map_err(ParseError::ParseColourTransformationError)
                }
                NormalEventType::Event4 => Event4::from_str(line, version)
                    .map(|e| e.map(Event::Event4))
                    .map_err(ParseError::ParseEvent4Error),
                NormalEventType::Other => {
                    // is it a storyboard object?
                    match Object::from_str(line, version) {
                        Ok(e) => Ok(e.map(Event::StoryboardObject)),
                        Err(err) => {
                            if let ParseObjectError::UnknownObjectType = err {
                                // try AudioSample
                                AudioSample::from_str(line, version)
                                    .map(|e| e.map(Event::AudioSample))
                                    .map_err(|e| {
                                        if let ParseAudioSampleError::WrongEvent = e {
                                            ParseError::UnknownEventType
                                        } else {
                                            ParseError::ParseAudioSampleError(e)
                                        }
                                    })
                            } else {
                                Err(ParseError::ParseStoryboardObjectError(err))
                            }
                        }
                    }
                }
            };

            match res {
                Ok(event) => {
                    if let Some(event) = event {
                        events.0.push(event)
                    }
                }
                Err(e) => return Err(Error::new(e, line_index)),
            }
        }

        Ok(Some(events))
    }

    pub fn to_string_variables(
        &self,
        version: Version,
        variables: &[Variable],
    ) -> Option<String> {
        let mut s = self
            .0
            .iter()
            .map(|event| event.to_string_variables(version, variables));

        Some(s.map_string_new_line())
    }
}

impl VersionedToString for Events {
    fn to_string(&self, version: Version) -> Option<String> {
        self.to_string_variables(version, &[])
    }
}

impl VersionedDefault for Events {
    fn default(_: Version) -> Option<Self> {
        Some(Events(Vec::new()))
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[non_exhaustive]
/// All possible events types.
pub enum Event {
    Comment(String),
    Background(Background),
    Video(Video),
    Break(Break),
    ColourTransformation(ColourTransformation),
    Event4(Event4),
    StoryboardObject(Object),
    AudioSample(AudioSample),
}

impl VersionedToString for Event {
    fn to_string(&self, version: Version) -> Option<String> {
        self.to_string_variables(version, &[])
    }
}

impl Event {
    pub fn to_string_variables(&self, version: Version, variables: &[Variable]) -> Option<String> {
        match self {
            Event::Comment(comment) => Some(format!("//{comment}")),
            Event::Background(background) => background.to_string(version),
            Event::Video(video) => video.to_string(version),
            Event::Break(break_) => break_.to_string(version),
            Event::ColourTransformation(colour_trans) => colour_trans.to_string(version),
            Event::Event4(event4) => event4.to_string(version),
            Event::StoryboardObject(object) => object.to_string_variables(version, variables),
            Event::AudioSample(audio_sample) => Some(audio_sample.to_string(version).unwrap()),
        }
    }
}

fn commands_to_string_variables(
    cmds: &[Command],
    version: Version,
    variables: &[Variable],
) -> Option<String> {
    let mut builder = Vec::new();
    let mut indentation = 1usize;

    for cmd in cmds {
        builder.push(format!(
            "{}{}",
            " ".repeat(indentation),
            cmd.to_string_variables(version, variables).unwrap()
        ));

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

                builder.push(format!(
                    "{}{}",
                    " ".repeat(indentation),
                    cmd.to_string_variables(version, variables).unwrap()
                ));
                match &cmd.properties {
                    CommandProperties::Loop { commands, .. }
                    | CommandProperties::Trigger { commands, .. }
                        if !commands.is_empty() =>
                    {
                        // save the current cmds and index
                        // ignore if index is already at the end of the current cmds
                        if current_index < current_cmds.len() {
                            cmds_stack.push((current_cmds, current_index, indentation));
                        }

                        current_cmds = commands;
                        current_index = 0;
                        indentation += 1;
                    }
                    _ => {
                        if current_index >= current_cmds.len() {
                            // check for end of commands
                            match cmds_stack.pop() {
                                Some((last_cmds, last_index, last_indentation)) => {
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

pub trait EventWithCommands {
    fn try_push_cmd(&mut self, cmd: Command, indentation: usize) -> Result<(), CommandPushError> {
        if indentation == 1 {
            // first match no loop required
            self.commands_mut().push(cmd);
            Ok(())
        } else {
            let mut last_cmd = match self.commands_mut().last_mut() {
                Some(last_cmd) => last_cmd,
                None => return Err(CommandPushError::InvalidIndentation(1, indentation)),
            };

            for i in 1..indentation {
                last_cmd = if let CommandProperties::Loop { commands, .. }
                | CommandProperties::Trigger { commands, .. } =
                    &mut last_cmd.properties
                {
                    if i + 1 == indentation {
                        // last item
                        commands.push(cmd);
                        return Ok(());
                    } else {
                        match commands.last_mut() {
                            Some(sub_cmd) => sub_cmd,
                            None => {
                                return Err(CommandPushError::InvalidIndentation(
                                    i - 1,
                                    indentation,
                                ))
                            }
                        }
                    }
                } else {
                    return Err(CommandPushError::InvalidIndentation(1, indentation));
                };
            }

            unreachable!();
        }
    }

    fn commands(&self) -> &[Command];

    fn commands_mut(&mut self) -> &mut Vec<Command>;

    /// Returns the command as a `String`.
    /// - Instead of making the command into a string using `Display` or `VersionedToString`, use this to get the command as a string.
    fn to_string_cmd(&self, version: Version) -> Option<String>;

    /// Returns the command as a `String`.
    /// - Contains the commands as a string as well.
    /// - Use this in the `to_string` method with an empty `variables` array.
    fn to_string_variables(&self, version: Version, variables: &[Variable]) -> Option<String> {
        match self.to_string_cmd(version) {
            Some(s) => {
                let cmds = match commands_to_string_variables(self.commands(), version, variables) {
                    Some(mut cmds) => {
                        if !cmds.is_empty() {
                            cmds = format!("\n{cmds}");
                        }

                        cmds
                    }
                    None => return None,
                };

                Some(format!("{s}{cmds}"))
            }
            None => None,
        }
    }
}
