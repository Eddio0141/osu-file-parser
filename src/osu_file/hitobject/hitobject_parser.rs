use nom::{
    bytes::complete::*,
    combinator::*,
    error::{context, VerboseError},
    sequence::*,
    IResult,
};
use strum_macros::{EnumString, IntoStaticStr};

use super::{types::*, *};

use crate::{helper::nth_bit_state_i64, osu_file::Position, parsers::*};

pub fn hitobject(s: &str) -> IResult<&str, HitObject, VerboseError<&str>> {
    let hitsound = context(
        Context::InvalidHitsound.into(),
        map_res(is_not(","), |f: &str| f.parse::<HitSound>()),
    );
    let mut hitsample = context(
        Context::InvalidHitsample.into(),
        map_res(take_till(|_| false), |f: &str| f.parse::<HitSample>()),
    );

    let (s, (x, _, y, _, time, _, obj_type, _, hitsound, _)) = tuple((
        context(Context::InvalidX.into(), comma_field_i32()),
        context(Context::MissingY.into(), comma()),
        context(Context::InvalidY.into(), comma_field_i32()),
        context(Context::MissingTime.into(), comma()),
        context(Context::InvalidTime.into(), comma_field_i32()),
        context(Context::MissingObjType.into(), comma()),
        context(Context::InvalidObjType.into(), comma_field_i32()),
        context(Context::MissingHitsound.into(), comma()),
        hitsound,
        context(Context::MissingObjParams.into(), comma()),
    ))(s)?;

    let position = Position { x, y };

    let new_combo = nth_bit_state_i64(obj_type as i64, 2);
    let combo_skip_count = ComboSkipCount::try_from((obj_type >> 4 & 0b111) as u8).unwrap();

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
            Context::InvalidCurveType.into(),
            map_res(is_not("|"), |f: &str| f.parse::<CurveType>()),
        );
        let decimal = map_res(is_not(","), |f: &str| f.parse::<Decimal>());
        let curve_points = context(
            Context::InvalidCurvePoints.into(),
            pipe_vec(|s: &str| s.parse::<CurvePoint>()),
        );
        let edge_sounds = context(
            Context::InvalidEdgeSounds.into(),
            pipe_vec(|s: &str| s.parse::<HitSound>()),
        );
        let edge_sets = context(
            Context::InvalidEdgeSets.into(),
            pipe_vec(|s: &str| s.parse::<EdgeSet>()),
        );

        let (
            s,
            (curve_type, _, curve_points, slides, _, length, _, edge_sounds, edge_sets, hitsample),
        ) = tuple((
            curve_type,
            context(Context::MissingCurvePoints.into(), pipe),
            curve_points,
            context(Context::InvalidSlides.into(), comma_field_i32()),
            context(Context::MissingLength.into(), comma()),
            context(Context::InvalidLength.into(), decimal),
            context(Context::MissingEdgeSounds.into(), comma()),
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
            context(Context::InvalidEndTime.into(), comma_field_i32()),
            context(Context::MissingHitsample.into(), comma()),
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
            Context::InvalidEndTime.into(),
            map_res(take_until(":"), |s: &str| s.parse()),
        );
        let (s, (end_time, _, hitsample)) = tuple((
            end_time,
            context(Context::MissingHitsample.into(), char(':')),
            hitsample,
        ))(s)?;

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
        return context(Context::UnknownObjType.into(), fail)(s);
    }
}

#[derive(Debug, EnumString, IntoStaticStr)]
pub enum Context {
    InvalidX,
    InvalidY,
    InvalidTime,
    InvalidObjType,
    InvalidCurveType,
    InvalidCurvePoints,
    InvalidSlides,
    InvalidLength,
    InvalidEndTime,
    InvalidHitsound,
    InvalidHitsample,
    InvalidEdgeSounds,
    InvalidEdgeSets,
    MissingY,
    MissingTime,
    MissingObjType,
    MissingCurveType,
    MissingCurvePoints,
    MissingSlides,
    MissingLength,
    MissingEndTime,
    MissingHitsound,
    MissingHitsample,
    MissingEdgeSounds,
    MissingEdgeSets,
    MissingObjParams,
    UnknownObjType,
}
