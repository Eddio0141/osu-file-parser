use std::str::FromStr;

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
    OsuFile, Position, VersionedFromStr, VersionedToString,
};

fn storyboard_cmds_parse(c: &mut Criterion) {
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

    let mut group = c.benchmark_group("storyboard_cmds_parse");

    group.bench_function("fade", |b| {
        b.iter(|| {
            Command::from_str(black_box(fade_str), black_box(14)).unwrap();
        })
    });
    group.bench_function("move", |b| {
        b.iter(|| {
            Command::from_str(black_box(move_str), black_box(14)).unwrap();
        })
    });
    group.bench_function("move_x", |b| {
        b.iter(|| {
            Command::from_str(black_box(move_x_str), black_box(14)).unwrap();
        })
    });
    group.bench_function("move_y", |b| {
        b.iter(|| {
            Command::from_str(black_box(move_y_str), black_box(14)).unwrap();
        })
    });
    group.bench_function("scale", |b| {
        b.iter(|| {
            Command::from_str(black_box(scale_str), black_box(14)).unwrap();
        })
    });
    group.bench_function("vector_scale", |b| {
        b.iter(|| {
            Command::from_str(black_box(vector_scale_str), black_box(14)).unwrap();
        })
    });
    group.bench_function("rotate", |b| {
        b.iter(|| {
            Command::from_str(black_box(rotate_str), black_box(14)).unwrap();
        })
    });
    group.bench_function("colour", |b| {
        b.iter(|| {
            Command::from_str(black_box(colour_str), black_box(14)).unwrap();
        })
    });
    group.bench_function("parameter", |b| {
        b.iter(|| {
            Command::from_str(black_box(parameter_str), black_box(14)).unwrap();
        })
    });
    group.bench_function("loop", |b| {
        b.iter(|| {
            Command::from_str(black_box(loop_str), black_box(14)).unwrap();
        })
    });
    group.bench_function("trigger", |b| {
        b.iter(|| {
            Command::from_str(black_box(trigger_str), black_box(14)).unwrap();
        })
    });
}

fn storyboard_cmds_to_string(c: &mut Criterion) {
    let fade = Command::from_str("F,0,500,1000,0,0.5", 14)
        .unwrap()
        .unwrap();
    let move_ = Command::from_str("M,0,500,1000,0,1,2,3", 14)
        .unwrap()
        .unwrap();
    let move_x = Command::from_str("MX,0,500,1000,0,1", 14).unwrap().unwrap();
    let move_y = Command::from_str("MY,0,500,1000,0,1", 14).unwrap().unwrap();
    let scale = Command::from_str("S,0,500,1000,0,0.5", 14)
        .unwrap()
        .unwrap();
    let vector_scale = Command::from_str("V,0,500,1000,0,0,0.5,0.5", 14)
        .unwrap()
        .unwrap();
    let rotate = Command::from_str("R,0,500,1000,0,0.5", 14)
        .unwrap()
        .unwrap();
    let colour = Command::from_str("C,0,500,1000,0,0,0,255,255,255", 14)
        .unwrap()
        .unwrap();
    let parameter = Command::from_str("P,0,500,1000,H", 14).unwrap().unwrap();
    let loop_ = Command::from_str("L,0,10", 14).unwrap().unwrap();
    let trigger = Command::from_str("T,HitSound,500,1000", 14)
        .unwrap()
        .unwrap();

    let mut group = c.benchmark_group("storyboard_cmds_to_string");

    group.bench_function("fade", |b| {
        b.iter(|| {
            black_box(&fade).to_string(14);
        })
    });
    group.bench_function("move", |b| {
        b.iter(|| {
            black_box(&move_).to_string(14);
        })
    });
    group.bench_function("move_x", |b| {
        b.iter(|| {
            black_box(&move_x).to_string(14);
        })
    });
    group.bench_function("move_y", |b| {
        b.iter(|| {
            black_box(&move_y).to_string(14);
        })
    });
    group.bench_function("scale", |b| {
        b.iter(|| {
            black_box(&scale).to_string(14);
        })
    });
    group.bench_function("vector_scale", |b| {
        b.iter(|| {
            black_box(&vector_scale).to_string(14);
        })
    });
    group.bench_function("rotate", |b| {
        b.iter(|| {
            black_box(&rotate).to_string(14);
        })
    });
    group.bench_function("colour", |b| {
        b.iter(|| {
            black_box(&colour).to_string(14);
        })
    });
    group.bench_function("parameter", |b| {
        b.iter(|| {
            black_box(&parameter).to_string(14);
        })
    });
    group.bench_function("loop", |b| {
        b.iter(|| {
            black_box(&loop_).to_string(14);
        })
    });
    group.bench_function("trigger", |b| {
        b.iter(|| {
            black_box(&trigger).to_string(14);
        })
    });
}

