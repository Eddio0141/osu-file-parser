use std::fmt::Display;
use std::str::FromStr;

use crate::osu_file::Integer;
use nom::error::VerboseErrorKind;
use nom::Finish;
use rust_decimal::Decimal;
use strum_macros::Display;

use super::error::*;
use super::parser::command;
use super::types::*;

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

impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // usage of parser and error handling here is the most clean way I can do this probably
        // TODO, if its a parse error, parse the str once more to get the error type
        match command(s).finish() {
            Ok(ok) => Ok(ok.1),
            Err(err) => {
                let mut input = None;
                let mut nom_err = None;
                let mut context = None;

                for err in &err.errors {
                    input = Some(err.0);

                    match err.1 {
                        VerboseErrorKind::Context(c) => context = Some(c),
                        VerboseErrorKind::Nom(nom) => nom_err = Some(nom),
                        VerboseErrorKind::Char(_) => (),
                    }
                }

                let input = input.unwrap();

                let err = match context {
                    Some(context) => match context {
                        "missing_start_time" => CommandParseError::MissingField(Field::StartTime),
                        "start_time"
                        | "loop_count"
                        | "group_number"
                        | "invalid_red_field"
                        | "invalid_blue_field"
                        | "invalid_green_field"
                        | "invalid_continuing_red_field"
                        | "invalid_continuing_green_field"
                        | "invalid_continuing_blue_field" => {
                            CommandParseError::ParseIntError(input.to_string())
                        }
                        "missing_loop_count" => CommandParseError::MissingField(Field::LoopCount),
                        "missing_trigger_type" => {
                            CommandParseError::MissingField(Field::TriggerType)
                        }
                        "trigger_type" => {
                            CommandParseError::TriggerTypeParseError(input.to_string())
                        }
                        "missing_params" => CommandParseError::MissingCommandParams,
                        "missing_blue_field" => CommandParseError::MissingField(Field::Blue),
                        "missing_green_field" => CommandParseError::MissingField(Field::Green),
                        "parameter_type" | "continuing_parameter_types" => {
                            CommandParseError::ParameterTypeParseError(input.to_string())
                        }
                        "first_parameter" | "second_parameter" | "continuing_parameters" => {
                            CommandParseError::DecimalParseError(input.to_string())
                        }
                        "missing_second_parameter" => CommandParseError::MissingSecondParameter,
                        "eof" => CommandParseError::InvalidFieldEnding(input.to_string()),
                        _ => unimplemented!("unimplemented context {context}"),
                    },
                    None => unimplemented!("no context found for command parser"),
                };

                Err(err)
            }
        }
    }
}

#[derive(Display, Debug)]
pub enum Field {
    Easing,
    StartTime,
    EndTime,
    LoopCount,
    TriggerType,
    Green,
    Blue,
}
