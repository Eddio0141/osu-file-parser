use nom::{
    bytes::complete::*,
    combinator::*,
    error::{context, VerboseError},
    sequence::*,
    IResult,
};

use self::context::*;

use super::{types::*, *};

use crate::{helper::nth_bit_state_i64, osu_file::Position, parsers::*};

pub fn hitobject(s: &str) -> IResult<&str, HitObject, VerboseError<&str>> {
    let hitsound = context(
        INVALID_HITSOUND,
        map_res(is_not(","), |f: &str| f.parse::<HitSound>()),
    );
    let mut hitsample = context(
        INVALID_HITSAMPLE,
        map_res(take_till(|_| false), |f: &str| f.parse::<HitSample>()),
    );

    let (s, (x, _, y, _, time, _, obj_type, _, hitsound, _)) = tuple((
        context(INVALID_X, comma_field_i32()),
        context(MISSING_Y, comma()),
        context(INVALID_Y, comma_field_i32()),
        context(MISSING_TIME, comma()),
        context(INVALID_TIME, comma_field_i32()),
        context(MISSING_OBJ_TYPE, comma()),
        context(INVALID_OBJ_TYPE, comma_field_i32()),
        context(MISSING_HITSOUND, comma()),
        hitsound,
        context(MISSING_OBJ_PARAMS, comma()),
    ))(s)?;

    let position = Position { x, y };

    let new_combo = nth_bit_state_i64(obj_type as i64, 2);
    let combo_skip_count = (obj_type >> 4 & 0b111) as u8;

    if nth_bit_state_i64(obj_type as i64, 0) {
        let (s, hitsample) = hitsample(s)?;

        // hitcircle
        Ok((
            s,
            HitObject {
                position,
                time,
                obj_params: HitObjectParams::HitCircle,
                new_combo,
                combo_skip_count,
                hitsound,
                hitsample,
            },
        ))
    } else if nth_bit_state_i64(obj_type as i64, 1) {
        // slider
        let pipe = char('|');
        let curve_type = context(
            INVALID_CURVE_TYPE,
            map_res(is_not("|"), |f: &str| f.parse::<CurveType>()),
        );
        let decimal = map_res(is_not(","), |f: &str| f.parse::<Decimal>());
        let curve_points = context(
            INVALID_CURVE_POINTS,
            pipe_vec(|s: &str| s.parse::<CurvePoint>()),
        );
        let edge_sounds = context(
            INVALID_EDGE_SOUNDS,
            pipe_vec(|s: &str| s.parse::<HitSound>()),
        );
        let edge_sets = context(INVALID_EDGE_SETS, pipe_vec(|s: &str| s.parse::<EdgeSet>()));

        let (
            s,
            (curve_type, _, curve_points, slides, _, length, _, edge_sounds, edge_sets, hitsample),
        ) = tuple((
            curve_type,
            context(MISSING_CURVE_POINTS, pipe),
            curve_points,
            context(INVALID_SLIDES, comma_field_i32()),
            context(MISSING_LENGTH, comma()),
            context(INVALID_LENGTH, decimal),
            context(MISSING_EDGE_SOUNDS, comma()),
            edge_sounds,
            edge_sets,
            hitsample,
        ))(s)?;

        Ok((
            s,
            HitObject {
                position,
                time,
                obj_params: HitObjectParams::Slider {
                    curve_type,
                    curve_points,
                    slides,
                    length,
                    edge_sounds,
                    edge_sets,
                },
                new_combo,
                combo_skip_count,
                hitsound,
                hitsample,
            },
        ))
    } else if nth_bit_state_i64(obj_type as i64, 3) {
        // spinner
        let (s, (end_time, _, hitsample)) = tuple((
            context(INVALID_END_TIME, comma_field_i32()),
            context(MISSING_HITSAMPLE, comma()),
            hitsample,
        ))(s)?;

        Ok((
            s,
            HitObject {
                position,
                time,
                obj_params: HitObjectParams::Spinner { end_time },
                new_combo,
                combo_skip_count,
                hitsound,
                hitsample,
            },
        ))
    } else if nth_bit_state_i64(obj_type as i64, 7) {
        // osu!mania hold
        // ppy has done it once again
        let end_time = context(
            INVALID_END_TIME,
            map_res(take_until(":"), |s: &str| s.parse()),
        );
        let (s, (end_time, _, hitsample)) =
            tuple((end_time, context(MISSING_HITSAMPLE, char(':')), hitsample))(s)?;

        Ok((
            s,
            HitObject {
                position,
                time,
                obj_params: HitObjectParams::OsuManiaHold { end_time },
                new_combo,
                combo_skip_count,
                hitsound,
                hitsample,
            },
        ))
    } else {
        // osu file format didn't specify what to do with no bit flags set
        return context(UNKNOWN_OBJ_TYPE, fail)(s);
    }
}

// TODO make this an enum
pub mod context {
    // invalid value
    pub const INVALID_X: &str = "INVALID_X";
    pub const INVALID_Y: &str = "INVALID_Y";
    pub const INVALID_TIME: &str = "INVALID_TIME";
    pub const INVALID_OBJ_TYPE: &str = "INVALID_OBJ_TYPE";
    pub const INVALID_HITSOUND: &str = "INVALID_HITSOUND";
    pub const INVALID_HITSAMPLE: &str = "INVALID_HITSAMPLE";
    pub const INVALID_CURVE_TYPE: &str = "INVALID_CURVE_TYPE";
    pub const INVALID_CURVE_POINTS: &str = "INVALID_CURVE_POINTS";
    pub const INVALID_EDGE_SOUNDS: &str = "INVALID_EDGE_SOUNDS";
    pub const INVALID_EDGE_SETS: &str = "INVALID_EDGE_SETS";
    pub const INVALID_SLIDES: &str = "INVALID_SLIDES";
    pub const INVALID_LENGTH: &str = "INVALID_LENGTH";
    pub const INVALID_END_TIME: &str = "INVALID_END_TIME";

    // missing value
    pub const MISSING_Y: &str = "MISSING_Y";
    pub const MISSING_TIME: &str = "MISSING_TIME";
    pub const MISSING_OBJ_TYPE: &str = "MISSING_OBJ_TYPE";
    pub const MISSING_HITSOUND: &str = "MISSING_HITSOUND";
    pub const MISSING_OBJ_PARAMS: &str = "MISSING_OBJ_PARAMS";
    pub const MISSING_HITSAMPLE: &str = "MISSING_HITSAMPLE";
    pub const MISSING_CURVE_POINTS: &str = "MISSING_CURVE_POINTS";
    pub const MISSING_EDGE_SOUNDS: &str = "MISSING_EDGE_SOUNDS";
    pub const MISSING_LENGTH: &str = "MISSING_LENGTH";

    // other
    pub const UNKNOWN_OBJ_TYPE: &str = "UNKNOWN_OBJ_TYPE";
}
