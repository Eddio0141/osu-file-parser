use std::path::{Path, PathBuf};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{cut, fail};
use nom::error::context;
use nom::sequence::{preceded, tuple};
use nom::Parser;

use crate::osu_file::{
    FilePath, Position, Version, VersionedDefault, VersionedFromStr, VersionedToString,
};
use crate::parsers::{
    comma, comma_field, comma_field_type, comma_field_versioned_type, consume_rest_versioned_type,
    nothing,
};

use super::cmds::*;
use super::error::*;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[non_exhaustive]
pub enum Layer {
    Background,
    Fail,
    Pass,
    Foreground,
}

impl VersionedFromStr for Layer {
    type Err = ParseLayerError;

    fn from_str(s: &str, _: Version) -> std::result::Result<Option<Self>, Self::Err> {
        match s {
            "Background" => Ok(Some(Layer::Background)),
            "Fail" => Ok(Some(Layer::Fail)),
            "Pass" => Ok(Some(Layer::Pass)),
            "Foreground" => Ok(Some(Layer::Foreground)),
            _ => Err(ParseLayerError::UnknownVariant),
        }
    }
}

impl VersionedToString for Layer {
    fn to_string(&self, _: Version) -> Option<String> {
        let layer = match self {
            Layer::Background => "Background",
            Layer::Fail => "Fail",
            Layer::Pass => "Pass",
            Layer::Foreground => "Foreground",
        };

        Some(layer.to_string())
    }
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
    fn to_string(&self, version: Version) -> Option<String> {
        let pos_str = format!("{},{}", self.position.x, self.position.y);

        let object_str = match &self.object_type {
            ObjectType::Sprite(sprite) => format!(
                "Sprite,{},{},{},{}",
                self.layer.to_string(version).unwrap(),
                self.origin.to_string(version).unwrap(),
                sprite.filepath.to_string(version).unwrap(),
                pos_str
            ),
            ObjectType::Animation(anim) => {
                format!(
                    "Animation,{},{},{},{},{},{},{}",
                    self.layer.to_string(version).unwrap(),
                    self.origin.to_string(version).unwrap(),
                    anim.filepath.to_string(version).unwrap(),
                    pos_str,
                    anim.frame_count,
                    anim.frame_delay,
                    anim.loop_type.to_string(version).unwrap()
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
                    builder.push(format!(
                        "{}{}",
                        " ".repeat(indentation),
                        cmd.to_string(version).unwrap()
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
                                cmd.to_string(version).unwrap()
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
impl VersionedFromStr for Object {
    type Err = ParseObjectError;

    fn from_str(s: &str, version: Version) -> Result<Option<Self>, Self::Err> {
        let layer = || {
            preceded(
                context(ParseObjectError::MissingLayer.into(), comma()),
                context(
                    ParseObjectError::InvalidLayer.into(),
                    comma_field_versioned_type(version),
                ),
            )
        };
        let origin = || {
            preceded(
                context(ParseObjectError::MissingOrigin.into(), comma()),
                context(
                    ParseObjectError::InvalidOrigin.into(),
                    comma_field_versioned_type(version),
                ),
            )
        };
        let file_path = || {
            preceded(
                context(ParseObjectError::MissingFilePath.into(), comma()),
                comma_field(),
            )
            .map(|p| p.into())
        };
        let position = || {
            tuple((
                preceded(
                    context(ParseObjectError::MissingPositionX.into(), comma()),
                    context(
                        ParseObjectError::InvalidPositionX.into(),
                        comma_field_type(),
                    ),
                ),
                preceded(
                    context(ParseObjectError::MissingPositionY.into(), comma()),
                    context(
                        ParseObjectError::InvalidPositionY.into(),
                        comma_field_type(),
                    ),
                ),
            ))
            .map(|(x, y)| Position { x, y })
        };
        let frame_count = preceded(
            context(ParseObjectError::MissingFrameCount.into(), comma()),
            context(
                ParseObjectError::InvalidFrameCount.into(),
                comma_field_type(),
            ),
        );
        let frame_delay = preceded(
            context(ParseObjectError::MissingFrameDelay.into(), comma()),
            context(
                ParseObjectError::InvalidFrameDelay.into(),
                comma_field_type(),
            ),
        );
        let loop_type = alt((
            preceded(
                context(ParseObjectError::MissingLoopType.into(), comma()),
                context(
                    ParseObjectError::InvalidLoopType.into(),
                    consume_rest_versioned_type(version),
                ),
            ),
            nothing().map(|_| LoopType::default(version).unwrap()),
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
            context(ParseObjectError::UnknownObjectType.into(), fail),
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

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
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

impl VersionedToString for Origin {
    fn to_string(&self, _: Version) -> Option<String> {
        let origin = match self {
            Origin::TopLeft => "TopLeft",
            Origin::Centre => "Centre",
            Origin::CentreLeft => "CentreLeft",
            Origin::TopRight => "TopRight",
            Origin::BottomCentre => "BottomCentre",
            Origin::TopCentre => "TopCentre",
            Origin::Custom => "Custom",
            Origin::CentreRight => "CentreRight",
            Origin::BottomLeft => "BottomLeft",
            Origin::BottomRight => "BottomRight",
        };

        Some(origin.to_string())
    }
}

impl VersionedFromStr for Origin {
    type Err = ParseOriginError;

    fn from_str(s: &str, _: Version) -> std::result::Result<Option<Self>, Self::Err> {
        match s {
            "TopLeft" => Ok(Some(Origin::TopLeft)),
            "Centre" => Ok(Some(Origin::Centre)),
            "CentreLeft" => Ok(Some(Origin::CentreLeft)),
            "TopRight" => Ok(Some(Origin::TopRight)),
            "BottomCentre" => Ok(Some(Origin::BottomCentre)),
            "TopCentre" => Ok(Some(Origin::TopCentre)),
            "Custom" => Ok(Some(Origin::Custom)),
            "CentreRight" => Ok(Some(Origin::CentreRight)),
            "BottomLeft" => Ok(Some(Origin::BottomLeft)),
            "BottomRight" => Ok(Some(Origin::BottomRight)),
            _ => Err(ParseOriginError::UnknownVariant),
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[non_exhaustive]
pub enum LoopType {
    LoopForever,
    LoopOnce,
}

impl VersionedToString for LoopType {
    fn to_string(&self, _: Version) -> Option<String> {
        let loop_type = match self {
            LoopType::LoopForever => "LoopForever",
            LoopType::LoopOnce => "LoopOnce",
        };

        Some(loop_type.to_string())
    }
}

impl VersionedFromStr for LoopType {
    type Err = ParseLoopTypeError;

    fn from_str(s: &str, _: Version) -> std::result::Result<Option<Self>, Self::Err> {
        match s {
            "LoopForever" => Ok(Some(LoopType::LoopForever)),
            "LoopOnce" => Ok(Some(LoopType::LoopOnce)),
            _ => Err(ParseLoopTypeError::UnknownVariant),
        }
    }
}

// TODO all default to versioned default
impl VersionedDefault for LoopType {
    fn default(_: Version) -> Option<Self> {
        Some(Self::LoopForever)
    }
}
