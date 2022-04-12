use nom::{branch::*, bytes::complete::*, combinator::*, error::*, multi::*, sequence::*, *};

use crate::osu_file::parsers::*;

use self::context::*;

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
                context(MISSING_START_TIME, comma()),
                context(INVALID_START_TIME, comma_field_i32()),
                context(MISSING_LOOP_COUNT, comma()),
                context(
                    INVALID_LOOP_COUNT,
                    map_res(take_while(|_| true), |s: &str| s.parse()),
                ),
                context(EOF, eof),
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
                    context(MISSING_TRIGGER_TYPE, comma()),
                    context(
                        INVALID_TRIGGER_TYPE,
                        map_res(comma_field(), |s: &str| s.parse()),
                    ),
                    context(MISSING_START_TIME, comma()),
                    context(INVALID_START_TIME, comma_field_i32()),
                    context(MISSING_END_TIME, comma()),
                    context(
                        INVALID_END_TIME,
                        alt((
                            comma_field_i32().map(Some),
                            verify(comma_field(), |t: &str| t.is_empty()).map(|_| None),
                        )),
                    ),
                    context(
                        INVALID_GROUP_NUMBER,
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
                context(MISSING_EASING, comma()),
                context(
                    INVALID_EASING,
                    map_opt(comma_field_i32(), |s| Easing::from_repr(s as usize)),
                ),
                context(MISSING_START_TIME, comma()),
                context(INVALID_START_TIME, comma_field_i32()),
                context(MISSING_END_TIME, comma()),
                context(
                    INVALID_END_TIME,
                    alt((
                        comma_field_i32().map(Some),
                        verify(comma_field(), |t: &str| t.is_empty()).map(|_| None),
                    )),
                ),
                context(MISSING_ADDITIONAL_FIELDS, comma()),
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
                            context(INVALID_COLOUR, field_u8()),
                            context(MISSING_BLUE_FIELD, comma()),
                            context(INVALID_COLOUR, field_u8()),
                            context(MISSING_GREEN_FIELD, comma()),
                            context(INVALID_COLOUR, field_u8()),
                            continuing_colours,
                            context(INVALID_CONTINUING_U8, eof),
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
                        context(INVALID_PARAMETER_TYPE, parameter()),
                        continuing_parameters,
                        context(INVALID_PARAMETER_TYPE, eof),
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
                                    context(INVALID_DECIMAL, decimal()),
                                    context(MISSING_SECOND_FIELD, comma()),
                                    context(INVALID_DECIMAL, decimal()),
                                    continuing_fields,
                                    context(INVALID_CONTINUING_DECIMAL, eof),
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
                            let continuing = many0(preceded(comma(), decimal()));

                            let (_, (start, continuing, _)) = tuple((
                                context(INVALID_DECIMAL, decimal()),
                                continuing,
                                context(INVALID_CONTINUING_DECIMAL, eof),
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
                                _ => context(UNKNOWN_EVENT, fail)(s_input),
                            }
                        }
                    }
                }
            }
        }
    }
}

mod context {
    // missing field errors
    pub const MISSING_START_TIME: &str = "missing_start_time";
    pub const MISSING_END_TIME: &str = "missing_end_time";
    pub const MISSING_LOOP_COUNT: &str = "missing_loop_count";
    pub const MISSING_TRIGGER_TYPE: &str = "missing_trigger_type";
    pub const MISSING_EASING: &str = "missing_easing";
    pub const MISSING_ADDITIONAL_FIELDS: &str = "missing_additional_fields";
    pub const MISSING_GREEN_FIELD: &str = "missing_green_field";
    pub const MISSING_BLUE_FIELD: &str = "missing_blue_field";
    pub const MISSING_SECOND_FIELD: &str = "missing_second_field";

    // errors for trying to parse a string as a type
    pub const INVALID_START_TIME: &str = "invalid_start_time";
    pub const INVALID_END_TIME: &str = "invalid_end_time";
    pub const INVALID_LOOP_COUNT: &str = "invalid_loop_count";
    pub const INVALID_TRIGGER_TYPE: &str = "invalid_trigger_type";
    pub const INVALID_GROUP_NUMBER: &str = "invalid_group_number";
    pub const INVALID_EASING: &str = "invalid_easing";
    pub const INVALID_COLOUR: &str = "invalid_colour";
    pub const INVALID_PARAMETER_TYPE: &str = "invalid_parameter_type";
    pub const INVALID_DECIMAL: &str = "invalid_decimal";

    // invalid continuing fields
    pub const INVALID_CONTINUING_U8: &str = "invalid_continuing_u8";
    pub const INVALID_CONTINUING_DECIMAL: &str = "invalid_continuing_decimal";

    pub const EOF: &str = "eof";
    pub const UNKNOWN_EVENT: &str = "unknown_event";
}