fn storyboard_loop_cmd_to_string(c: &mut Criterion) {
    let loop_cmd = |commands| Command {
        start_time: 0,
        properties: CommandProperties::Loop {
            loop_count: 5,
            commands,
        },
    };

    let event = Event::StoryboardObject(Object {
        layer: Layer::Background,
        origin: Origin::BottomCentre,
        position: Position::default(),
        object_type: ObjectType::Sprite(Sprite {
            filepath: "".into(),
        }),
        commands: vec![loop_cmd(vec![loop_cmd(vec![loop_cmd(vec![loop_cmd(
            vec![loop_cmd(Vec::new())],
        )])])])],
    });

    c.bench_function("storyboard_loop_cmd_to_string", |b| {
        b.iter(|| black_box(&event).to_string(14))
    });
}

fn hitobject_parse(c: &mut Criterion) {
    let hitcircle_str = "221,350,9780,1,0,0:0:0:0:";
    let slider_str = "31,85,3049,2,0,B|129:55|123:136|228:86,1,172.51,2|0,3:2|0:2,0:2:0:0:";
    let spinner_str = "256,192,33598,12,0,431279,0:0:0:0:";
    let osu_mania_hold_str = "51,192,350,128,2,849:0:0:0:0:";

    let mut group = c.benchmark_group("hitobjects_parse");

    group.bench_function("hitcircle", |b| {
        b.iter(|| {
            HitObject::from_str(black_box(hitcircle_str), 14)
                .unwrap()
                .unwrap();
        })
    });
    group.bench_function("slider", |b| {
        b.iter(|| {
            HitObject::from_str(black_box(slider_str), 14)
                .unwrap()
                .unwrap();
        })
    });
    group.bench_function("spinner", |b| {
        b.iter(|| {
            HitObject::from_str(black_box(spinner_str), 14)
                .unwrap()
                .unwrap();
        })
    });
    group.bench_function("osu_mania_hold", |b| {
        b.iter(|| {
            HitObject::from_str(black_box(osu_mania_hold_str), 14)
                .unwrap()
                .unwrap();
        })
    });
}

fn hitobject_to_string(c: &mut Criterion) {
    let hitcircle = HitObject::from_str("221,350,9780,1,0,0:0:0:0:", 14)
        .unwrap()
        .unwrap();
    let slider = HitObject::from_str(
        "31,85,3049,2,0,B|129:55|123:136|228:86,1,172.51,2|0,3:2|0:2,0:2:0:0:",
        14,
    )
    .unwrap()
    .unwrap();
    let spinner = HitObject::from_str("256,192,33598,12,0,431279,0:0:0:0:", 14)
        .unwrap()
        .unwrap();
    let osu_mania_hold = HitObject::from_str("51,192,350,128,2,849:0:0:0:0:", 14)
        .unwrap()
        .unwrap();

    let mut group = c.benchmark_group("hitobjects_to_string");

    group.bench_function("hitcircle", |b| {
        b.iter(|| black_box(&hitcircle).to_string(14))
    });
    group.bench_function("slider", |b| b.iter(|| black_box(&slider).to_string(14)));
    group.bench_function("spinner", |b| b.iter(|| black_box(&spinner).to_string(14)));
    group.bench_function("osu_mania_hold", |b| {
        b.iter(|| black_box(&osu_mania_hold).to_string(14))
    });
}

