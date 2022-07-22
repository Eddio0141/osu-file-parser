pub mod error;
pub mod types;

use std::fmt::Display;

use super::error::*;
use super::types::*;
use crate::osb::Variable;
use crate::osu_file::types::Decimal;
use crate::osu_file::{Integer, Version, VersionedFromRepr, VersionedFromStr, VersionedToString};
use crate::parsers::*;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::combinator::*;
use nom::error::context;
use nom::multi::many0;
use nom::sequence::*;
use nom::Parser;

pub use error::*;
pub use types::*;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Command {
    pub start_time: Option<Integer>,
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

fn continuing_versioned_to_string<T>(continuing: &[T], version: Version) -> String
where
    T: VersionedToString,
{
    if continuing.is_empty() {
        String::new()
    } else {
        format!(
            ",{}",
            continuing
                .iter()
                .map(|field| field.to_string(version).unwrap())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

impl VersionedToString for Command {
    fn to_string(&self, version: Version) -> Option<String> {
        self.to_string_variables(version, &[])
    }
}

impl Command {
    pub(crate) fn to_string_variables(
        &self,
        version: Version,
        variables: &[Variable],
    ) -> Option<String> {
        let end_time_to_string =
            |end_time: &Option<i32>| end_time.map_or("".to_string(), |t| t.to_string());
        let variable_replace = |header, cmd: String| {
            let mut cmd = cmd;

            for variable in variables {
                if cmd.contains(&variable.value) {
                    cmd = cmd.replace(&variable.value, &format!("${}", variable.name));
                    break;
                }
            }

            format!("{header},{cmd}")
        };
        let start_time = self.start_time.map_or(String::new(), |t| t.to_string());

        let cmd_str = match &self.properties {
            CommandProperties::Fade {
                easing,
                end_time,
                start_opacity,
                continuing_opacities,
            } => {
                let cmd = format!(
                    "{},{start_time},{},{start_opacity}{}",
                    *easing as usize,
                    end_time_to_string(end_time),
                    continuing_to_string(continuing_opacities),
                );

                variable_replace("F", cmd)
            }
            CommandProperties::Move {
                easing,
                end_time,
                positions_xy,
            } => {
                let cmd = format!(
                    "{},{start_time},{},{positions_xy}",
                    *easing as usize,
                    end_time_to_string(end_time),
                );

                variable_replace("M", cmd)
            }
            CommandProperties::MoveX {
                easing,
                end_time,
                start_x,
                continuing_x,
            } => {
                let cmd = format!(
                    "{},{start_time},{},{start_x}{}",
                    *easing as usize,
                    end_time_to_string(end_time),
                    continuing_to_string(continuing_x),
                );

                variable_replace("MX", cmd)
            }
            CommandProperties::MoveY {
                easing,
                end_time,
                start_y,
                continuing_y,
            } => {
                let cmd = format!(
                    "{},{start_time},{},{start_y}{}",
                    *easing as usize,
                    end_time_to_string(end_time),
                    continuing_to_string(continuing_y),
                );

                variable_replace("MY", cmd)
            }
            CommandProperties::Scale {
                easing,
                end_time,
                start_scale,
                continuing_scales,
            } => {
                let cmd = format!(
                    "{},{start_time},{},{start_scale}{}",
                    *easing as usize,
                    end_time_to_string(end_time),
                    continuing_to_string(continuing_scales),
                );

                variable_replace("S", cmd)
            }
            CommandProperties::VectorScale {
                easing,
                end_time,
                scales_xy,
            } => {
                let cmd = format!(
                    "{},{start_time},{},{}",
                    *easing as usize,
                    end_time_to_string(end_time),
                    scales_xy,
                );

                variable_replace("V", cmd)
            }
            CommandProperties::Rotate {
                easing,
                end_time,
                start_rotation,
                continuing_rotations,
            } => {
                let cmd = format!(
                    "{},{start_time},{},{start_rotation}{}",
                    *easing as usize,
                    end_time_to_string(end_time),
                    continuing_to_string(continuing_rotations),
                );

                variable_replace("R", cmd)
            }
            CommandProperties::Colour {
                easing,
                end_time,
                colours,
            } => {
                let cmd = format!(
                    "{},{start_time},{},{}",
                    *easing as usize,
                    end_time_to_string(end_time),
                    colours.to_string(version).unwrap(),
                );

                variable_replace("C", cmd)
            }
            CommandProperties::Parameter {
                easing,
                end_time,
                parameter,
                continuing_parameters,
            } => {
                let cmd = format!(
                    "{},{start_time},{},{}{}",
                    *easing as usize,
                    end_time_to_string(end_time),
                    parameter.to_string(version).unwrap(),
                    continuing_versioned_to_string(continuing_parameters, version),
                );

                variable_replace("P", cmd)
            }
            CommandProperties::Loop {
                loop_count,
                // ignore commands since its handled separately
                commands: _,
            } => {
                let cmd = format!("{start_time},{loop_count}");

                variable_replace("L", cmd)
            }
            CommandProperties::Trigger {
                trigger_type,
                end_time,
                group_number,
                // ignore commands since its handled separately
                commands: _,
            } => {
                let cmd = format!(
                    "{},{start_time},{}{}",
                    trigger_type.to_string(version).unwrap(),
                    end_time_to_string(end_time),
                    group_number.map_or(String::new(), |group_number| format!(",{group_number}")),
                );

                variable_replace("T", cmd)
            }
        };

        Some(cmd_str)
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
        group_number: Option<Integer>,
        commands: Vec<Command>,
    },
}

impl VersionedFromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str, version: Version) -> std::result::Result<Option<Self>, Self::Err> {
        let indentation = take_while(|c: char| c == ' ' || c == '_');
        let start_time = || {
            preceded(
                context(ParseCommandError::MissingStartTime.into(), comma()),
                context(
                    ParseCommandError::InvalidStartTime.into(),
                    comma_field_type().map(Some),
                ),
            )
        };
        let end_time = || {
            preceded(
                context(ParseCommandError::MissingEndTime.into(), cut(comma())),
                alt((
                    verify(comma_field(), |s: &str| s.trim().is_empty()).map(|_| None),
                    cut(
                        context(ParseCommandError::InvalidEndTime.into(), comma_field_type())
                            .map(Some),
                    ),
                )),
            )
        };
        let easing = || {
            cut(preceded(
                context(ParseCommandError::MissingEasing.into(), comma()),
                context(
                    ParseCommandError::InvalidEasing.into(),
                    map_res(comma_field_type(), |easing| {
                        Easing::from_repr(easing, version).map(|easing| easing.unwrap())
                    }),
                ),
            ))
        };
        // cases for start_time and end_time
        // ...,easing,start_time,end_time,...
        // ...,easing,start_time,,...
        // ...,easing,,,...
        let easing_start_end_time = || {
            tuple((
                easing(),
                alt((
                    tuple((tag(",,"), peek(comma()))).map(|_| (None, None)),
                    tuple((start_time(), end_time())),
                )),
            ))
        };
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
                    cut(tuple((
                        easing_start_end_time(),
                        preceded(
                            context(missing_starting_first, comma()),
                            context(invalid_start_first, comma_field_type()),
                        ),
                        preceded(
                            context(missing_starting_second, comma()),
                            context(invalid_starting_second, comma_field_type()),
                        ),
                        terminated(continuing, context(invalid_continuing, eof)),
                    ))),
                )
            };
        let continuing_decimal_fields =
            |command_type: &'static str, missing_start, invalid_start, invalid_continuing| {
                let continuing = many0(preceded(comma(), comma_field_type()));

                preceded(
                    tag(command_type),
                    cut(tuple((
                        easing_start_end_time(),
                        preceded(
                            context(missing_start, comma()),
                            context(invalid_start, comma_field_type()),
                        ),
                        terminated(continuing, context(invalid_continuing, eof)),
                    ))),
                )
            };

        let loop_ = preceded(
            tag("L"),
            cut(tuple((
                alt((tuple((comma(), peek(comma()))).map(|_| None), start_time())),
                preceded(
                    context(ParseCommandError::MissingLoopCount.into(), comma()),
                    context(
                        ParseCommandError::InvalidLoopCount.into(),
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
                verify(rest, |s: &str| s.trim().is_empty()).map(|_| (None, None)),
                verify(rest, |s: &str| s == ",").map(|_| (None, None)),
            ));
            let trigger_group_number = preceded(
                tuple((comma(), comma())),
                context(
                    ParseCommandError::InvalidGroupNumber.into(),
                    cut(consume_rest_type()),
                ),
            )
            .map(|group_number| (None, Some(group_number)));
            let trigger_end_time = preceded(
                comma(),
                context(
                    ParseCommandError::InvalidEndTime.into(),
                    cut(consume_rest_type()),
                ),
            )
            .map(|end_time| (Some(end_time), None));
            let trigger_everything = tuple((
                preceded(comma(), comma_field_type()),
                preceded(
                    comma(),
                    context(
                        ParseCommandError::InvalidGroupNumber.into(),
                        cut(consume_rest_type()),
                    ),
                ),
            ))
            .map(|(end_time, group_number)| (Some(end_time), Some(group_number)));

            preceded(
                tuple((
                    tag("T"),
                    context(ParseCommandError::MissingTriggerType.into(), cut(comma())),
                )),
                cut(tuple((
                    context(
                        ParseCommandError::InvalidTriggerType.into(),
                        map_res(comma_field(), |s| {
                            TriggerType::from_str(s, version).map(|t| t.unwrap())
                        }),
                    ),
                    alt((tuple((comma(), peek(comma()))).map(|_| None), start_time())),
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
                cut(tuple((
                    easing_start_end_time(),
                    preceded(
                        context(ParseCommandError::MissingRed.into(), comma()),
                        context(ParseCommandError::InvalidRed.into(), comma_field_type()),
                    ),
                    preceded(
                        context(ParseCommandError::MissingGreen.into(), comma()),
                        context(ParseCommandError::InvalidGreen.into(), comma_field_type()),
                    ),
                    preceded(
                        context(ParseCommandError::MissingBlue.into(), comma()),
                        context(ParseCommandError::InvalidBlue.into(), comma_field_type()),
                    ),
                    terminated(
                        continuing_colours,
                        context(ParseCommandError::InvalidContinuingColours.into(), eof),
                    ),
                ))),
            )
            .map(
                |((easing, (start_time, end_time)), start_r, start_g, start_b, continuing)| {
                    Command {
                        start_time,
                        properties: CommandProperties::Colour {
                            easing,
                            end_time,
                            colours: Colours {
                                start: (start_r, start_g, start_b),
                                continuing,
                            },
                        },
                    }
                },
            )
        };
        let parameter = {
            let continuing_parameters =
                many0(preceded(comma(), comma_field_versioned_type(version)));

            preceded(
                tag("P"),
                cut(tuple((
                    easing_start_end_time(),
                    preceded(
                        context(ParseCommandError::MissingParameterType.into(), comma()),
                        context(
                            ParseCommandError::InvalidParameterType.into(),
                            comma_field_versioned_type(version),
                        ),
                    ),
                    terminated(
                        continuing_parameters,
                        context(ParseCommandError::InvalidContinuingParameters.into(), eof),
                    ),
                ))),
            )
            .map(
                |((easing, (start_time, end_time)), parameter, continuing_parameters)| Command {
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
        let move_ = {
            {
                let continuing = alt((
                    eof.map(|_| None),
                    cut(preceded(comma(), comma_field_type()).map(Some)),
                ));
                let continuing = many0(preceded(comma(), tuple((comma_field_type(), continuing))));

                preceded(
                    tuple(((tag("M")), peek(comma()))),
                    cut(tuple((
                        easing_start_end_time(),
                        preceded(
                            context(ParseCommandError::MissingMoveX.into(), comma()),
                            context(ParseCommandError::InvalidMoveX.into(), comma_field_type()),
                        ),
                        preceded(
                            context(ParseCommandError::MissingMoveY.into(), comma()),
                            context(ParseCommandError::InvalidMoveY.into(), comma_field_type()),
                        ),
                        terminated(
                            continuing,
                            context(ParseCommandError::InvalidContinuingMove.into(), eof),
                        ),
                    ))),
                )
            }
        }
        .map(
            |((easing, (start_time, end_time)), start_x, start_y, continuing)| Command {
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
            ParseCommandError::MissingScaleX.into(),
            ParseCommandError::InvalidScaleX.into(),
            ParseCommandError::MissingScaleY.into(),
            ParseCommandError::InvalidScaleY.into(),
            ParseCommandError::InvalidContinuingScales.into(),
        )
        .map(
            |((easing, (start_time, end_time)), start_x, start_y, continuing)| Command {
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
            ParseCommandError::MissingStartOpacity.into(),
            ParseCommandError::InvalidStartOpacity.into(),
            ParseCommandError::InvalidContinuingOpacities.into(),
        )
        .map(
            |((easing, (start_time, end_time)), start_opacity, continuing_opacities)| Command {
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
            ParseCommandError::MissingMoveX.into(),
            ParseCommandError::InvalidMoveX.into(),
            ParseCommandError::InvalidContinuingMove.into(),
        )
        .map(
            |((easing, (start_time, end_time)), start_x, continuing_x)| Command {
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
            ParseCommandError::MissingMoveY.into(),
            ParseCommandError::InvalidMoveY.into(),
            ParseCommandError::InvalidContinuingMove.into(),
        )
        .map(
            |((easing, (start_time, end_time)), start_y, continuing_y)| Command {
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
            ParseCommandError::MissingStartScale.into(),
            ParseCommandError::InvalidStartScale.into(),
            ParseCommandError::InvalidContinuingScales.into(),
        )
        .map(
            |((easing, (start_time, end_time)), start_scale, continuing_scales)| Command {
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
            ParseCommandError::MissingStartRotation.into(),
            ParseCommandError::InvalidStartRotation.into(),
            ParseCommandError::InvalidContinuingRotation.into(),
        )
        .map(
            |((easing, (start_time, end_time)), start_rotation, continuing_rotations)| Command {
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
            // note: if adding new command, make sure to check if the char is conflicting
            // if it is, make sure we peek for a comma after the tag check
            alt((
                move_,
                rotate,
                scale,
                vector_scale,
                fade,
                parameter,
                move_y,
                colour,
                move_x,
                trigger,
                loop_,
                context(ParseCommandError::UnknownCommandType.into(), fail),
            )),
        )(s)?;

        Ok(Some(parse.1))
    }
}
