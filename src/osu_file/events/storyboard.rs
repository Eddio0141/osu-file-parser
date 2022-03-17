use std::{path::PathBuf, str::{FromStr, Split}};

use rust_decimal::Decimal;
use thiserror::Error;

use crate::osu_file::{Position, Integer};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Object {
    pub layer: Layer,
    pub origin: Origin,
    pub position: Position,
    pub object_type: ObjectType,
}

impl Object {
    pub fn push_cmd(&mut self, cmd: Command) {}
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
    pub fn new(frame_count: u32, frame_delay: u32, loop_type: LoopType, filepath: PathBuf) -> Self {
        Self {
            frame_count,
            frame_delay,
            loop_type,
            filepath,
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
    filepath: PathBuf,
}

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

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct LoopType;

pub enum Command {
    Fade {
        easing: Easing,
        start_time: Integer,
        end_time: Integer,
        start_opacity: Decimal,
        end_opacity: Decimal,
    },
    Move,
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

        let event = s.next().ok_or(CommandParseError::MissingField("event"))?.trim();

        let easing_parse = |s: &mut Split<char>| s.next().ok_or(CommandParseError::MissingField("easing"))?;
        let start_time_parse = |s: &mut Split<char>| s.next().ok_or(CommandParseError::MissingField("start_time"));
        let end_time_parse = |s: &mut Split<char>| s.next().ok_or(CommandParseError::MissingField("end_time"));

        let decimal_parse = |s: &mut Split<char>, field_name| s.next().ok_or(CommandParseError::MissingField(field_name));

        match event {
            "F" => {
                Ok(Command::Fade { easing: easing_parse(&mut s)?, start_time: start_time_parse(&mut s)?,
                end_time: end_time_parse(&mut s), start_opacity: decimal_parse(&mut s, "start_opacity")?, end_opacity: decimal_parse(&mut s, "end_opacity") })}
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
}

pub enum Easing {

}
