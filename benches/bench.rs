use std::{path::PathBuf, str::FromStr};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use osu_file_parser::osu_file::{
    events::{
        storyboard::{
            cmds::{Command, CommandProperties},
            sprites::{Layer, Object, ObjectType, Origin, Sprite},
        },
        Event,
    },
    hitobjects::HitObject,
    OsuFile, Position,
};

fn storyboard_cmds_bench(c: &mut Criterion) {
    let fade_str = "F,0,500,1000,0,0.5";
    let move_str = "M,0,500,1000,0,1,2,3";
    let move_x_str = "MX,0,500,1000,0,1";
    let move_y_str = "MY,0,500,1000,0,1";
    let scale_str = "S,0,500,1000,0,0.5";
    let vector_scale_str = "V,0,500,1000,0,0,0.5,0.5";
    let rotate_str = "R,0,500,1000,0,0.5";
    let colour_str = "C,0,500,1000,0,0,0,255,255,255";
    let parameter_str = "P,0,500,1000,H";
    let loop_str = "L,0,10";
    let trigger_str = "T,HitSound,500,1000";

    let mut group = c.benchmark_group("storyboard_cmds");

    group.bench_function("fade_cmd", |b| {
        b.iter(|| {
            Command::from_str(black_box(fade_str)).unwrap();
        })
    });
    group.bench_function("move_cmd", |b| {
        b.iter(|| {
            Command::from_str(black_box(move_str)).unwrap();
        })
    });
    group.bench_function("move_x_cmd", |b| {
        b.iter(|| {
            Command::from_str(black_box(move_x_str)).unwrap();
        })
    });
    group.bench_function("move_y_cmd", |b| {
        b.iter(|| {
            Command::from_str(black_box(move_y_str)).unwrap();
        })
    });
    group.bench_function("scale_cmd", |b| {
        b.iter(|| {
            Command::from_str(black_box(scale_str)).unwrap();
        })
    });
    group.bench_function("vector_scale_cmd", |b| {
        b.iter(|| {
            Command::from_str(black_box(vector_scale_str)).unwrap();
        })
    });
    group.bench_function("rotate_cmd", |b| {
        b.iter(|| {
            Command::from_str(black_box(rotate_str)).unwrap();
        })
    });
    group.bench_function("colour_cmd", |b| {
        b.iter(|| {
            Command::from_str(black_box(colour_str)).unwrap();
        })
    });
    group.bench_function("parameter_cmd", |b| {
        b.iter(|| {
            Command::from_str(black_box(parameter_str)).unwrap();
        })
    });
    group.bench_function("loop_cmd", |b| {
        b.iter(|| {
            Command::from_str(black_box(loop_str)).unwrap();
        })
    });
    group.bench_function("trigger_cmd", |b| {
        b.iter(|| {
            Command::from_str(black_box(trigger_str)).unwrap();
        })
    });
}

fn storyboard_loop_cmd_display(c: &mut Criterion) {
    let loop_cmd = |commands| Command {
        start_time: 0,
        properties: CommandProperties::Loop {
            loop_count: 5,
            commands,
        },
    };

    let event = Event::Storyboard(Object {
        layer: Layer::Background,
        origin: Origin::BottomCentre,
        position: Position::default(),
        object_type: ObjectType::Sprite(Sprite {
            filepath: PathBuf::new(),
        }),
        commands: vec![loop_cmd(vec![loop_cmd(vec![loop_cmd(vec![loop_cmd(
            vec![loop_cmd(Vec::new())],
        )])])])],
    });

    c.bench_function("loop_cmd_display", |b| {
        b.iter(|| black_box(&event).to_string())
    });
}

fn hitobject_parse_bench(c: &mut Criterion) {
    let hitcircle_str = "221,350,9780,1,0,0:0:0:0:";
    let slider_str = "31,85,3049,2,0,B|129:55|123:136|228:86,1,172.51,2|0,3:2|0:2,0:2:0:0:";
    let spinner_str = "256,192,33598,12,0,431279,0:0:0:0:";
    let osu_mania_hold_str = "51,192,350,128,2,849:0:0:0:0:";

    let mut group = c.benchmark_group("hitobjects_parse");

    group.bench_function("hitcircle_parse", |b| {
        b.iter(|| {
            black_box(hitcircle_str).parse::<HitObject>().unwrap();
        })
    });
    group.bench_function("slider_parse", |b| {
        b.iter(|| {
            black_box(slider_str).parse::<HitObject>().unwrap();
        })
    });
    group.bench_function("spinner_parse", |b| {
        b.iter(|| {
            black_box(spinner_str).parse::<HitObject>().unwrap();
        })
    });
    group.bench_function("osu_mania_hold_parse", |b| {
        b.iter(|| {
            black_box(osu_mania_hold_str).parse::<HitObject>().unwrap();
        })
    });
}

const ONE_HOUR_OSU: &str = include_str!("./files/1hr.osu");
const CRAZY_OSU: &str = include_str!("./files/crazy.osu");

fn files_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("files_parse");

    group.bench_function("1hr_parse", |b| {
        b.iter(|| {
            black_box(ONE_HOUR_OSU).parse::<OsuFile>().unwrap();
        })
    });
    group.bench_function("crazy_parse", |b| {
        b.iter(|| {
            black_box(CRAZY_OSU).parse::<OsuFile>().unwrap();
        })
    });
}

criterion_group!(
    benches,
    storyboard_cmds_bench,
    storyboard_loop_cmd_display,
    hitobject_parse_bench,
    files_bench,
);
criterion_main!(benches);
