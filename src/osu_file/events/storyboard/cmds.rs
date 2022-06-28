use std::fmt::Display;

use super::error::*;
use super::types::*;
use crate::osu_file::Integer;
use crate::osu_file::VersionedFromString;
use crate::parsers::*;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::combinator::*;
use nom::error::context;
use nom::multi::many0;
use nom::sequence::*;
use nom::Parser;
use rust_decimal::Decimal;

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
    pub start: (u8, u8, u8),
    pub continuing: Vec<(u8, Option<u8>, Option<u8>)>,
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

impl VersionedFromString for Command {
    type ParseError = CommandParseError;

    fn from_str(s: &str, _: usize) -> std::result::Result<Option<Self>, Self::ParseError> {
        let indentation = take_while(|c: char| c == ' ' || c == '_');
        let start_time = || {
            preceded(
                context(CommandParseError::MissingStartTime.into(), cut(comma())),
                context(
                    CommandParseError::InvalidStartTime.into(),
                    cut(comma_field_type()),
                ),
            )
        };
        let end_time = || {
            preceded(
                context(CommandParseError::MissingEndTime.into(), cut(comma())),
                alt((
                    verify(comma_field(), |s: &str| s.trim().is_empty()).map(|_| None),
                    cut(
                        context(CommandParseError::InvalidEndTime.into(), comma_field_type())
                            .map(Some),
                    ),
                )),
            )
        };
        let easing = || {
            cut(preceded(
                context(CommandParseError::MissingEasing.into(), comma()),
                context(
                    CommandParseError::InvalidEasing.into(),
                    map_opt(comma_field_type(), Easing::from_repr),
                ),
            ))
        };
        let start_time_end_time_easing = || tuple((easing(), start_time(), end_time()));
        let continuing_decimal_two_fields =
            |command_type: &'static str,
             missing_starting_first,
             invalid_start_first,
             missing_starting_second,
             invalid_starting_second,
             invalid_continuing| {
                let continuing = alt((
                    eof.map(|_| None),
                    cut(preceded(comma(), comma_field_type()).map(Some)),
                ));
                let continuing = many0(preceded(comma(), tuple((comma_field_type(), continuing))));

                preceded(
                    tag(command_type),
                    tuple((
                        start_time_end_time_easing(),
                        cut(preceded(
                            context(missing_starting_first, comma()),
                            context(invalid_start_first, comma_field_type()),
                        )),
                        cut(preceded(
                            context(missing_starting_second, comma()),
                            context(invalid_starting_second, comma_field_type()),
                        )),
                        terminated(continuing, context(invalid_continuing, cut(eof))),
                    )),
                )
            };
        let continuing_decimal_fields =
            |command_type: &'static str, missing_start, invalid_start, invalid_continuing| {
                let continuing = many0(preceded(comma(), comma_field_type()));

                preceded(
                    tag(command_type),
                    tuple((
                        start_time_end_time_easing(),
                        cut(preceded(
                            context(missing_start, comma()),
                            context(invalid_start, comma_field_type()),
                        )),
                        terminated(continuing, context(invalid_continuing, cut(eof))),
                    )),
                )
            };

