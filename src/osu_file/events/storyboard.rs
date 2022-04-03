use std::{
    error::Error,
    fmt::Display,
    num::ParseIntError,
    path::{Path, PathBuf},
    str::{FromStr, Split},
};

use nom::{
    bytes::complete::take_while,
    character::complete::char,
    combinator::{map_opt, map_res, opt},
    multi::many0,
    sequence::{preceded, tuple},
    Parser,
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

fn continuing_to_string<T>(continuing: &[T]) -> String
where
    T: Display,
{
    if continuing.is_empty() {
        String::new()
    } else {
        format!(
            ",{}",
            continuing
                .iter()
                .map(|field| field.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let end_time_to_string =
            |end_time: &Option<i32>| end_time.map_or("".to_string(), |t| t.to_string());

        let cmd_str = match &self.properties {
            CommandProperties::Fade {
                easing,
                end_time,
                start_opacity,
                continuing_opacities,
            } => format!(
                "F,{},{},{},{start_opacity}{}",
                *easing as usize,
                self.start_time,
                end_time_to_string(end_time),
                continuing_to_string(continuing_opacities),
            ),
            CommandProperties::Move {
                easing,
                end_time,
                positions_xy,
            } => format!(
                "M,{},{},{},{positions_xy}",
                *easing as usize,
                self.start_time,
                end_time_to_string(end_time),
            ),
            CommandProperties::MoveX {
                easing,
                end_time,
                start_x,
                continuing_x,
            } => format!(
                "MX,{},{},{},{start_x}{}",
                *easing as usize,
                self.start_time,
                end_time_to_string(end_time),
                continuing_to_string(continuing_x),
            ),
            CommandProperties::MoveY {
                easing,
                end_time,
                start_y,
                continuing_y,
            } => format!(
                "MY,{},{},{},{start_y}{}",
                *easing as usize,
                self.start_time,
                end_time_to_string(end_time),
                continuing_to_string(continuing_y),
            ),
            CommandProperties::Scale {
                easing,
                end_time,
                start_scale,
                continuing_scales,
            } => format!(
                "S,{},{},{},{start_scale}{}",
                *easing as usize,
                self.start_time,
                end_time_to_string(end_time),
                continuing_to_string(continuing_scales),
            ),
            CommandProperties::VectorScale {
                easing,
                end_time,
                scales_xy,
            } => format!(
                "V,{},{},{},{}",
                *easing as usize,
                self.start_time,
                end_time_to_string(end_time),
                scales_xy,
            ),
            CommandProperties::Rotate {
                easing,
                end_time,
                start_rotation,
                continuing_rotations,
            } => format!(
                "R,{},{},{},{start_rotation}{}",
                *easing as usize,
                self.start_time,
                end_time_to_string(end_time),
                continuing_to_string(continuing_rotations),
            ),
            CommandProperties::Colour {
                easing,
                end_time,
                colours,
            } => format!(
                "C,{},{},{},{}",
                *easing as usize,
                self.start_time,
                end_time_to_string(end_time),
                colours,
            ),
            CommandProperties::Parameter {
                easing,
                end_time,
                parameter,
                continuing_parameters,
            } => format!(
                "P,{},{},{},{}{}",
                *easing as usize,
                self.start_time,
                end_time_to_string(end_time),
                parameter,
                continuing_to_string(continuing_parameters),
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
                "T,{trigger_type},{},{}{}",
                self.start_time,
                end_time_to_string(end_time),
                group_number.map_or(String::new(), |group_number| format!(",{group_number}")),
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
        start_opacity: Decimal,
        continuing_opacities: Vec<Decimal>,
    },
    Move {
        easing: Easing,
        end_time: Option<Integer>,
        positions_xy: ContinuingFields<Decimal>,
    },
    MoveX {
        easing: Easing,
        end_time: Option<Integer>,
        start_x: Decimal,
        continuing_x: Vec<Decimal>,
    },
    MoveY {
        easing: Easing,
        end_time: Option<Integer>,
        start_y: Decimal,
        continuing_y: Vec<Decimal>,
    },
    Scale {
        easing: Easing,
        end_time: Option<Integer>,
        start_scale: Decimal,
        continuing_scales: Vec<Decimal>,
    },
    VectorScale {
        easing: Easing,
        end_time: Option<Integer>,
        scales_xy: ContinuingFields<Decimal>,
    },
    Rotate {
        easing: Easing,
        end_time: Option<Integer>,
        start_rotation: Decimal,
        continuing_rotations: Vec<Decimal>,
    },
    Colour {
        easing: Easing,
        end_time: Option<Integer>,
        colours: Colours,
    },
    Parameter {
        easing: Easing,
        end_time: Option<Integer>,
        parameter: Parameter,
        continuing_parameters: Vec<Parameter>,
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

#[derive(Clone, Debug, Hash, PartialEq, Eq, Default)]
pub struct ContinuingFields<T> {
    start: (T, T),
    continuing: Vec<(T, Option<T>)>,
}

impl<T> ContinuingFields<T> {
    pub fn new(
        start: (T, T),
        continuing: Vec<(T, Option<T>)>,
    ) -> Result<Self, InvalidSecondFieldOption> {
        // error if continuing 2nd field is None without it being at the end of the list
        if continuing
            .iter()
            .enumerate()
            .any(|(i, (_, field_2))| field_2.is_none() && i != continuing.len() - 1)
        {
            Err(InvalidSecondFieldOption)
        } else {
            Ok(Self { start, continuing })
        }
    }

    pub fn start_values(&self) -> &(T, T) {
        &self.start
    }

    pub fn start_values_mut(&mut self) -> &mut (T, T) {
        &mut self.start
    }

    pub fn continuing_fields(&self) -> &[(T, Option<T>)] {
        &self.continuing
    }

    pub fn push_continuing_fields(&mut self, continuing_fields: (T, Option<T>))
    where
        T: std::marker::Copy,
    {
        // if the last continuing field 1 is None, its the equalivant of having the previous index's positition 1 (or the start 1 if no elements)
        if let Some(last_continuing) = self.continuing.last() {
            if last_continuing.1.is_none() {
                // find last 1 field
                let last_field = if let Some(last_continuing_with_1) =
                    self.continuing.get(self.continuing.len() - 2)
                {
                    last_continuing_with_1.1.as_ref()
                } else {
                    // backup field 1 is the starting one
                    Some(&self.start.1)
                };

                self.continuing.last_mut().unwrap().1 = last_field.copied();
            }
        }
        self.continuing.push(continuing_fields);
    }

    pub fn set_continuing_fields(
        &mut self,
        index: usize,
        fields: (T, Option<T>),
    ) -> Result<(), ContinuingSetError> {
        // if index isn't the last index, field 1 being none will return an error
        if index != self.continuing.len() - 1 && fields.1.is_none() {
            Err(ContinuingSetError::InvalidSecondFieldOption)
        } else {
            match self.continuing.get_mut(index) {
                Some(continuing_position_xy) => {
                    *continuing_position_xy = fields;
                    Ok(())
                }
                None => Err(ContinuingSetError::IndexOutOfBounds),
            }
        }
    }
}

impl<T> Display for ContinuingFields<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut builder = vec![self.start.0.to_string(), self.start.1.to_string()];

        for fields in &self.continuing {
            builder.push(fields.0.to_string());
            if let Some(field) = &fields.1 {
                builder.push(field.to_string());
            }
        }

        write!(f, "{}", builder.join(","))
    }
}

#[derive(Debug, Error)]
pub enum ContinuingSetError {
    #[error("continuing fields index out of bounds")]
    IndexOutOfBounds,
    #[error(
        "continuing fields 2nd field is none without it being the last item in the continuing fields")]
    InvalidSecondFieldOption,
}

#[derive(Debug, Error)]
#[error(
    "continuing fields 2nd field is none without it being the last item in the continuing fields"
)]
pub struct InvalidSecondFieldOption;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Colours {
    start: (u8, u8, u8),
    continuing: Vec<(u8, Option<u8>, Option<u8>)>,
}

impl Colours {
    pub fn new(
        start: (u8, u8, u8),
        continuing: Vec<(u8, Option<u8>, Option<u8>)>,
    ) -> Result<Self, InvalidColourFieldOption> {
        for (i, (_, green, blue)) in continuing.iter().enumerate() {
            if i != continuing.len() - 1 {
                if green.is_none() {
                    return Err(InvalidColourFieldOption::Green);
                } else if blue.is_none() {
                    return Err(InvalidColourFieldOption::Blue);
                }
            }
            if green.is_none() && blue.is_some() {
                return Err(InvalidColourFieldOption::Blue);
            }
        }
        Ok(Self { start, continuing })
    }

    pub fn start_rgb(&self) -> &(u8, u8, u8) {
        &self.start
    }

    pub fn start_rgb_mut(&mut self) -> &mut (u8, u8, u8) {
        &mut self.start
    }

    pub fn continuing_fields(&self) -> &[(u8, Option<u8>, Option<u8>)] {
        &self.continuing
    }

    pub fn push_continuing_rgbs(&mut self, continuing_fields: (u8, Option<u8>, Option<u8>)) {
        if let Some(last_continuing) = self.continuing.last() {
            if last_continuing.1.is_none() {
                // find last field
                let last_field = if let Some(last_continuing_with_1) =
                    self.continuing.get(self.continuing.len() - 2)
                {
                    last_continuing_with_1.1.as_ref()
                } else {
                    // backup field 1 is the starting one
                    Some(&self.start.1)
                };

                self.continuing.last_mut().unwrap().1 = last_field.copied();
            } else if last_continuing.2.is_none() {
                // find last field
                let last_field = if let Some(last_continuing_with_2) =
                    self.continuing.get(self.continuing.len() - 2)
                {
                    last_continuing_with_2.2.as_ref()
                } else {
                    // backup field 2 is the starting one
                    Some(&self.start.2)
                };

                self.continuing.last_mut().unwrap().2 = last_field.copied();
            }
        }
        self.continuing.push(continuing_fields);
    }

    pub fn set_continuing_rgbs(
        &mut self,
        index: usize,
        fields: (u8, Option<u8>, Option<u8>),
    ) -> Result<(), ContinuingRGBSetError> {
        // if index isn't the last index, b or g being none will return an error
        let index_is_last = index == self.continuing.len() - 1;

        if !index_is_last && fields.1.is_none() {
            Err(ContinuingRGBSetError::InvalidFieldOption(
                InvalidColourFieldOption::Green,
            ))
        } else if (!index_is_last && fields.2.is_none())
            || (fields.1.is_none() && fields.2.is_some())
        {
            Err(ContinuingRGBSetError::InvalidFieldOption(
                InvalidColourFieldOption::Blue,
            ))
        } else {
            match self.continuing.get_mut(index) {
                Some(continuing) => {
                    *continuing = fields;
                    Ok(())
                }
                None => Err(ContinuingRGBSetError::IndexOutOfBounds),
            }
        }
    }
}

impl Display for Colours {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut builder = vec![
            self.start.0.to_string(),
            self.start.1.to_string(),
            self.start.1.to_string(),
        ];

        for fields in &self.continuing {
            builder.push(fields.0.to_string());
            if let Some(field) = &fields.1 {
                builder.push(field.to_string());
            }
            if let Some(field) = &fields.2 {
                builder.push(field.to_string());
            }
        }

        write!(f, "{}", builder.join(","))
    }
}

