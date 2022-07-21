pub mod audio_sample;
pub mod error;
pub mod normal_event;
pub mod storyboard;

use nom::branch::alt;
use nom::combinator::{cut, eof, peek, success};
use nom::sequence::tuple;
use nom::Parser;
use nom::{bytes::complete::tag, combinator::rest, sequence::preceded};

use crate::helper::trait_ext::MapOptStringNewLine;
use crate::osb::Variable;
use crate::parsers::comma;

use self::storyboard::cmds::Command;
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
    pub(crate) fn from_str_variables(
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
                match events.0.last_mut() {
                    Some(Event::StoryboardObject(sprite)) => {
                        let line_without_header = match line.chars().position(|c| c == ',') {
                            Some(i) => &line[i + 1..],
                            None => line,
                        };

                        let mut line_with_variable = None;
                        for variable in variables {
                            let variable_full = format!("${}", variable.name);

                            if line_without_header.contains(&variable_full) {
                                line_with_variable =
                                    Some(line.replace(&variable_full, &variable.value));
                                break;
                            }
                        }

                        let cmd = match line_with_variable {
                            Some(line_with_variable) => Error::new_from_result_into(
                                Command::from_str(&line_with_variable, version),
                                line_index,
                            )?,
                            None => Error::new_from_result_into(
                                Command::from_str(line, version),
                                line_index,
                            )?,
                        };
                        if let Some(cmd) = cmd {
                            Error::new_from_result_into(
                                sprite.try_push_cmd(cmd, indent),
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
                }
                continue;
            }

            // normal event trying
            let (_, type_) = alt((
                background(),
                video(),
                break_(),
                colour_transformation(),
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
                NormalEventType::Other => {
                    // is it a storyboard object?
                    match Object::from_str(line, version) {
                        Ok(e) => Ok(e.map(Event::StoryboardObject)),
                        Err(err) => {
                            if let ParseObjectError::UnknownObjectType = err {
                                // try AudioSample
                                AudioSample::from_str(line, version)
                                    .map(|e| e.map(Event::AudioSample))
                                    .map_err(ParseError::ParseAudioSampleError)
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
}

impl VersionedToString for Events {
    fn to_string(&self, version: Version) -> Option<String> {
        let mut s = self.0.iter().map(|i| i.to_string(version));

        Some(s.map_string_new_line())
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
    StoryboardObject(Object),
    AudioSample(AudioSample),
}

impl VersionedToString for Event {
    fn to_string(&self, version: Version) -> Option<String> {
        match self {
            Event::Comment(comment) => Some(format!("//{comment}")),
            Event::Background(background) => background.to_string(version),
            Event::Video(video) => video.to_string(version),
            Event::Break(break_) => break_.to_string(version),
            Event::ColourTransformation(colour_trans) => colour_trans.to_string(version),
            Event::StoryboardObject(object) => object.to_string(version),
            Event::AudioSample(audio_sample) => Some(audio_sample.to_string(version).unwrap()),
        }
    }
}
