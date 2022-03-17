use std::{
    error::Error,
    fmt::Display,
    num::ParseIntError,
    path::{Path, PathBuf},
    str::{FromStr, Split},
};

use rust_decimal::Decimal;
use thiserror::Error;

use crate::osu_file::{Integer, Position};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Object {
    pub layer: Layer,
    pub origin: Origin,
    pub position: Position,
    pub object_type: ObjectType,
}

impl Object {
    pub fn push_cmd(&mut self, _cmd: Command) {
        todo!()
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
                })
            }
            _ => return Err(ObjectParseError::UnknownObjectType(object_type.to_string())),
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
    frame_count: u32,
    frame_delay: u32,
    loop_type: LoopType,
    filepath: PathBuf,
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

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Layer {
    Background,
    Fail,
    Pass,
    Foreground,
}

impl TryFrom<Integer> for Layer {
    type Error = LayerParseError;

    fn try_from(value: Integer) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Layer::Background),
            1 => Ok(Layer::Fail),
            2 => Ok(Layer::Pass),
            3 => Ok(Layer::Foreground),
            _ => Err(LayerParseError::UnknownLayerType(value)),
        }
    }
}

impl From<Layer> for Integer {
    fn from(layer: Layer) -> Self {
        match layer {
            Layer::Background => 0,
            Layer::Fail => 1,
            Layer::Pass => 2,
            Layer::Foreground => 3,
        }
    }
}

impl Display for Layer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Integer::from(*self))
    }
}

impl FromStr for Layer {
    type Err = LayerParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Background" => Ok(Layer::Background),
            "Fail" => Ok(Layer::Fail),
            "Pass" => Ok(Layer::Pass),
            "Foreground" => Ok(Layer::Foreground),
            _ => {
                // TODO is this even the case? find out
                // attempt integer parse

                let s: Integer = s.parse().map_err(|err| LayerParseError::ValueParseError {
                    source: err,
                    value: s.to_string(),
                })?;

                Layer::try_from(s)
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum LayerParseError {
    #[error("Unknown layer type {0}")]
    UnknownLayerType(Integer),
    #[error("There was a problem converting a `str` into an `Integer`")]
    ValueParseError {
        #[source]
        source: ParseIntError,
        value: String,
    },
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
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

impl TryFrom<Integer> for Origin {
    type Error = OriginParseError;

    fn try_from(value: Integer) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Origin::TopLeft),
            1 => Ok(Origin::Centre),
            2 => Ok(Origin::CentreLeft),
            3 => Ok(Origin::TopRight),
            4 => Ok(Origin::BottomCentre),
            5 => Ok(Origin::TopCentre),
            6 => Ok(Origin::Custom),
            7 => Ok(Origin::CentreRight),
            8 => Ok(Origin::BottomLeft),
            9 => Ok(Origin::BottomRight),
            _ => Err(OriginParseError::UnknownOriginType(value)),
        }
    }
}

impl From<Origin> for Integer {
    fn from(origin: Origin) -> Self {
        match origin {
            Origin::TopLeft => 0,
            Origin::Centre => 1,
            Origin::CentreLeft => 2,
            Origin::TopRight => 3,
            Origin::BottomCentre => 4,
            Origin::TopCentre => 5,
            Origin::Custom => 6,
            Origin::CentreRight => 7,
            Origin::BottomLeft => 8,
            Origin::BottomRight => 9,
        }
    }
}

impl Display for Origin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Integer::from(*self))
    }
}

impl FromStr for Origin {
    type Err = OriginParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TopLeft" => Ok(Origin::TopLeft),
            "Centre" => Ok(Origin::Centre),
            "CentreLeft" => Ok(Origin::CentreLeft),
            "TopRight" => Ok(Origin::TopRight),
            "BottomCentre" => Ok(Origin::BottomCentre),
            "TopCentre" => Ok(Origin::TopCentre),
            "Custom" => Ok(Origin::Custom),
            "CentreRight" => Ok(Origin::CentreRight),
            "BottomLeft" => Ok(Origin::BottomLeft),
            "BottomRight" => Ok(Origin::BottomRight),
            _ => {
                // TODO is this ever used?
                // attempt int parse
                let s: Integer = s.parse().map_err(|err| OriginParseError::ValueParseError {
                    source: err,
                    value: s.to_string(),
                })?;

                Origin::try_from(s)
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum OriginParseError {
    #[error("Unknown origin type {0}")]
    UnknownOriginType(Integer),
    #[error("Failed to parse the `str` {value} into an `Integer`")]
    ValueParseError {
        #[source]
        source: ParseIntError,
        value: String,
    },
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum LoopType {
    LoopForever,
    LoopOnce,
}

impl Default for LoopType {
    fn default() -> Self {
        Self::LoopForever
    }
}

impl Display for LoopType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let loop_type = match self {
            LoopType::LoopForever => "LoopForever",
            LoopType::LoopOnce => "LoopOnce",
        };

        write!(f, "{loop_type}")
    }
}

impl FromStr for LoopType {
    type Err = UnknownLoopType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LoopForever" => Ok(LoopType::LoopForever),
            "LoopOnce" => Ok(LoopType::LoopOnce),
            _ => Err(UnknownLoopType(s.to_string())),
        }
    }
}

#[derive(Debug, Error)]
#[error("Unknown loop type {0}")]
pub struct UnknownLoopType(String);

pub enum Command {
    Fade {
        easing: Easing,
        start_time: Integer,
        end_time: Integer,
        start_opacity: Decimal,
        end_opacity: Decimal,
    },
    Move {
        easing: Easing,
        start_time: Integer,
        end_time: Integer,
        start_x: Decimal,
        start_y: Decimal,
        end_x: Decimal,
        end_y: Decimal,
    },
    MoveX,
    MoveY,
    Scale,
    VectorScale,
    Rotate,
    Colour,
    Parameter,
    Loop,
    Trigger,
}

impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(',');

        let event = s
            .next()
            .ok_or(CommandParseError::MissingField("event"))?
            .trim();

        let easing_parse = |s: &mut Split<char>| {
            let s = s.next().ok_or(CommandParseError::MissingField("easing"))?;
            s.parse().map_err(|err| CommandParseError::FieldParseError {
                source: Box::new(err),
                value: s.to_string(),
            })
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
        let end_time_parse = |s: &mut Split<char>| {
            let s = s
                .next()
                .ok_or(CommandParseError::MissingField("end_time"))?;
            s.parse().map_err(|err| CommandParseError::FieldParseError {
                source: Box::new(err),
                value: s.to_string(),
            })
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

        match event {
            "F" => Ok(Command::Fade {
                easing: easing_parse(&mut s)?,
                start_time: start_time_parse(&mut s)?,
                end_time: end_time_parse(&mut s)?,
                start_opacity: decimal_parse(&mut s, "start_opacity")?,
                end_opacity: decimal_parse(&mut s, "end_opacity")?,
            }),
            "M" => Ok(Command::Move {
                easing: easing_parse(&mut s)?,
                start_time: start_time_parse(&mut s)?,
                end_time: end_time_parse(&mut s)?,
                start_x: decimal_parse(&mut s, "start_x")?,
                start_y: decimal_parse(&mut s, "start_y")?,
                end_x: decimal_parse(&mut s, "end_x")?,
                end_y: decimal_parse(&mut s, "end_y")?,
            }),
            _ => Err(CommandParseError::UnknownEvent(event.to_string())),
        }
    }
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
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Easing {}

impl FromStr for Easing {
    type Err = EasingParseError;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Error)]
#[error("")]
// TODO
pub struct EasingParseError;