#[derive(Debug, Error)]
pub enum ContinuingRGBSetError {
    #[error("continuing fields index out of bounds")]
    IndexOutOfBounds,
    #[error(transparent)]
    InvalidFieldOption(#[from] InvalidColourFieldOption),
}

#[derive(Debug, Error)]
pub enum InvalidColourFieldOption {
    #[error("continuing fields green field is none without it being the last item in the continuing fields")]
    Green,
    #[error("continuing fields blue field is none without it being the last item in the continuing fields")]
    Blue,
}

impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let indentation = take_while::<_, _, nom::error::Error<_>>(|c| c == ' ' || c == '_');
        let field = || take_while(|c| c != ',');
        let field_i32 = || map_res(field(), |s: &str| s.parse());
        let comma = || char(',');

        // only parse a single command
        // TODO error checks
        // TODO use alt with all possible stuff
        let (s, (_, command_type, _)) = tuple((indentation, field(), comma()))(s).unwrap();

        // handle generic commands
        match command_type {
            "L" => {
                let (s, (start_time, _, loop_count)) = tuple((
                    field_i32(),
                    comma(),
                    map_res(take_while(|_| true), |s: &str| s.parse()),
                ))(s)
                .unwrap();

                if !s.is_empty() {
                    todo!();
                }

                Ok(Command {
                    start_time,
                    properties: CommandProperties::Loop {
                        loop_count,
                        commands: Vec::new(),
                    },
                })
            }
            "T" => {
                let (s, (trigger_type, _, start_time, _, end_time, group_number)) = tuple((
                    map_res(field(), |s: &str| s.parse()),
                    comma(),
                    field_i32(),
                    comma(),
                    opt(field_i32()),
                    opt(tuple((comma(), field_i32()))).map(|value| value.map(|(_, value)| value)),
                ))(
                    s
                )
                .unwrap();

                if !s.is_empty() {
                    todo!();
                }

                Ok(Command {
                    start_time,
                    properties: CommandProperties::Trigger {
                        trigger_type,
                        end_time,
                        group_number,
                        commands: Vec::new(),
                    },
                })
            }
            _ => {
                let (s, (easing, _, start_time, _, end_time, _)) = tuple((
                    map_opt(field_i32(), |s| Easing::from_repr(s as usize)),
                    comma(),
                    field_i32(),
                    comma(),
                    opt(field_i32()),
                    comma(),
                ))(s)
                .unwrap();

                // divided into more common fields
                // those fields either have 1 property up to 3, which is almost all decimal types, other than the colour fields and the parameter fields
                match command_type {
                    "C" => {
                        // colour
                        let field_u8 = || map_res(field(), |s: &str| s.parse());

                        let continuing_colour = || opt(preceded(comma(), field_u8()));
                        let continuing_colours = many0(preceded(
                            comma(),
                            tuple((field_u8(), continuing_colour(), continuing_colour())),
                        ));

                        let (s, (start_r, _, start_g, _, start_b, continuing_colours)) =
                            tuple((
                                field_u8(),
                                comma(),
                                field_u8(),
                                comma(),
                                field_u8(),
                                continuing_colours,
                            ))(s)
                            .unwrap();

                        if s.is_empty() {
                            Ok(Command {
                                start_time,
                                properties: CommandProperties::Colour {
                                    easing,
                                    end_time,
                                    // requires no error checks, the fields stack on top of each other after the first 3 fields
                                    colours: Colours::new(
                                        (start_r, start_g, start_b),
                                        continuing_colours,
                                    )
                                    .unwrap(),
                                },
                            })
                        } else {
                            Err(CommandParseError::InvalidFieldEnding)
                        }
                    }
                    "P" => {
                        // parameter
                        let parameter = || map_res(field(), |s: &str| s.parse());
                        let continuing_parameters = many0(preceded(comma(), parameter()));

                        let (s, (parameter, continuing_parameters)) =
                            tuple((parameter(), continuing_parameters))(s).unwrap();

                        if s.is_empty() {
                            Ok(Command {
                                start_time,
                                properties: CommandProperties::Parameter {
                                    easing,
                                    end_time,
                                    parameter,
                                    continuing_parameters,
                                },
                            })
                        } else {
                            Err(CommandParseError::InvalidFieldEnding)
                        }
                    }
                    _ => {
                        let decimal = || map_res(field(), |s: &str| s.parse());

                        // divided into types with 1 continuous field and 2 fields thats continuous
                        match command_type {
                            "M" | "V" => {
                                let continuing = || opt(preceded(comma(), decimal()));
                                let continuing_fields =
                                    many0(preceded(comma(), tuple((decimal(), continuing()))));

                                let (s, (start_1, _, start_2, continuing)) =
                                    tuple((decimal(), comma(), decimal(), continuing_fields))(s)
                                        .unwrap();

                                let continuing_fields =
                                    ContinuingFields::new((start_1, start_2), continuing).unwrap();

                                if s.is_empty() {
                                    match command_type {
                                        "M" => Ok(Command {
                                            start_time,
                                            properties: CommandProperties::Move {
                                                easing,
                                                end_time,
                                                positions_xy: continuing_fields,
                                            },
                                        }),
                                        "V" => Ok(Command {
                                            start_time,
                                            properties: CommandProperties::VectorScale {
                                                easing,
                                                end_time,
                                                scales_xy: continuing_fields,
                                            },
                                        }),
                                        _ => unreachable!(),
                                    }
                                } else {
                                    Err(CommandParseError::InvalidFieldEnding)
                                }
                            }
                            // this is where the unreachable event type gets handled too
                            _ => {
                                let continuing = many0(preceded(comma(), decimal()));

                                let (s, (start, continuing)) =
                                    tuple((decimal(), continuing))(s).unwrap();

                                if s.is_empty() {
                                    match command_type {
                                        "F" => Ok(Command {
                                            start_time,
                                            properties: CommandProperties::Fade {
                                                easing,
                                                end_time,
                                                start_opacity: start,
                                                continuing_opacities: continuing,
                                            },
                                        }),
                                        "MX" => Ok(Command {
                                            start_time,
                                            properties: CommandProperties::MoveX {
                                                easing,
                                                end_time,
                                                start_x: start,
                                                continuing_x: continuing,
                                            },
                                        }),
                                        "MY" => Ok(Command {
                                            start_time,
                                            properties: CommandProperties::MoveY {
                                                easing,
                                                end_time,
                                                start_y: start,
                                                continuing_y: continuing,
                                            },
                                        }),
                                        "S" => Ok(Command {
                                            start_time,
                                            properties: CommandProperties::Scale {
                                                easing,
                                                end_time,
                                                start_scale: start,
                                                continuing_scales: continuing,
                                            },
                                        }),
                                        "R" => Ok(Command {
                                            start_time,
                                            properties: CommandProperties::Rotate {
                                                easing,
                                                end_time,
                                                start_rotation: start,
                                                continuing_rotations: continuing,
                                            },
                                        }),
                                        _ => Err(CommandParseError::UnknownEvent(
                                            command_type.to_string(),
                                        )),
                                    }
                                } else {
                                    Err(CommandParseError::InvalidFieldEnding)
                                }
                            }
                        }
                    }
                }
            }
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
// TODO check field types to be something that makes sense
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
    #[error("Invalid field ending formatting")]
    InvalidFieldEnding,
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
