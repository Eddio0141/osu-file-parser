use std::error::Error;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::str::{FromStr, Split};

use strum_macros::{Display, EnumString, FromRepr};

use crate::osu_file::{events::storyboard::CommandProperties, Position};

use super::{error::*, Command};

// TODO investivage if integer form is valid
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Display, FromRepr, EnumString)]
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
impl FromStr for Object {
    type Err = ObjectParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(',');

        let object_type = s
            .next()
            .ok_or(ObjectParseError::MissingField("object_type"))?;
        // I don't parse until the object_type is valid
        let position = |s: &mut Split<char>| {
            Ok(Position {
                x: parse_object_field(s, "x")?,
                y: parse_object_field(s, "y")?,
            })
        };

        match object_type {
            "Sprite" => {
                let layer = parse_object_field(&mut s, "layer")?;
                let origin = parse_object_field(&mut s, "origin")?;
                let filepath = parse_object_field(&mut s, "filepath")?;
                let position = position(&mut s)?;

                Ok(Object {
                    layer,
                    origin,
                    position,
                    object_type: ObjectType::Sprite(Sprite { filepath }),
                    commands: Vec::new(),
                })
            }
            "Animation" => {
                let layer = parse_object_field(&mut s, "layer")?;
                let origin = parse_object_field(&mut s, "origin")?;
                let filepath = parse_object_field(&mut s, "filepath")?;
                let position = position(&mut s)?;
                let frame_count = parse_object_field(&mut s, "frameCount")?;
                let frame_delay = parse_object_field(&mut s, "frameDelay")?;
                let loop_type = parse_object_field(&mut s, "loopType")?;

                Ok(Object {
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
                })
            }
            _ => Err(ObjectParseError::UnknownObjectType(object_type.to_string())),
        }
    }
}

fn parse_object_field<T>(
    s: &mut Split<char>,
    field_name: &'static str,
) -> Result<T, ObjectParseError>
where
    T: FromStr,
    <T as FromStr>::Err: 'static + Error,
{
    let s = s.next().ok_or(ObjectParseError::MissingField(field_name))?;
    s.parse().map_err(|err| ObjectParseError::FieldParseError {
        source: Box::new(err),
        field_name,
        value: s.to_string(),
    })
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fields = vec![self.layer.to_string(), self.origin.to_string()];

        match &self.object_type {
            ObjectType::Sprite(sprite) => {
                fields.push(sprite.filepath.to_string_lossy().to_string());
                fields.push(self.position.x.to_string());
                fields.push(self.position.y.to_string());
            }
            ObjectType::Animation(animation) => {
                fields.push(animation.filepath.to_string_lossy().to_string());
                fields.push(self.position.x.to_string());
                fields.push(self.position.y.to_string());
                fields.push(animation.frame_count.to_string());
                fields.push(animation.frame_delay.to_string());
                fields.push(animation.loop_type.to_string());
            }
        }

        write!(f, "{}", fields.join(","))
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Animation {
    // TODO what types are those counts
    pub frame_count: u32,
    pub frame_delay: u32,
    pub loop_type: LoopType,
    pub filepath: PathBuf,
}

impl Animation {
    pub fn new(frame_count: u32, frame_delay: u32, loop_type: LoopType, filepath: &Path) -> Self {
        Self {
            frame_count,
            frame_delay,
            loop_type,
            filepath: filepath.to_path_buf(),
        }
    }

    pub fn frame_file_names(&self) -> Vec<String> {
        let mut file_names = Vec::with_capacity(self.frame_count as usize);

        let file_name = self
            .filepath
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        let file_extension = self.filepath.extension().map(|s| s.to_string_lossy());

        for i in 0..self.frame_count {
            match &file_extension {
                Some(file_extension) => file_names.push(format!(
                    "{}{i}.{file_extension}",
                    file_name
                        .strip_suffix(&format!(".{}", file_extension.as_ref()))
                        .unwrap()
                )),
                None => file_names.push(format!("{file_name}{i}")),
            };
        }
        file_names
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Sprite {
    pub filepath: PathBuf,
}

impl Sprite {
    pub fn new(filepath: &Path) -> Result<Self, FilePathNotRelative> {
        if filepath.is_absolute() {
            Err(FilePathNotRelative)
        } else {
            Ok(Self {
                filepath: filepath.to_path_buf(),
            })
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum ObjectType {
    Sprite(Sprite),
    Animation(Animation),
}

// TODO investigate if integer form is valid
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Display, FromRepr, EnumString)]
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
pub enum LoopType {
    LoopForever,
    LoopOnce,
}

impl Default for LoopType {
    fn default() -> Self {
        Self::LoopForever
    }
}