const ONE_HOUR_OSU: &str = include_str!("./files/1hr.osu");
const CRAZY_OSU: &str = include_str!("./files/crazy.osu");

fn files_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("files_parse");

    group.bench_function("1hr", |b| {
        b.iter(|| {
            black_box(ONE_HOUR_OSU).parse::<OsuFile>().unwrap();
        })
    });
    group.bench_function("crazy", |b| {
        b.iter(|| {
            black_box(CRAZY_OSU).parse::<OsuFile>().unwrap();
        })
    });
}

fn files_to_string(c: &mut Criterion) {
    let one_hour_osu = OsuFile::from_str(ONE_HOUR_OSU).unwrap();
    let crazy_osu = OsuFile::from_str(CRAZY_OSU).unwrap();

    let mut group = c.benchmark_group("files_to_string");

    group.bench_function("1hr", |b| b.iter(|| black_box(&one_hour_osu).to_string()));
    group.bench_function("crazy", |b| b.iter(|| black_box(&crazy_osu).to_string()));
}

const ASPIRE_OSB1: &str = include_str!("./files/aspire_osb1.osb");
const ASPIRE_OSB2: &str = include_str!("./files/aspire_osb2.osb");

fn aspire_osb_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("aspire_osb_parse");
    let mut aspire_osb1_osu = OsuFile::default(14);
    let mut aspire_osb2_osu = OsuFile::default(14);

    group.bench_function("aspire_osb1", |b| {
        b.iter(|| {
            aspire_osb1_osu.append_osb(black_box(ASPIRE_OSB1)).unwrap();
        })
    });
    group.bench_function("aspire_osb2", |b| {
        b.iter(|| {
            aspire_osb2_osu.append_osb(black_box(ASPIRE_OSB2)).unwrap();
        })
    });
}

fn aspire_osb_to_string(c: &mut Criterion) {
    let mut aspire_osb1_osu = OsuFile::default(14);
    aspire_osb1_osu.append_osb(ASPIRE_OSB1).unwrap();
    let mut aspire_osb2_osu = OsuFile::default(14);
    aspire_osb2_osu.append_osb(ASPIRE_OSB2).unwrap();

    let mut group = c.benchmark_group("aspire_osb_to_string");

    group.bench_function("aspire_osb1", |b| {
        b.iter(|| black_box(&aspire_osb1_osu).osb_to_string().unwrap())
    });
    group.bench_function("aspire_osb2", |b| {
        b.iter(|| black_box(&aspire_osb2_osu).osb_to_string().unwrap())
    });
}

const ASPIRE1: &str = include_str!("./files/aspire1.osu");
const ASPIRE2: &str = include_str!("./files/aspire2.osu");
const ASPIRE3: &str = include_str!("./files/aspire3.osu");
const ASPIRE4: &str = include_str!("./files/aspire4.osu");
const ASPIRE5: &str = include_str!("./files/aspire5.osu");
const ASPIRE6: &str = include_str!("./files/aspire6.osu");
const ASPIRE7: &str = include_str!("./files/aspire7.osu");
const ASPIRE8: &str = include_str!("./files/aspire8.osu");
const ASPIRE9: &str = include_str!("./files/aspire9.osu");
const ASPIRE10: &str = include_str!("./files/aspire10.osu");

