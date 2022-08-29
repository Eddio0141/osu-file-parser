use std::path::{Path, PathBuf};

use either::Either;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{cut, fail};
use nom::error::context;
use nom::sequence::{preceded, tuple};
use nom::Parser;

use crate::events::EventWithCommands;
use crate::osu_file::{
    FilePath, Position, Version, VersionedDefault, VersionedFromStr, VersionedToString,
};
use crate::parsers::{
    comma, comma_field, comma_field_type, comma_field_versioned_type, consume_rest_versioned_type,
    nothing,
};
use crate::{Integer, VersionedFrom, VersionedTryFrom};

use super::cmds::*;
use super::error::*;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[non_exhaustive]
pub enum Layer {
    Background,
    Fail,
    Pass,
    Foreground,
    Overlay,
}

impl VersionedFromStr for Layer {
    type Err = ParseLayerError;

    fn from_str(s: &str, _: Version) -> std::result::Result<Option<Self>, Self::Err> {
        match s {
            "Background" => Ok(Some(Layer::Background)),
            "Fail" => Ok(Some(Layer::Fail)),
            "Pass" => Ok(Some(Layer::Pass)),
            "Foreground" => Ok(Some(Layer::Foreground)),
            "Overlay" => Ok(Some(Layer::Overlay)),
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
            Layer::Overlay => "Overlay",
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
        self.to_string_variables(version, &[])
    }
}

impl EventWithCommands for Object {
    fn commands(&self) -> &[Command] {
        &self.commands
    }

    fn commands_mut(&mut self) -> &mut Vec<Command> {
        &mut self.commands
    }

    fn to_string_cmd(&self, version: Version) -> Option<String> {
        let pos_str = format!("{},{}", self.position.x, self.position.y);

        let obj = match &self.object_type {
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

        Some(obj)
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
    pub frame_delay: rust_decimal::Decimal,
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
pub struct Origin {
    /// Origin type.
    /// - `Left` variant would be the valid enum variants.
    /// - `Right` variant is for other variants that is used but isn't documented.
    pub type_: Either<OriginType, Integer>,
    pub shorthand: bool,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[non_exhaustive]
pub enum OriginType {
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

impl From<OriginType> for Origin {
    fn from(type_: OriginType) -> Self {
        Self {
            type_: Either::Left(type_),
            shorthand: true,
        }
    }
}

impl VersionedToString for Origin {
    fn to_string(&self, version: Version) -> Option<String> {
        let origin = if self.shorthand {
            let origin = match self.type_ {
                Either::Left(type_) => {
                    <Integer as VersionedFrom<OriginType>>::from(type_, version).unwrap()
                }
                Either::Right(type_) => type_,
            };

            origin.to_string()
        } else {
            match self.type_ {
                Either::Left(type_) => match type_ {
                    OriginType::TopLeft => "TopLeft",
                    OriginType::Centre => "Centre",
                    OriginType::CentreLeft => "CentreLeft",
                    OriginType::TopRight => "TopRight",
                    OriginType::BottomCentre => "BottomCentre",
                    OriginType::TopCentre => "TopCentre",
                    OriginType::Custom => "Custom",
                    OriginType::CentreRight => "CentreRight",
                    OriginType::BottomLeft => "BottomLeft",
                    OriginType::BottomRight => "BottomRight",
                }
                .to_string(),
                Either::Right(type_) => type_.to_string(),
            }
        };

        Some(origin)
    }
}

impl VersionedFromStr for Origin {
    type Err = ParseOriginError;

    fn from_str(s: &str, version: Version) -> std::result::Result<Option<Self>, Self::Err> {
        match s {
            "TopLeft" => Ok(Some(Origin {
                shorthand: false,
                type_: Either::Left(OriginType::TopLeft),
            })),
            "Centre" => Ok(Some(Origin {
                shorthand: false,
                type_: Either::Left(OriginType::Centre),
            })),
            "CentreLeft" => Ok(Some(Origin {
                shorthand: false,
                type_: Either::Left(OriginType::CentreLeft),
            })),
            "TopRight" => Ok(Some(Origin {
                shorthand: false,
                type_: Either::Left(OriginType::TopRight),
            })),
            "BottomCentre" => Ok(Some(Origin {
                shorthand: false,
                type_: Either::Left(OriginType::BottomCentre),
            })),
            "TopCentre" => Ok(Some(Origin {
                shorthand: false,
                type_: Either::Left(OriginType::TopCentre),
            })),
            "Custom" => Ok(Some(Origin {
                shorthand: false,
                type_: Either::Left(OriginType::Custom),
            })),
            "CentreRight" => Ok(Some(Origin {
                shorthand: false,
                type_: Either::Left(OriginType::CentreRight),
            })),
            "BottomLeft" => Ok(Some(Origin {
                shorthand: false,
                type_: Either::Left(OriginType::BottomLeft),
            })),
            "BottomRight" => Ok(Some(Origin {
                shorthand: false,
                type_: Either::Left(OriginType::BottomRight),
            })),
            _ => {
                let s = s.parse()?;
                let type_ = match <OriginType as VersionedTryFrom<Integer>>::try_from(s, version) {
                    Ok(type_) => Either::Left(type_.unwrap()),
                    Err(err) => match err {
                        OriginTryFromIntError::UnknownVariant => Either::Right(s),
                    },
                };

                Ok(Some(Origin {
                    type_,
                    shorthand: true,
                }))
            }
        }
    }
}

impl VersionedTryFrom<Integer> for OriginType {
    type Error = OriginTryFromIntError;

    fn try_from(value: Integer, _: Version) -> Result<Option<Self>, Self::Error> {
        match value {
            0 => Ok(Some(OriginType::TopLeft)),
            1 => Ok(Some(OriginType::Centre)),
            2 => Ok(Some(OriginType::CentreLeft)),
            3 => Ok(Some(OriginType::TopRight)),
            4 => Ok(Some(OriginType::BottomCentre)),
            5 => Ok(Some(OriginType::TopCentre)),
            6 => Ok(Some(OriginType::Custom)),
            7 => Ok(Some(OriginType::CentreRight)),
            8 => Ok(Some(OriginType::BottomLeft)),
            9 => Ok(Some(OriginType::BottomRight)),
            _ => Err(OriginTryFromIntError::UnknownVariant),
        }
    }
}

impl VersionedFrom<OriginType> for Integer {
    fn from(value: OriginType, _: Version) -> Option<Self> {
        match value {
            OriginType::TopLeft => Some(0),
            OriginType::Centre => Some(1),
            OriginType::CentreLeft => Some(2),
            OriginType::TopRight => Some(3),
            OriginType::BottomCentre => Some(4),
            OriginType::TopCentre => Some(5),
            OriginType::Custom => Some(6),
            OriginType::CentreRight => Some(7),
            OriginType::BottomLeft => Some(8),
            OriginType::BottomRight => Some(9),
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

impl VersionedDefault for LoopType {
    fn default(_: Version) -> Option<Self> {
        Some(Self::LoopForever)
    }
}
