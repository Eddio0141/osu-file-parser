use nom::{branch::*, bytes::complete::*, combinator::*, error::*, multi::*, sequence::*, *};

use crate::osu_file::parsers::*;

use super::{cmds::*, types::*};

pub fn command(s_input: &str) -> IResult<&str, Command, VerboseError<&str>> {
    let indentation = take_while::<_, _, VerboseError<_>>(|c| c == ' ' || c == '_');
    // let ints = || tuple((many0(preceded(comma(), comma_field_i32())), eof));

    // only parse a single command
    // a command type will never be missing
    // even an empty string will mean it is a command type of being empty
    let (s, command_type) = preceded(indentation, comma_field())(s_input).unwrap();

    // handle generic commands
    match command_type {
        "L" => {
            let (s, (_, start_time, _, loop_count, _)) = tuple((
                context("missing_start_time", comma()),
                context("start_time", comma_field_i32()),
                context("missing_loop_count", comma()),
                context(
                    "loop_count",
                    map_res(take_while(|_| true), |s: &str| s.parse()),
                ),
                context("eof", eof),
            ))(s)?;

            Ok((
                s,
                Command {
                    start_time,
                    properties: CommandProperties::Loop {
                        loop_count,
                        commands: Vec::new(),
                    },
                },
            ))
        }
        "T" => {
            let (s, (_, trigger_type, _, start_time, _, end_time, group_number)) =
                tuple((
                    context("missing_trigger_type", comma()),
                    context("trigger_type", map_res(comma_field(), |s: &str| s.parse())),
                    context("missing_start_time", comma()),
                    context("start_time", comma_field_i32()),
                    context("missing_end_time", comma()),
                    context(
                        "end_time",
                        alt((
                            comma_field_i32().map(Some),
                            verify(comma_field(), |t: &str| t.is_empty()).map(|_| None),
                        )),
                    ),
                    context(
                        "group_number",
                        alt((
                            eof.map(|_| None),
                            preceded(comma(), comma_field_i32()).map(Some),
                        )),
                    ),
                ))(s)?;

            Ok((
                s,
                Command {
                    start_time,
                    properties: CommandProperties::Trigger {
                        trigger_type,
                        end_time,
                        group_number,
                        commands: Vec::new(),
                    },
                },
            ))
        }
        _ => {
            let (s, (_, easing, _, start_time, _, end_time, _)) = tuple((
                context("missing_easing", comma()),
                context(
                    "easing",
                    map_opt(comma_field_i32(), |s| Easing::from_repr(s as usize)),
                ),
                context("missing_start_time", comma()),
                context("start_time", comma_field_i32()),
                context("missing_end_time", comma()),
                context(
                    "end_time",
                    alt((
                        comma_field_i32().map(Some),
                        verify(comma_field(), |t: &str| t.is_empty()).map(|_| None),
                    )),
                ),
                context("missing_params", comma()),
            ))(s)?;

            // divided into more common fields
            // those fields either have 1 property up to 3, which is almost all decimal types, other than the colour fields and the parameter fields
            match command_type {
                "C" => {
                    // colour
                    let field_u8 = || map_res(comma_field(), |s: &str| s.parse::<u8>());

                    let continuing_colour = || {
                        alt((
                            eof.map(|_| None),
                            preceded(comma(), field_u8()).map(|v| Some(v)),
                        ))
                    };
                    let continuing_colours = many0(preceded(
                        comma(),
                        tuple((field_u8(), continuing_colour(), continuing_colour())),
                    ));

                    let (s, (start_r, _, start_g, _, start_b, continuing_colours, _)) =
                        tuple((
                            context("invalid_red_field", field_u8()),
                            context("missing_blue_field", comma()),
                            context("invalid_blue_field", field_u8()),
                            context("missing_green_field", comma()),
                            context("invalid_green_field", field_u8()),
                            continuing_colours,
                            context("invalid_colour", eof),
                        ))(s)?;

                    Ok((
                        s,
                        Command {
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
                        },
                    ))
                }
                "P" => {
                    // parameter
                    let parameter = || map_res(comma_field(), |s: &str| s.parse());
                    // TODO whats going to happen with the error with the many0
                    let continuing_parameters = many0(preceded(comma(), parameter()));

                    let (s, (parameter, continuing_parameters, _)) = tuple((
                        context("parameter_type", parameter()),
                        continuing_parameters,
                        context("invalid_parameter", eof),
                    ))(s)?;

                    Ok((
                        s,
                        Command {
                            start_time,
                            properties: CommandProperties::Parameter {
                                easing,
                                end_time,
                                parameter,
                                continuing_parameters,
                            },
                        },
                    ))
                }
                _ => {
                    let decimal = || map_res(comma_field(), |s: &str| s.parse());

                    // divided into types with 1 continuous field and 2 fields thats continuous
                    match command_type {
                        "M" | "V" => {
                            let continuing = || {
                                alt((
                                    eof.map(|_| None),
                                    preceded(comma(), decimal()).map(|v| Some(v)),
                                ))
                            };
                            // TODO check error on many0
                            let continuing_fields =
                                many0(preceded(comma(), tuple((decimal(), continuing()))));

                            let (s, (start_1, _, start_2, continuing, _)) =
                                tuple((
                                    context("first_field", decimal()),
                                    context("missing_second_field", comma()),
                                    context("second_field", decimal()),
                                    continuing_fields,
                                    context("invalid_dec", eof),
                                ))(s)?;

                            let continuing_fields =
                                ContinuingFields::new((start_1, start_2), continuing).unwrap();

                            match command_type {
                                "M" => Ok((
                                    s,
                                    Command {
                                        start_time,
                                        properties: CommandProperties::Move {
                                            easing,
                                            end_time,
                                            positions_xy: continuing_fields,
                                        },
                                    },
                                )),
                                "V" => Ok((
                                    s,
                                    Command {
                                        start_time,
                                        properties: CommandProperties::VectorScale {
                                            easing,
                                            end_time,
                                            scales_xy: continuing_fields,
                                        },
                                    },
                                )),
                                _ => unreachable!(),
                            }
                        }
                        // this is where the unreachable event type gets handled too
                        _ => {
                            // TODO this error is not handled properly
                            let continuing = many0(preceded(comma(), decimal()));

                            let (_, (start, continuing, _)) = tuple((
                                context("first_parameter", decimal()),
                                context("continuing_parameters", continuing),
                                context("eof", eof),
                            ))(s)?;

                            match command_type {
                                "F" => Ok((
                                    s,
                                    Command {
                                        start_time,
                                        properties: CommandProperties::Fade {
                                            easing,
                                            end_time,
                                            start_opacity: start,
                                            continuing_opacities: continuing,
                                        },
                                    },
                                )),
                                "MX" => Ok((
                                    s,
                                    Command {
                                        start_time,
                                        properties: CommandProperties::MoveX {
                                            easing,
                                            end_time,
                                            start_x: start,
                                            continuing_x: continuing,
                                        },
                                    },
                                )),
                                "MY" => Ok((
                                    s,
                                    Command {
                                        start_time,
                                        properties: CommandProperties::MoveY {
                                            easing,
                                            end_time,
                                            start_y: start,
                                            continuing_y: continuing,
                                        },
                                    },
                                )),
                                "S" => Ok((
                                    s,
                                    Command {
                                        start_time,
                                        properties: CommandProperties::Scale {
                                            easing,
                                            end_time,
                                            start_scale: start,
                                            continuing_scales: continuing,
                                        },
                                    },
                                )),
                                "R" => Ok((
                                    s,
                                    Command {
                                        start_time,
                                        properties: CommandProperties::Rotate {
                                            easing,
                                            end_time,
                                            start_rotation: start,
                                            continuing_rotations: continuing,
                                        },
                                    },
                                )),
                                _ => context("unknown_event", fail)(s_input),
                            }
                        }
                    }
                }
            }
        }
    }
}
