use std::path::{Path, PathBuf};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{cut, fail};
use nom::error::context;
use nom::sequence::{preceded, tuple};
use nom::Parser;
use strum_macros::{Display, EnumString, FromRepr};

use crate::osu_file::{FilePath, Position, VersionedFromString, VersionedToString};
use crate::parsers::{comma, comma_field, comma_field_type, consume_rest_type, nothing};

use super::cmds::*;
use super::error::*;

// TODO investivage if integer form is valid
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Display, FromRepr, EnumString)]
#[non_exhaustive]
pub enum Layer {
    Background,
    Fail,
    Pass,
    Foreground,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Object {
    pub layer: Layer,
    pub origin: Origin,
    pub position: Position,
    pub object_type: ObjectType,
    pub commands: Vec<Command>,
}

impl VersionedToString for Object {
    fn to_string(&self, _: usize) -> Option<String> {
        let pos_str = format!("{},{}", self.position.x, self.position.y);

        let object_str = match &self.object_type {
            ObjectType::Sprite(sprite) => format!(
                "Sprite,{},{},{},{}",
                self.layer, self.origin, sprite.filepath, pos_str
            ),
            ObjectType::Animation(anim) => {
                format!(
                    "Animation,{},{},{},{},{},{},{}",
                    self.layer,
                    self.origin,
                    anim.filepath,
                    pos_str,
                    anim.frame_count,
                    anim.frame_delay,
                    anim.loop_type
                )
            }
        };

        let cmds = {
            if self.commands.is_empty() {
                None
            } else {
                let mut builder = Vec::new();
                let mut indentation = 1usize;

                for cmd in &self.commands {
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
        };

        match cmds {
            Some(cmds) => Some(format!("{object_str}\n{cmds}")),
            None => Some(object_str),
        }
    }
}

impl Object {
    pub fn try_push_cmd(
        &mut self,
        cmd: Command,
        indentation: usize,
    ) -> Result<(), CommandPushError> {
        if indentation == 1 {
            // first match no loop required
            self.commands.push(cmd);
            Ok(())
        } else {
            let mut last_cmd = match self.commands.last_mut() {
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
}

// it will reject commands since push_cmd is used for that case
impl VersionedFromString for Object {
    type ParseError = ObjectParseError;

    fn from_str(s: &str, _: usize) -> Result<Option<Self>, Self::ParseError> {
        let layer = || {
            preceded(
                context(ObjectParseError::MissingLayer.into(), comma()),
                context(ObjectParseError::InvalidLayer.into(), comma_field_type()),
            )
        };
        let origin = || {
            preceded(
                context(ObjectParseError::MissingOrigin.into(), comma()),
                context(ObjectParseError::InvalidOrigin.into(), comma_field_type()),
            )
        };
        let file_path = || {
            preceded(
                context(ObjectParseError::MissingFilePath.into(), comma()),
                comma_field(),
            )
            .map(|p| p.into())
        };
        let position = || {
            tuple((
                preceded(
                    context(ObjectParseError::MissingPositionX.into(), comma()),
                    context(
                        ObjectParseError::InvalidPositionX.into(),
                        comma_field_type(),
                    ),
                ),
                preceded(
                    context(ObjectParseError::MissingPositionY.into(), comma()),
                    context(
                        ObjectParseError::InvalidPositionY.into(),
                        comma_field_type(),
                    ),
                ),
            ))
            .map(|(x, y)| Position { x, y })
        };
        let frame_count = preceded(
            context(ObjectParseError::MissingFrameCount.into(), comma()),
            context(
                ObjectParseError::InvalidFrameCount.into(),
                comma_field_type(),
            ),
        );
        let frame_delay = preceded(
            context(ObjectParseError::MissingFrameDelay.into(), comma()),
            context(
                ObjectParseError::InvalidFrameDelay.into(),
                comma_field_type(),
            ),
        );
        let loop_type = alt((
            preceded(
                context(ObjectParseError::MissingLoopType.into(), comma()),
                context(
                    ObjectParseError::InvalidLoopType.into(),
                    consume_rest_type(),
                ),
            ),
            nothing().map(|_| LoopType::default()),
        ));

        let (_, object) = alt((
            preceded(
                tag("Sprite"),
                cut(tuple((layer(), origin(), file_path(), position()))).map(
                    |(layer, origin, filepath, position)| Object {
                        layer,
                        origin,
                        position,
                        object_type: ObjectType::Sprite(Sprite { filepath }),
                        commands: Vec::new(),
                    },
                ),
            ),
            preceded(
                tag("Animation"),
                cut(tuple((
                    layer(),
                    origin(),
                    file_path(),
                    position(),
                    frame_count,
                    frame_delay,
                    loop_type,
                ))),
            )
            .map(
                |(layer, origin, filepath, position, frame_count, frame_delay, loop_type)| Object {
                    layer,
                    origin,
                    position,
                    object_type: ObjectType::Animation(Animation {
                        frame_count,
                        frame_delay,
                        loop_type,
                        filepath,
                    }),
                    commands: Vec::new(),
                },
            ),
            context(ObjectParseError::UnknownObjectType.into(), fail),
        ))(s)?;

        Ok(Some(object))
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Animation {
    pub frame_count: u32,
    pub frame_delay: u32,
    pub loop_type: LoopType,
    pub filepath: FilePath,
}

impl Animation {
    pub fn frame_file_names(&self) -> Vec<PathBuf> {
        let mut file_names = Vec::with_capacity(self.frame_count as usize);

        let filepath = self.filepath.get();
        let mut file_name = filepath.file_name().unwrap().to_string_lossy().to_string();
        let file_extension = match filepath.extension() {
            Some(extension) => {
                let extension = format!(".{}", extension.to_string_lossy());
                file_name = file_name.strip_suffix(&extension).unwrap().to_string();
                extension
            }
            None => String::new(),
        };

        for i in 0..self.frame_count {
            file_names.push(format!("{file_name}{i}{file_extension}").into());
        }

        file_names
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Sprite {
    pub filepath: FilePath,
}

impl Sprite {
    pub fn new(filepath: &Path) -> Result<Self, FilePathNotRelative> {
        if filepath.is_absolute() {
            Err(FilePathNotRelative)
        } else {
            Ok(Self {
                filepath: filepath.into(),
            })
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[non_exhaustive]
pub enum ObjectType {
    Sprite(Sprite),
    Animation(Animation),
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Display, FromRepr, EnumString)]
#[non_exhaustive]
pub enum Origin {
    TopLeft,
    Centre,
    CentreLeft,
    TopRight,
    BottomCentre,
    TopCentre,
    Custom,
    CentreRight,
    BottomLeft,
    BottomRight,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Display, EnumString)]
#[non_exhaustive]
pub enum LoopType {
    LoopForever,
    LoopOnce,
}

impl Default for LoopType {
    fn default() -> Self {
        Self::LoopForever
    }
}
