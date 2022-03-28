use std::{
    error::Error,
    fmt::Display,
    num::ParseIntError,
    path::{Path, PathBuf},
    str::{FromStr, Split},
};

use rust_decimal::Decimal;
use strum_macros::{Display, EnumString, FromRepr, IntoStaticStr};
use thiserror::Error;

use crate::osu_file::{Integer, Position};

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

#[derive(Debug, Error)]
pub enum CommandPushError {
    #[error("Invalid indentation, expected {0}, got {1}")]
    InvalidIndentation(usize, usize),
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

#[derive(Debug, Error)]
pub enum ObjectParseError {
    #[error("Unknown object type {0}")]
    UnknownObjectType(String),
    #[error("The object is missing the field {0}")]
    MissingField(&'static str),
    #[error("The field {field_name} failed to parse from a `str` to a type")]
    FieldParseError {
        #[source]
        source: Box<dyn Error>,
        field_name: &'static str,
        value: String,
    },
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

#[derive(Debug, Error)]
#[error("The filepath needs to be a path relative to where the .osu file is, not a full path such as `C:\\folder\\image.png`")]
pub struct FilePathNotRelative;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum ObjectType {
    Sprite(Sprite),
    Animation(Animation),
}

// TODO investivage if integer form is valid
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Display, FromRepr, EnumString)]
pub enum Layer {
    Background,
    Fail,
    Pass,
    Foreground,
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

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Command {
    pub start_time: Integer,
    pub properties: CommandProperties,
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cmd_str = match &self.properties {
            CommandProperties::Fade {
                easing,
                end_time,
                start_opacity,
                end_opacity,
            } => format!(
                "F,{},{},{end_time},{start_opacity},{end_opacity}",
                *easing as usize, self.start_time
            ),
            CommandProperties::Move {
                easing,
                end_time,
                start_x,
                start_y,
                end_x,
                end_y,
            } => format!(
                "M,{},{},{end_time},{start_x},{start_y},{end_x},{end_y}",
                *easing as usize, self.start_time
            ),
            CommandProperties::MoveX {
                easing,
                end_time,
                start_x,
                end_x,
            } => format!(
                "MX,{},{},{end_time},{start_x},{end_x}",
                *easing as usize, self.start_time
            ),
            CommandProperties::MoveY {
                easing,
                end_time,
                start_y,
                end_y,
            } => format!(
                "MY,{},{},{end_time},{start_y},{end_y}",
                *easing as usize, self.start_time
            ),
            CommandProperties::Scale {
                easing,
                end_time,
                start_scale,
                end_scale,
            } => format!(
                "S,{},{},{end_time},{start_scale},{end_scale}",
                *easing as usize, self.start_time
            ),
            CommandProperties::VectorScale {
                easing,
                end_time,
                start_scale_x,
                start_scale_y,
                end_scale_x,
                end_scale_y,
            } => format!(
                "V,{},{},{end_time},{start_scale_x},{start_scale_y},{end_scale_x},{end_scale_y}",
                *easing as usize, self.start_time
            ),
            CommandProperties::Rotate {
                easing,
                end_time,
                start_rotate,
                end_rotate,
            } => format!(
                "R,{},{},{end_time},{start_rotate},{end_rotate}",
                *easing as usize, self.start_time
            ),
            CommandProperties::Colour {
                easing,
                end_time,
                start_r,
                start_g,
                start_b,
                end_r,
                end_g,
                end_b,
            } => format!(
                "C,{},{},{end_time},{start_r},{start_g},{start_b},{end_r},{end_g},{end_b}",
                *easing as usize, self.start_time
            ),
            CommandProperties::Parameter {
                easing,
                end_time,
                parameter,
            } => format!(
                "P,{},{},{end_time},{parameter}",
                *easing as usize, self.start_time
            ),
            CommandProperties::Loop {
                loop_count,
                // ignore commands since its handled separately
                commands: _,
            } => format!("L,{},{loop_count}", self.start_time),
            CommandProperties::Trigger {
                trigger_type,
                end_time,
                group_number,
                // ignore commands since its handled separately
                commands: _,
            } => format!(
                "T,{trigger_type},{},{end_time}{}",
                self.start_time,
                match group_number {
                    Some(group_number) => format!(",{group_number}"),
                    None => String::new(),
                }
            ),
        };

        write!(f, "{cmd_str}")
    }
}

// TODO make most enums non-exhaustive
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[non_exhaustive]
pub enum CommandProperties {
    Fade {
        easing: Easing,
        end_time: Option<Integer>,
        opacities: Vec<Opacities>,
    },
    Move {
        easing: Easing,
        end_time: Option<Integer>,
        positions_xy: Vec<PositionsXY>,
    },
    MoveX {
        easing: Easing,
        end_time: Option<Integer>,
        positions_x: Vec<PositionsX>,
    },
    MoveY {
        easing: Easing,
        end_time: Option<Integer>,
        positions_y: Vec<PositionsY>,
    },
    Scale {
        easing: Easing,
        end_time: Option<Integer>,
        scales: Vec<Scales>,
    },
    VectorScale {
        easing: Easing,
        end_time: Option<Integer>,
        scales_xy: Vec<ScalesXY>,
    },
    Rotate {
        easing: Easing,
        end_time: Option<Integer>,
        rotations: Vec<Rotations>,
    },
    Colour {
        easing: Easing,
        end_time: Option<Integer>,
        rgbs: Vec<Rgbs>,
    },
    Parameter {
        easing: Easing,
        end_time: Option<Integer>,
        parameter: Vec<Parameter>,
    },
    Loop {
        loop_count: u32,
        commands: Vec<Command>,
    },
    Trigger {
        trigger_type: TriggerType,
        end_time: Option<Integer>,
        // TODO find out if negative group numbers are fine
        group_number: Option<Integer>,
        commands: Vec<Command>,
    },
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Opacities {
    pub start: Decimal,
    pub end: Option<Decimal>,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct PositionsXY {
    pub start: (Decimal, Decimal),
    pub end: (Decimal, Option<Decimal>),
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct PositionsX {
    pub start: Decimal,
    pub end: Option<Decimal>,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct PositionsY {
    pub start: Decimal,
    pub end: Option<Decimal>,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Scales {
    pub start: Decimal,
    pub end: Option<Decimal>,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ScalesXY {
    pub start: (Decimal, Decimal),
    pub end: (Decimal, Option<Decimal>),
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Rotations {
    pub start: Decimal,
    pub end: Option<Decimal>,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Rgbs {
    pub start: (u8, u8, u8),
    pub end: (Option<u8>, Option<u8>, Option<u8>),
}

impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(',');

        let event = s
            .next()
            .ok_or(CommandParseError::MissingField("event"))?
            .trim_matches(|c| c == ' ' || c == '_');

        let easing_parse = |s: &mut Split<char>| {
            let s = s.next().ok_or(CommandParseError::MissingField("easing"))?;
            let s = s
                .parse()
                .map_err(|err| CommandParseError::FieldParseError {
                    source: Box::new(err),
                    value: s.to_string(),
                })?;
            Easing::from_repr(s).ok_or(CommandParseError::InvalidEasing(s))
        };
        let start_time_parse = |s: &mut Split<char>| {
            let s = s
                .next()
                .ok_or(CommandParseError::MissingField("start_time"))?;
            s.parse().map_err(|err| CommandParseError::FieldParseError {
                source: Box::new(err),
                value: s.to_string(),
            })
        };
        let end_time_parse = |s: &mut Split<char>, start_time| {
            let s = s
                .next()
                .ok_or(CommandParseError::MissingField("end_time"))?;

            if s.trim().is_empty() {
                Ok(start_time)
            } else {
                s.parse().map_err(|err| CommandParseError::FieldParseError {
                    source: Box::new(err),
                    value: s.to_string(),
                })
            }
        };

        let decimal_parse = |s: &mut Split<char>, field_name| {
            let s = s
                .next()
                .ok_or(CommandParseError::MissingField(field_name))?;
            s.parse().map_err(|err| CommandParseError::FieldParseError {
                source: Box::new(err),
                value: s.to_string(),
            })
        };
        let decimal_optional_parse = |s: &mut Split<char>, none_value| {
            let s = s.next();

            match s {
                Some(s) => s.parse().map_err(|err| CommandParseError::FieldParseError {
                    source: Box::new(err),
                    value: s.to_string(),
                }),
                None => Ok(none_value),
            }
        };
        let u8_parse = |s: &mut Split<char>, field_name| {
            let s = s
                .next()
                .ok_or(CommandParseError::MissingField(field_name))?;
            s.parse().map_err(|err| CommandParseError::FieldParseError {
                source: Box::new(err),
                value: s.to_string(),
            })
        };

        match event {
            "F" => {
                let easing = easing_parse(&mut s)?;
                let start_time = start_time_parse(&mut s)?;
                let end_time = end_time_parse(&mut s, start_time)?;
                let start_opacity = decimal_parse(&mut s, "start_opacity")?;

                Ok(Command {
                    start_time,
                    properties: CommandProperties::Fade {
                        easing,
                        end_time,
                        start_opacity,
                        end_opacity: decimal_optional_parse(&mut s, start_opacity)?,
                    },
                })
            }
            "M" => {
                let easing = easing_parse(&mut s)?;
                let start_time = start_time_parse(&mut s)?;

                Ok(Command {
                    start_time,
                    properties: CommandProperties::Move {
                        easing,
                        end_time: end_time_parse(&mut s, start_time)?,
                        start_x: decimal_parse(&mut s, "start_x")?,
                        start_y: decimal_parse(&mut s, "start_y")?,
                        end_x: decimal_parse(&mut s, "end_x")?,
                        end_y: decimal_parse(&mut s, "end_y")?,
                    },
                })
            }
            "MX" => {
                let easing = easing_parse(&mut s)?;
                let start_time = start_time_parse(&mut s)?;

                Ok(Command {
                    start_time,
                    properties: CommandProperties::MoveX {
                        easing,
                        end_time: end_time_parse(&mut s, start_time)?,
                        start_x: decimal_parse(&mut s, "start_x")?,
                        end_x: decimal_parse(&mut s, "end_x")?,
                    },
                })
            }
            "MY" => {
                let easing = easing_parse(&mut s)?;
                let start_time = start_time_parse(&mut s)?;

                Ok(Command {
                    start_time,
                    properties: CommandProperties::MoveY {
                        easing,
                        end_time: end_time_parse(&mut s, start_time)?,
                        start_y: decimal_parse(&mut s, "start_y")?,
                        end_y: decimal_parse(&mut s, "end_y")?,
                    },
                })
            }
            "S" => {
                let easing = easing_parse(&mut s)?;
                let start_time = start_time_parse(&mut s)?;
                let end_time = end_time_parse(&mut s, start_time)?;
                let start_scale = decimal_parse(&mut s, "start_scale")?;

                Ok(Command {
                    start_time,
                    properties: CommandProperties::Scale {
                        easing,
                        end_time,
                        start_scale,
                        end_scale: decimal_optional_parse(&mut s, start_scale)?,
                    },
                })
            }
            "V" => {
                let easing = easing_parse(&mut s)?;
                let start_time = start_time_parse(&mut s)?;
                let end_time = end_time_parse(&mut s, start_time)?;

                Ok(Command {
                    start_time,
                    properties: CommandProperties::VectorScale {
                        easing,
                        end_time,
                        start_scale_x: decimal_parse(&mut s, "start_scale_x")?,
                        start_scale_y: decimal_parse(&mut s, "start_scale_y")?,
                        end_scale_x: decimal_parse(&mut s, "end_scale_x")?,
                        end_scale_y: decimal_parse(&mut s, "end_scale_x")?,
                    },
                })
            }
            "R" => {
                let easing = easing_parse(&mut s)?;
                let start_time = start_time_parse(&mut s)?;
                let end_time = end_time_parse(&mut s, start_time)?;

                Ok(Command {
                    start_time,
                    properties: CommandProperties::Rotate {
                        easing,
                        end_time,
                        start_rotate: decimal_parse(&mut s, "start_rotate")?,
                        end_rotate: decimal_parse(&mut s, "end_rotate")?,
                    },
                })
            }
            "C" => {
                let easing = easing_parse(&mut s)?;
                let start_time = start_time_parse(&mut s)?;
                let end_time = end_time_parse(&mut s, start_time)?;

                Ok(Command {
                    start_time,
                    properties: CommandProperties::Colour {
                        easing,
                        end_time,
                        start_r: u8_parse(&mut s, "start_r")?,
                        start_g: u8_parse(&mut s, "start_g")?,
                        start_b: u8_parse(&mut s, "start_b")?,
                        end_r: u8_parse(&mut s, "end_r")?,
                        end_g: u8_parse(&mut s, "end_g")?,
                        end_b: u8_parse(&mut s, "end_b")?,
                    },
                })
            }
            "P" => {
                let easing = easing_parse(&mut s)?;
                let start_time = start_time_parse(&mut s)?;
                let end_time = end_time_parse(&mut s, start_time)?;

                Ok(Command {
                    start_time,
                    properties: CommandProperties::Parameter {
                        easing,
                        end_time,
                        parameter: {
                            let s = s
                                .next()
                                .ok_or(CommandParseError::MissingField("parameter"))?;
                            s.parse()
                                .map_err(|err| CommandParseError::FieldParseError {
                                    source: Box::new(err),
                                    value: s.to_string(),
                                })?
                        },
                    },
                })
            }
            "L" => Ok(Command {
                start_time: start_time_parse(&mut s)?,
                properties: CommandProperties::Loop {
                    loop_count: {
                        let s = s
                            .next()
                            .ok_or(CommandParseError::MissingField("loopcount"))?;
                        s.parse()
                            .map_err(|err| CommandParseError::FieldParseError {
                                source: Box::new(err),
                                value: s.to_string(),
                            })?
                    },
                    commands: Vec::new(),
                },
            }),
            "T" => {
                let trigger_type = {
                    let s = s
                        .next()
                        .ok_or(CommandParseError::MissingField("triggerType"))?;
                    s.parse()
                        .map_err(|err| CommandParseError::FieldParseError {
                            source: Box::new(err),
                            value: s.to_string(),
                        })?
                };
                let start_time = start_time_parse(&mut s)?;

                Ok(Command {
                    start_time,
                    properties: CommandProperties::Trigger {
                        trigger_type,
                        end_time: end_time_parse(&mut s, start_time)?,
                        group_number: {
                            let s = s.next();

                            match s {
                                Some(s) => Some(s.parse().map_err(|err| {
                                    CommandParseError::FieldParseError {
                                        source: Box::new(err),
                                        value: s.to_string(),
                                    }
                                })?),
                                None => None,
                            }
                        },
                        commands: Vec::new(),
                    },
                })
            }
            _ => Err(CommandParseError::UnknownEvent(event.to_string())),
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, IntoStaticStr, EnumString, Display)]
pub enum Parameter {
    #[strum(serialize = "H")]
    ImageFlipHorizontal,
    #[strum(serialize = "V")]
    ImageFlipVertical,
    #[strum(serialize = "A")]
    UseAdditiveColourBlending,
}

// TODO what is group_number
// TODO for all usize used in the project, but replace with something more concrete since its unrelated to hardware
// and also for nonzerousize
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum TriggerType {
    HitSound {
        sample_set: Option<SampleSet>,
        additions_sample_set: Option<SampleSet>,
        addition: Option<Addition>,
        custom_sample_set: Option<usize>,
    },
    Passing,
    Failing,
}

impl FromStr for TriggerType {
    type Err = TriggerTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        match s.strip_prefix("HitSound") {
            Some(s) => match s {
                "Passing" => Ok(TriggerType::Passing),
                "Failing" => Ok(TriggerType::Failing),
                "" => Ok(TriggerType::HitSound {
                    sample_set: None,
                    additions_sample_set: None,
                    addition: None,
                    custom_sample_set: None,
                }),
                _ => {
                    let fields = {
                        let mut fields = Vec::new();
                        let mut builder = String::with_capacity(256);

                        for (i, ch) in s.chars().enumerate() {
                            if i != 0 && (ch.is_uppercase() || ch.is_numeric()) {
                                fields.push(builder.to_owned());
                                builder.clear();
                            }
                            builder.push(ch);
                        }

                        fields.push(builder);

                        fields
                    };

                    // TODO for the project, make sure all fields are used in iterator next call
                    if fields.len() > 4 {
                        return Err(TriggerTypeParseError::TooManyHitSoundFields(fields.len()));
                    }

                    let mut field_parse_attempt_index = 0;

                    let mut sample_set = None;
                    let mut additions_sample_set = None;
                    let mut addition = None;
                    let mut custom_sample_set = None;

                    for field in fields {
                        loop {
                            match field_parse_attempt_index {
                                0 => if let Ok(field) = field.parse() {
                                    sample_set = Some(field);
                                    field_parse_attempt_index += 1;
                                    break;
                                }
                                1 => if let Ok(field) = field.parse() {
                                    additions_sample_set = Some(field);
                                    field_parse_attempt_index += 1;
                                    break;
                                }
                                2 => if let Ok(field) = field.parse() {
                                    addition = Some(field);
                                    field_parse_attempt_index += 1;
                                    break;
                                }
                                3 => if let Ok(field) = field.parse() {
                                    custom_sample_set = Some(field);
                                    field_parse_attempt_index += 1;
                                    break;
                                } else {
                                    return Err(TriggerTypeParseError::UnknownHitSoundType(s.to_string()))
                                }
                                _ => unreachable!("The check for field size is already done so this is impossible to reach")
                            }
                            field_parse_attempt_index += 1;
                        }
                    }

                    Ok(TriggerType::HitSound {
                        sample_set,
                        additions_sample_set,
                        addition,
                        custom_sample_set,
                    })
                }
            },
            None => Err(TriggerTypeParseError::UnknownTriggerType(s.to_string())),
        }
    }
}

impl Display for TriggerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let trigger_type = match self {
            TriggerType::HitSound {
                sample_set,
                additions_sample_set,
                addition,
                custom_sample_set,
            } => format!(
                "HitSound{}{}{}{}",
                sample_set.map_or(String::new(), |s| s.to_string()),
                additions_sample_set.map_or(String::new(), |s| s.to_string()),
                addition.map_or(String::new(), |s| s.to_string()),
                custom_sample_set.map_or(String::new(), |s| s.to_string())
            ),
            TriggerType::Passing => "HitSoundPassing".to_string(),
            TriggerType::Failing => "HitSoundFailing".to_string(),
        };

        write!(f, "{trigger_type}")
    }
}

#[derive(Debug, Error)]
pub enum TriggerTypeParseError {
    #[error("There are too many `HitSound` fields: {0}")]
    TooManyHitSoundFields(usize),
    #[error("There was a problem parsing a field")]
    FieldParseError {
        #[from]
        source: ParseIntError,
    },
    #[error("Unknown trigger type {0}")]
    UnknownTriggerType(String),
    #[error("Unknown `HitSound` type {0}")]
    UnknownHitSoundType(String),
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, EnumString, Display)]
pub enum SampleSet {
    All,
    Normal,
    Soft,
    Drum,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, EnumString, Display)]
pub enum Addition {
    Whistle,
    Finish,
    Clap,
}

#[derive(Debug, Error)]
pub enum CommandParseError {
    #[error("The field {0} is missing from the [`Command`]")]
    MissingField(&'static str),
    #[error("The event type {0} is unknown")]
    UnknownEvent(String),
    #[error("Attempted to parse {value} from a `str` as another type")]
    FieldParseError {
        #[source]
        source: Box<dyn Error>,
        value: String,
    },
    #[error("Invalid easing, {0}")]
    InvalidEasing(usize),
}

// TODO does this have integer form?
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, FromRepr)]
pub enum Easing {
    Linear,
    EasingOut,
    EasingIn,
    QuadIn,
    QuadOut,
    QuadInOut,
    CubicIn,
    CubicOut,
    CubicInOut,
    QuartIn,
    QuartOut,
    QuartInOut,
    QuintIn,
    QuintOut,
    QuintInOut,
    SineIn,
    SineOut,
    SineInOut,
    ExpoIn,
    ExpoOut,
    ExpoInOut,
    CircIn,
    CircOut,
    CircInOut,
    ElasticIn,
    ElasticOut,
    ElasticHalfOut,
    ElasticQuarterOut,
    ElasticInOut,
    BackIn,
    BackOut,
    BackInOut,
    BounceIn,
    BounceOut,
    BounceInOut,
}

#[derive(Debug, Error)]
pub enum EasingParseError {
    #[error(transparent)]
    ValueParseError(#[from] ParseIntError),
    #[error("Unknown easing type {0}")]
    UnknownEasingType(usize),
}