fn aspire_files_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("aspire_files_parse");

    group.bench_function("aspire1", |b| {
        b.iter(|| {
            black_box(ASPIRE1).parse::<OsuFile>().unwrap();
        })
    });
    group.bench_function("aspire2", |b| {
        b.iter(|| {
            black_box(ASPIRE2).parse::<OsuFile>().unwrap();
        })
    });
    group.bench_function("aspire3", |b| {
        b.iter(|| {
            black_box(ASPIRE3).parse::<OsuFile>().unwrap();
        })
    });
    group.bench_function("aspire4", |b| {
        b.iter(|| {
            black_box(ASPIRE4).parse::<OsuFile>().unwrap();
        })
    });
    group.bench_function("aspire5", |b| {
        b.iter(|| {
            black_box(ASPIRE5).parse::<OsuFile>().unwrap();
        })
    });
    group.bench_function("aspire6", |b| {
        b.iter(|| {
            black_box(ASPIRE6).parse::<OsuFile>().unwrap();
        })
    });
    group.bench_function("aspire7", |b| {
        b.iter(|| {
            black_box(ASPIRE7).parse::<OsuFile>().unwrap();
        })
    });
    group.bench_function("aspire8", |b| {
        b.iter(|| {
            black_box(ASPIRE8).parse::<OsuFile>().unwrap();
        })
    });
    group.bench_function("aspire9", |b| {
        b.iter(|| {
            black_box(ASPIRE9).parse::<OsuFile>().unwrap();
        })
    });
    group.bench_function("aspire10", |b| {
        b.iter(|| {
            black_box(ASPIRE10).parse::<OsuFile>().unwrap();
        })
    });
}

fn aspire_files_to_string(c: &mut Criterion) {
    let aspire1_osu = OsuFile::from_str(ASPIRE1).unwrap();
    let aspire2_osu = OsuFile::from_str(ASPIRE2).unwrap();
    let aspire3_osu = OsuFile::from_str(ASPIRE3).unwrap();
    let aspire4_osu = OsuFile::from_str(ASPIRE4).unwrap();
    let aspire5_osu = OsuFile::from_str(ASPIRE5).unwrap();
    let aspire6_osu = OsuFile::from_str(ASPIRE6).unwrap();
    let aspire7_osu = OsuFile::from_str(ASPIRE7).unwrap();
    let aspire8_osu = OsuFile::from_str(ASPIRE8).unwrap();
    let aspire9_osu = OsuFile::from_str(ASPIRE9).unwrap();
    let aspire10_osu = OsuFile::from_str(ASPIRE10).unwrap();

    let mut group = c.benchmark_group("aspire_files_to_string");

    group.bench_function("aspire1", |b| {
        b.iter(|| black_box(&aspire1_osu).to_string())
    });
    group.bench_function("aspire2", |b| {
        b.iter(|| black_box(&aspire2_osu).to_string())
    });
    group.bench_function("aspire3", |b| {
        b.iter(|| black_box(&aspire3_osu).to_string())
    });
    group.bench_function("aspire4", |b| {
        b.iter(|| black_box(&aspire4_osu).to_string())
    });
    group.bench_function("aspire5", |b| {
        b.iter(|| black_box(&aspire5_osu).to_string())
    });
    group.bench_function("aspire6", |b| {
        b.iter(|| black_box(&aspire6_osu).to_string())
    });
    group.bench_function("aspire7", |b| {
        b.iter(|| black_box(&aspire7_osu).to_string())
    });
    group.bench_function("aspire8", |b| {
        b.iter(|| black_box(&aspire8_osu).to_string())
    });
    group.bench_function("aspire9", |b| {
        b.iter(|| black_box(&aspire9_osu).to_string())
    });
    group.bench_function("aspire10", |b| {
        b.iter(|| black_box(&aspire10_osu).to_string())
    });
}

criterion_group!(
    benches,
    storyboard_cmds_parse,
    storyboard_cmds_to_string,
    storyboard_loop_cmd_to_string,
    hitobject_parse,
    hitobject_to_string,
    files_parse,
    files_to_string,
    aspire_osb_parse,
    aspire_osb_to_string,
    aspire_files_parse,
    aspire_files_to_string,
);
criterion_main!(benches);