        let loop_ = preceded(
            tag("L"),
            cut(tuple((
                start_time(),
                preceded(
                    context(CommandParseError::MissingLoopCount.into(), comma()),
                    context(
                        CommandParseError::InvalidLoopCount.into(),
                        map_res(rest, |s: &str| s.parse()),
                    ),
                ),
            ))),
        )
        .map(|(start_time, loop_count)| Command {
            start_time,
            properties: CommandProperties::Loop {
                loop_count,
                commands: Vec::new(),
            },
        });
        let trigger = {
            let trigger_nothing = alt((
                eof.map(|_| (None, None)),
                verify(rest, |s: &str| s == ",").map(|_| (None, None)),
            ));
            let trigger_group_number = preceded(
                tuple((comma(), comma())),
                context(
                    CommandParseError::InvalidGroupNumber.into(),
                    cut(consume_rest_type()),
                ),
            )
            .map(|group_number| (None, Some(group_number)));
            let trigger_end_time = preceded(
                comma(),
                context(
                    CommandParseError::InvalidEndTime.into(),
                    cut(consume_rest_type()),
                ),
            )
            .map(|end_time| (Some(end_time), None));
            let trigger_everything = tuple((
                preceded(comma(), comma_field_type()),
                preceded(
                    comma(),
                    context(
                        CommandParseError::InvalidGroupNumber.into(),
                        cut(consume_rest_type()),
                    ),
                ),
            ))
            .map(|(end_time, group_number)| (Some(end_time), Some(group_number)));

            preceded(
                tuple((
                    tag("T"),
                    context(CommandParseError::MissingTriggerType.into(), cut(comma())),
                )),
                cut(tuple((
                    context(
                        CommandParseError::InvalidTriggerType.into(),
                        comma_field_type(),
                    ),
                    start_time(),
                    // there are 4 possibilities:
                    alt((
                        // has everything
                        trigger_everything,
                        // has group number
                        trigger_group_number,
                        // nothing
                        trigger_nothing,
                        // has end time
                        trigger_end_time,
                    )),
                ))),
            )
            .map(
                |(trigger_type, start_time, (end_time, group_number))| Command {
                    start_time,
                    properties: CommandProperties::Trigger {
                        trigger_type,
                        end_time,
                        group_number,
                        commands: Vec::new(),
                    },
                },
            )
        };
        let colour = {
            let continuing_colour = || {
                alt((
                    eof.map(|_| None),
                    preceded(comma(), comma_field_type()).map(Some),
                ))
            };
            let continuing_colours = many0(preceded(
                comma(),
                tuple((comma_field_type(), continuing_colour(), continuing_colour())),
            ));

            preceded(
                tag("C"),
                tuple((
                    start_time_end_time_easing(),
                    cut(preceded(
                        context(CommandParseError::MissingRed.into(), comma()),
                        context(CommandParseError::InvalidRed.into(), comma_field_type()),
                    )),
                    cut(preceded(
                        context(CommandParseError::MissingGreen.into(), comma()),
                        context(CommandParseError::InvalidGreen.into(), comma_field_type()),
                    )),
                    cut(preceded(
                        context(CommandParseError::MissingBlue.into(), comma()),
                        context(CommandParseError::InvalidBlue.into(), comma_field_type()),
                    )),
                    terminated(
                        continuing_colours,
                        context(CommandParseError::InvalidContinuingColours.into(), cut(eof)),
                    ),
                )),
            )
            .map(
                |((easing, start_time, end_time), start_r, start_g, start_b, continuing)| Command {
                    start_time,
                    properties: CommandProperties::Colour {
                        easing,
                        end_time,
                        colours: Colours {
                            start: (start_r, start_g, start_b),
                            continuing,
                        },
                    },
                },
            )
        };
        let parameter = {
            let continuing_parameters = many0(preceded(comma(), comma_field_type()));

            preceded(
                tag("P"),
                tuple((
                    start_time_end_time_easing(),
                    cut(preceded(
                        context(CommandParseError::MissingParameterType.into(), comma()),
                        context(
                            CommandParseError::InvalidParameterType.into(),
                            comma_field_type(),
                        ),
                    )),
                    terminated(
                        continuing_parameters,
                        context(
                            CommandParseError::InvalidContinuingParameters.into(),
                            cut(eof),
                        ),
                    ),
                )),
            )
            .map(
                |((easing, start_time, end_time), parameter, continuing_parameters)| Command {
                    start_time,
                    properties: CommandProperties::Parameter {
                        easing,
                        end_time,
                        parameter,
                        continuing_parameters,
                    },
                },
            )
        };
        let move_ = continuing_decimal_two_fields(
            "M",
            CommandParseError::MissingMoveX.into(),
            CommandParseError::InvalidMoveX.into(),
            CommandParseError::MissingMoveY.into(),
            CommandParseError::InvalidMoveY.into(),
            CommandParseError::InvalidContinuingMove.into(),
        )
        .map(
            |((easing, start_time, end_time), start_x, start_y, continuing)| Command {
                start_time,
                properties: CommandProperties::Move {
                    easing,
                    end_time,
                    positions_xy: ContinuingFields {
                        start: (start_x, start_y),
                        continuing,
                    },
                },
            },
        );
        let vector_scale = continuing_decimal_two_fields(
            "V",
            CommandParseError::MissingScaleX.into(),
            CommandParseError::InvalidScaleX.into(),
            CommandParseError::MissingScaleY.into(),
            CommandParseError::InvalidScaleY.into(),
            CommandParseError::InvalidContinuingScales.into(),
        )
        .map(
            |((easing, start_time, end_time), start_x, start_y, continuing)| Command {
                start_time,
                properties: CommandProperties::VectorScale {
                    easing,
                    end_time,
                    scales_xy: ContinuingFields {
                        start: (start_x, start_y),
                        continuing,
                    },
                },
            },
        );
        let fade = continuing_decimal_fields(
            "F",
            CommandParseError::MissingStartOpacity.into(),
            CommandParseError::InvalidStartOpacity.into(),
            CommandParseError::InvalidContinuingOpacities.into(),
        )
        .map(
            |((easing, start_time, end_time), start_opacity, continuing_opacities)| Command {
                start_time,
                properties: CommandProperties::Fade {
                    easing,
                    end_time,
                    start_opacity,
                    continuing_opacities,
                },
            },
        );
        let move_x = continuing_decimal_fields(
            "MX",
            CommandParseError::MissingMoveX.into(),
            CommandParseError::InvalidMoveX.into(),
            CommandParseError::InvalidContinuingMove.into(),
        )
        .map(
            |((easing, start_time, end_time), start_x, continuing_x)| Command {
                start_time,
                properties: CommandProperties::MoveX {
                    easing,
                    end_time,
                    start_x,
                    continuing_x,
                },
            },
        );
        let move_y = continuing_decimal_fields(
            "MY",
            CommandParseError::MissingMoveY.into(),
            CommandParseError::InvalidMoveY.into(),
            CommandParseError::InvalidContinuingMove.into(),
        )
        .map(
            |((easing, start_time, end_time), start_y, continuing_y)| Command {
                start_time,
                properties: CommandProperties::MoveY {
                    easing,
                    end_time,
                    start_y,
                    continuing_y,
                },
            },
        );
        let scale = continuing_decimal_fields(
            "S",
            CommandParseError::MissingStartScale.into(),
            CommandParseError::InvalidStartScale.into(),
            CommandParseError::InvalidContinuingScales.into(),
        )
        .map(
            |((easing, start_time, end_time), start_scale, continuing_scales)| Command {
                start_time,
                properties: CommandProperties::Scale {
                    easing,
                    end_time,
                    start_scale,
                    continuing_scales,
                },
            },
        );
        let rotate = continuing_decimal_fields(
            "R",
            CommandParseError::MissingStartRotation.into(),
            CommandParseError::InvalidStartRotation.into(),
            CommandParseError::InvalidContinuingRotation.into(),
        )
        .map(
            |((easing, start_time, end_time), start_rotation, continuing_rotations)| Command {
                start_time,
                properties: CommandProperties::Rotate {
                    easing,
                    end_time,
                    start_rotation,
                    continuing_rotations,
                },
            },
        );

        // we order by the most common to the least common
        let parse = preceded(
            indentation,
            alt((
                fade,
                move_x,
                move_y,
                scale,
                rotate,
                loop_,
                trigger,
                colour,
                parameter,
                move_,
                vector_scale,
                context(CommandParseError::UnknownCommandType.into(), fail),
            )),
        )(s)?;

        Ok(Some(parse.1))
    }
}
