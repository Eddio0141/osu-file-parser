#[cfg(test)]
use pretty_assertions::assert_eq;
use std::{
    num::NonZeroUsize,
    path::{Path, PathBuf},
};

use rust_decimal_macros::dec;

use crate::osu_file::{
    colours::{Colour, Rgb},
    difficulty::Difficulty,
    editor::Editor,
    events::{
        storyboard::{
            self, Addition, Animation, Command, CommandProperties, Easing, Layer, LoopType, Object,
            ObjectType, Origin, Parameter, Sprite, TriggerType,
        },
        Background, Break, Event, EventParams, Events,
    },
    general::{CountdownSpeed, GameMode, General, OverlayPosition, SampleSet},
    hitobject::{
        self,
        types::{HitSample, HitSound},
        HitObjects,
    },
    metadata::Metadata,
    timingpoint::{self, Effects, SampleIndex, TimingPoint, TimingPoints, Volume},
    OsuFile, Position,
};

#[test]
fn general_parse() {
    let i = "AudioFilename: test.mp3
AudioLeadIn: 555
AudioHash: no.mp3
PreviewTime: 5
Countdown: 3
SampleSet: Soft
StackLeniency: 0.9
Mode: 1
LetterboxInBreaks: 1
StoryFireInFront: 0
UseSkinSprites: 1
AlwaysShowPlayfield: 0
OverlayPosition: Above
SkinPreference: myskin
EpilepsyWarning: 1
CountdownOffset: 120
SpecialStyle: 1
WidescreenStoryboard: 1
SamplesMatchPlaybackRate: 1";
    let i = i.parse::<General>().unwrap();

    let g = General {
        audio_filename: Some("test.mp3".to_string()),
        audio_lead_in: Some(555),
        audio_hash: Some("no.mp3".to_string()),
        preview_time: Some(5),
        countdown: Some(CountdownSpeed::Double),
        sample_set: Some(SampleSet::Soft),
        stack_leniency: Some(dec!(0.9)),
        mode: Some(GameMode::Taiko),
        letterbox_in_breaks: Some(true),
        story_fire_in_front: Some(false),
        use_skin_sprites: Some(true),
        always_show_playfield: Some(false),
        overlay_position: Some(OverlayPosition::Above),
        skin_preference: Some("myskin".to_string()),
        epilepsy_warning: Some(true),
        countdown_offset: Some(120),
        special_style: Some(true),
        widescreen_storyboard: Some(true),
        samples_match_playback_rate: Some(true),
    };

    assert_eq!(i, g);
}

#[test]
fn general_parse_back() {
    let i = "AudioFilename: test.mp3
AudioLeadIn: 555
AudioHash: no.mp3
PreviewTime: 5
Countdown: 3
SampleSet: Soft
StackLeniency: 0.9
Mode: 1
LetterboxInBreaks: 1
StoryFireInFront: 0
UseSkinSprites: 1
AlwaysShowPlayfield: 0
OverlayPosition: Above
SkinPreference: myskin
EpilepsyWarning: 1
CountdownOffset: 120
SpecialStyle: 1
WidescreenStoryboard: 1
SamplesMatchPlaybackRate: 1";
    let g = i.parse::<General>().unwrap();

    assert_eq!(i, g.to_string());
}

#[test]
fn editor_parse() {
    let i = "Bookmarks: 11018,21683,32349,37683,48349,59016,69683,80349,91016
DistanceSpacing: 0.8
BeatDivisor: 12
GridSize: 8
TimelineZoom: 2";
    let i: Editor = i.parse().unwrap();

    let e = Editor {
        bookmarks: vec![
            11018, 21683, 32349, 37683, 48349, 59016, 69683, 80349, 91016,
        ],
        distance_spacing: dec!(0.8),
        beat_divisor: dec!(12),
        grid_size: 8,
        timeline_zoom: dec!(2),
    };

    assert_eq!(i, e);
}

#[test]
fn editor_parse_back() {
    let i = "Bookmarks: 11018,21683,32349,37683,48349,59016,69683,80349,91016
DistanceSpacing: 0.8
BeatDivisor: 12
GridSize: 8
TimelineZoom: 2";
    let e: Editor = i.parse().unwrap();

    assert_eq!(i, e.to_string());
}

#[test]
fn metadata_parse() {
    let i = "Title:LOVE IS ORANGE
TitleUnicode:LOVE IS ORANGE
Artist:Orange Lounge
ArtistUnicode:Orange Lounge
Creator:Xnery
Version:Bittersweet Love
Source:beatmania IIDX 8th style
Tags:famoss 舟木智介 tomosuke funaki 徳井志津江 videogame ハードシャンソン Tart&Toffee
BeatmapID:3072232
BeatmapSetID:1499093";
    let i: Metadata = i.parse().unwrap();

    let m = Metadata {
        title: "LOVE IS ORANGE".to_string(),
        title_unicode: "LOVE IS ORANGE".to_string(),
        artist: "Orange Lounge".to_string(),
        artist_unicode: "Orange Lounge".to_string(),
        creator: "Xnery".to_string(),
        version: "Bittersweet Love".to_string(),
        source: "beatmania IIDX 8th style".to_string(),
        tags: vec![
            "famoss".to_string(),
            "舟木智介".to_string(),
            "tomosuke".to_string(),
            "funaki".to_string(),
            "徳井志津江".to_string(),
            "videogame".to_string(),
            "ハードシャンソン".to_string(),
            "Tart&Toffee".to_string(),
        ],
        beatmap_id: 3072232,
        beatmap_set_id: 1499093,
    };

    assert_eq!(i, m);
}

#[test]
fn metadata_parse_back() {
    let i = "Title:LOVE IS ORANGE
TitleUnicode:LOVE IS ORANGE
Artist:Orange Lounge
ArtistUnicode:Orange Lounge
Creator:Xnery
Version:Bittersweet Love
Source:beatmania IIDX 8th style
Tags:famoss 舟木智介 tomosuke funaki 徳井志津江 videogame ハードシャンソン Tart&Toffee
BeatmapID:3072232
BeatmapSetID:1499093";
    let m: Metadata = i.parse().unwrap();

    assert_eq!(i, m.to_string());
}

#[test]
fn difficulty_parse() {
    let i = "HPDrainRate:8
CircleSize:5
OverallDifficulty:8
ApproachRate:5
SliderMultiplier:1.4
SliderTickRate:1";
    let i: Difficulty = i.parse().unwrap();

    let d = Difficulty {
        hp_drain_rate: dec!(8),
        circle_size: dec!(5),
        overall_difficulty: dec!(8),
        approach_rate: dec!(5),
        slider_multiplier: dec!(1.4),
        slider_tickrate: dec!(1),
    };

    assert_eq!(i, d);
}

#[test]
fn difficulty_parse_back() {
    let i = "HPDrainRate:8
CircleSize:5
OverallDifficulty:8
ApproachRate:5
SliderMultiplier:1.4
SliderTickRate:1";
    let d: Difficulty = i.parse().unwrap();

    assert_eq!(i, d.to_string());
}

#[test]
fn colours_parse() {
    let i = "Combo1 : 255,128,255
SliderTrackOverride : 100,99,70
SliderBorder : 120,130,140";
    let i: Vec<Colour> = i.lines().map(|line| line.parse().unwrap()).collect();

    let c = vec![
        Colour::Combo(
            1,
            Rgb {
                red: 255,
                green: 128,
                blue: 255,
            },
        ),
        Colour::SliderTrackOverride(Rgb {
            red: 100,
            green: 99,
            blue: 70,
        }),
        Colour::SliderBorder(Rgb {
            red: 120,
            green: 130,
            blue: 140,
        }),
    ];

    assert_eq!(i, c);
}

#[test]
fn colours_parse_back() {
    let i = "Combo1 : 255,128,255
SliderTrackOverride : 100,99,70
SliderBorder : 120,130,140";
    let c: Vec<Colour> = i.lines().map(|line| line.parse().unwrap()).collect();

    assert_eq!(
        i,
        c.iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn timing_points_parse() {
    let i = "10000,333.33,4,0,0,100,1,1
12000,-25,4,3,0,100,0,1";
    let i: Vec<TimingPoint> = i
        .lines()
        .map(|timing_point| timing_point.parse().unwrap())
        .collect();

    let t = vec![
        TimingPoint::new_uninherited(
            10000,
            dec!(333.33),
            4,
            timingpoint::SampleSet::BeatmapDefault,
            SampleIndex::OsuDefaultHitsounds,
            Volume::new(100).unwrap(),
            Effects {
                kiai_time_enabled: true,
                no_first_barline_in_taiko_mania: false,
            },
        ),
        TimingPoint::new_inherited(
            12000,
            dec!(4),
            4,
            timingpoint::SampleSet::Drum,
            SampleIndex::OsuDefaultHitsounds,
            Volume::new(100).unwrap(),
            Effects {
                kiai_time_enabled: true,
                no_first_barline_in_taiko_mania: false,
            },
        ),
    ];

    assert_eq!(i, t);
}

#[test]
fn timing_points_parse_back() {
    let i = "10000,333.33,4,0,0,100,1,1
12000,-25,4,3,0,100,0,1";
    let t: Vec<TimingPoint> = i
        .lines()
        .map(|timing_point| timing_point.parse().unwrap())
        .collect();

    assert_eq!(
        i,
        t.iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn events_parse() {
    let i = "0,0,\"bg2.jpg\",0,0
0,0,bg2.jpg,0,0
//Break Periods
2,100,163";
    let i: Events = i.parse().unwrap();

    let e = Events(vec![
        Event::NormalEvent {
            start_time: 0,
            event_params: EventParams::Background(Background::new(
                Path::new("\"bg2.jpg\""),
                Position { x: 0, y: 0 },
            )),
        },
        Event::NormalEvent {
            start_time: 0,
            event_params: EventParams::Background(Background::new(
                Path::new("bg2.jpg"),
                Position { x: 0, y: 0 },
            )),
        },
        Event::Comment("Break Periods".to_string()),
        Event::NormalEvent {
            start_time: 100,
            event_params: EventParams::Break(Break { end_time: 163 }),
        },
    ]);

    assert_eq!(i, e);
}

#[test]
fn events_parse_back() {
    let i = "0,0,\"bg2.jpg\",0,0
0,0,bg2.jpg,0,0
//Break Periods
2,100,163";
    let e: Events = i.parse().unwrap();

    assert_eq!(i, e.to_string());
}

#[test]
fn frame_file_names() {
    let animation = Object {
        layer: Layer::Background,
        origin: Origin::BottomCentre,
        position: Position { x: 0, y: 0 },
        object_type: ObjectType::Animation(Animation::new(
            4,
            0,
            LoopType::LoopForever,
            Path::new("testfile.png"),
        )),
        commands: Vec::new(),
    };

    if let ObjectType::Animation(animation) = &animation.object_type {
        let file_names = animation.frame_file_names();

        assert_eq!(
            file_names,
            vec![
                "testfile0.png",
                "testfile1.png",
                "testfile2.png",
                "testfile3.png",
            ]
        );
    } else {
        unreachable!();
    }
}

#[test]
fn storyboard_sprites_cmd_parse() {
    let i = "
Sprite,Pass,Centre,\"Text\\Play2-HaveFunH.png\",320,240
 F,0,-28,,1
 M,3,100,120,140,180.123123,200,200
_MX,3,100,120,140,180.123123
_MY,3,100,120,140,180.123123
 S,0,-28,,0.4
 V,8,5000,5500,0.5,2,2,0.5
 R,7,5000,5500,-0.785,0.785
 C,6,50000,50001,0,0,0,255,255,255
 P,5,300,350,H
 P,5,300,350,V
 P,5,300,350,A
 L,500,10
  L,10,10
   M,3,100,120,140,180.123123,200,200
___S,0,-28,,0.4
 T,HitSound,0,10
  L,10,10
   M,3,100,120,140,180.123123,200,200
 T,HitSoundClap,0,10
  M,3,100,120,140,180.123123,200,200
 T,HitSoundFinish,0,10
  M,3,100,120,140,180.123123,200,200
 T,HitSoundWhistle,0,10
  M,3,100,120,140,180.123123,200,200
 T,HitSoundDrumWhistle,0,10
  M,3,100,120,140,180.123123,200,200
 T,HitSoundSoft,0,10
  M,3,100,120,140,180.123123,200,200
 T,HitSoundAllSoft,0,10
  M,3,100,120,140,180.123123,200,200
 T,HitSoundDrumClap0,0,10
  M,3,100,120,140,180.123123,200,200
 T,HitSound6,0,10
  M,3,100,120,140,180.123123,200,200
 T,HitSoundPassing,0,10
  M,3,100,120,140,180.123123,200,200
 T,HitSoundFailing,0,10
  M,3,100,120,140,180.123123,200,200";
    let i: Events = i.parse().unwrap();

    let s = Events(vec![Event::Storyboard(Object {
        layer: Layer::Pass,
        origin: Origin::Centre,
        position: Position { x: 320, y: 240 },
        object_type: ObjectType::Sprite(
            Sprite::new(Path::new("\"Text\\Play2-HaveFunH.png\"")).unwrap(),
        ),
        commands: vec![
            Command {
                start_time: -28,
                properties: CommandProperties::Fade {
                    easing: Easing::from_repr(0).unwrap(),
                    end_time: -28,
                    start_opacity: dec!(1),
                    end_opacity: dec!(1),
                },
            },
            Command {
                start_time: 100,
                properties: CommandProperties::Move {
                    easing: Easing::from_repr(3).unwrap(),
                    end_time: 120,
                    start_x: dec!(140),
                    start_y: dec!(180.123123),
                    end_x: dec!(200),
                    end_y: dec!(200),
                },
            },
            Command {
                start_time: 100,
                properties: CommandProperties::MoveX {
                    easing: Easing::from_repr(3).unwrap(),
                    end_time: 120,
                    start_x: dec!(140),
                    end_x: dec!(180.123123),
                },
            },
            Command {
                start_time: 100,
                properties: CommandProperties::MoveY {
                    easing: Easing::from_repr(3).unwrap(),
                    end_time: 120,
                    start_y: dec!(140),
                    end_y: dec!(180.123123),
                },
            },
            Command {
                start_time: -28,
                properties: CommandProperties::Scale {
                    easing: Easing::from_repr(0).unwrap(),
                    end_time: -28,
                    start_scale: dec!(0.4),
                    end_scale: dec!(0.4),
                },
            },
            Command {
                start_time: 5000,
                properties: CommandProperties::VectorScale {
                    easing: Easing::from_repr(8).unwrap(),
                    end_time: 5500,
                    start_scale_x: dec!(0.5),
                    start_scale_y: dec!(2),
                    end_scale_x: dec!(2),
                    end_scale_y: dec!(0.5),
                },
            },
            Command {
                start_time: 5000,
                properties: CommandProperties::Rotate {
                    easing: Easing::from_repr(7).unwrap(),
                    end_time: 5500,
                    start_rotate: dec!(-0.785),
                    end_rotate: dec!(0.785),
                },
            },
            Command {
                start_time: 50000,
                properties: CommandProperties::Colour {
                    easing: Easing::from_repr(6).unwrap(),
                    end_time: 50001,
                    start_r: 0,
                    start_g: 0,
                    start_b: 0,
                    end_r: 255,
                    end_g: 255,
                    end_b: 255,
                },
            },
            Command {
                start_time: 300,
                properties: CommandProperties::Parameter {
                    easing: Easing::from_repr(5).unwrap(),
                    end_time: 350,
                    parameter: Parameter::ImageFlipHorizontal,
                },
            },
            Command {
                start_time: 300,
                properties: CommandProperties::Parameter {
                    easing: Easing::from_repr(5).unwrap(),
                    end_time: 350,
                    parameter: Parameter::ImageFlipVertical,
                },
            },
            Command {
                start_time: 300,
                properties: CommandProperties::Parameter {
                    easing: Easing::from_repr(5).unwrap(),
                    end_time: 350,
                    parameter: Parameter::UseAdditiveColourBlending,
                },
            },
            Command {
                start_time: 500,
                properties: CommandProperties::Loop {
                    loop_count: 10,
                    commands: vec![Command {
                        start_time: 10,
                        properties: CommandProperties::Loop {
                            loop_count: 10,
                            commands: vec![
                                Command {
                                    start_time: 100,
                                    properties: CommandProperties::Move {
                                        easing: Easing::from_repr(3).unwrap(),
                                        end_time: 120,
                                        start_x: dec!(140),
                                        start_y: dec!(180.123123),
                                        end_x: dec!(200),
                                        end_y: dec!(200),
                                    },
                                },
                                Command {
                                    start_time: -28,
                                    properties: CommandProperties::Scale {
                                        easing: Easing::from_repr(0).unwrap(),
                                        end_time: -28,
                                        start_scale: dec!(0.4),
                                        end_scale: dec!(0.4),
                                    },
                                },
                            ],
                        },
                    }],
                },
            },
            Command {
                start_time: 0,
                properties: CommandProperties::Trigger {
                    trigger_type: TriggerType::HitSound {
                        sample_set: None,
                        additions_sample_set: None,
                        addition: None,
                        custom_sample_set: None,
                    },
                    end_time: 10,
                    group_number: None,
                    commands: vec![Command {
                        start_time: 10,
                        properties: CommandProperties::Loop {
                            loop_count: 10,
                            commands: vec![Command {
                                start_time: 100,
                                properties: CommandProperties::Move {
                                    easing: Easing::from_repr(3).unwrap(),
                                    end_time: 120,
                                    start_x: dec!(140),
                                    start_y: dec!(180.123123),
                                    end_x: dec!(200),
                                    end_y: dec!(200),
                                },
                            }],
                        },
                    }],
                },
            },
            Command {
                start_time: 0,
                properties: CommandProperties::Trigger {
                    trigger_type: TriggerType::HitSound {
                        sample_set: None,
                        additions_sample_set: None,
                        addition: Some(Addition::Clap),
                        custom_sample_set: None,
                    },
                    end_time: 10,
                    group_number: None,
                    commands: vec![Command {
                        start_time: 100,
                        properties: CommandProperties::Move {
                            easing: Easing::from_repr(3).unwrap(),
                            end_time: 120,
                            start_x: dec!(140),
                            start_y: dec!(180.123123),
                            end_x: dec!(200),
                            end_y: dec!(200),
                        },
                    }],
                },
            },
            Command {
                start_time: 0,
                properties: CommandProperties::Trigger {
                    trigger_type: TriggerType::HitSound {
                        sample_set: None,
                        additions_sample_set: None,
                        addition: Some(Addition::Finish),
                        custom_sample_set: None,
                    },
                    end_time: 10,
                    group_number: None,
                    commands: vec![Command {
                        start_time: 100,
                        properties: CommandProperties::Move {
                            easing: Easing::from_repr(3).unwrap(),
                            end_time: 120,
                            start_x: dec!(140),
                            start_y: dec!(180.123123),
                            end_x: dec!(200),
                            end_y: dec!(200),
                        },
                    }],
                },
            },
            Command {
                start_time: 0,
                properties: CommandProperties::Trigger {
                    trigger_type: TriggerType::HitSound {
                        sample_set: None,
                        additions_sample_set: None,
                        addition: Some(Addition::Whistle),
                        custom_sample_set: None,
                    },
                    end_time: 10,
                    group_number: None,
                    commands: vec![Command {
                        start_time: 100,
                        properties: CommandProperties::Move {
                            easing: Easing::from_repr(3).unwrap(),
                            end_time: 120,
                            start_x: dec!(140),
                            start_y: dec!(180.123123),
                            end_x: dec!(200),
                            end_y: dec!(200),
                        },
                    }],
                },
            },
            Command {
                start_time: 0,
                properties: CommandProperties::Trigger {
                    trigger_type: TriggerType::HitSound {
                        sample_set: Some(storyboard::SampleSet::Drum),
                        additions_sample_set: None,
                        addition: Some(Addition::Whistle),
                        custom_sample_set: None,
                    },
                    end_time: 10,
                    group_number: None,
                    commands: vec![Command {
                        start_time: 100,
                        properties: CommandProperties::Move {
                            easing: Easing::from_repr(3).unwrap(),
                            end_time: 120,
                            start_x: dec!(140),
                            start_y: dec!(180.123123),
                            end_x: dec!(200),
                            end_y: dec!(200),
                        },
                    }],
                },
            },
            Command {
                start_time: 0,
                properties: CommandProperties::Trigger {
                    trigger_type: TriggerType::HitSound {
                        sample_set: Some(storyboard::SampleSet::Soft),
                        additions_sample_set: None,
                        addition: None,
                        custom_sample_set: None,
                    },
                    end_time: 10,
                    group_number: None,
                    commands: vec![Command {
                        start_time: 100,
                        properties: CommandProperties::Move {
                            easing: Easing::from_repr(3).unwrap(),
                            end_time: 120,
                            start_x: dec!(140),
                            start_y: dec!(180.123123),
                            end_x: dec!(200),
                            end_y: dec!(200),
                        },
                    }],
                },
            },
            Command {
                start_time: 0,
                properties: CommandProperties::Trigger {
                    trigger_type: TriggerType::HitSound {
                        sample_set: Some(storyboard::SampleSet::All),
                        additions_sample_set: Some(storyboard::SampleSet::Soft),
                        addition: None,
                        custom_sample_set: None,
                    },
                    end_time: 10,
                    group_number: None,
                    commands: vec![Command {
                        start_time: 100,
                        properties: CommandProperties::Move {
                            easing: Easing::from_repr(3).unwrap(),
                            end_time: 120,
                            start_x: dec!(140),
                            start_y: dec!(180.123123),
                            end_x: dec!(200),
                            end_y: dec!(200),
                        },
                    }],
                },
            },
            Command {
                start_time: 0,
                properties: CommandProperties::Trigger {
                    trigger_type: TriggerType::HitSound {
                        sample_set: Some(storyboard::SampleSet::Drum),
                        additions_sample_set: None,
                        addition: Some(Addition::Clap),
                        custom_sample_set: Some(0),
                    },
                    end_time: 10,
                    group_number: None,
                    commands: vec![Command {
                        start_time: 100,
                        properties: CommandProperties::Move {
                            easing: Easing::from_repr(3).unwrap(),
                            end_time: 120,
                            start_x: dec!(140),
                            start_y: dec!(180.123123),
                            end_x: dec!(200),
                            end_y: dec!(200),
                        },
                    }],
                },
            },
            Command {
                start_time: 0,
                properties: CommandProperties::Trigger {
                    trigger_type: TriggerType::HitSound {
                        sample_set: None,
                        additions_sample_set: None,
                        addition: None,
                        custom_sample_set: Some(6),
                    },
                    end_time: 10,
                    group_number: None,
                    commands: vec![Command {
                        start_time: 100,
                        properties: CommandProperties::Move {
                            easing: Easing::from_repr(3).unwrap(),
                            end_time: 120,
                            start_x: dec!(140),
                            start_y: dec!(180.123123),
                            end_x: dec!(200),
                            end_y: dec!(200),
                        },
                    }],
                },
            },
            Command {
                start_time: 0,
                properties: CommandProperties::Trigger {
                    trigger_type: TriggerType::Passing,
                    end_time: 10,
                    group_number: None,
                    commands: vec![Command {
                        start_time: 100,
                        properties: CommandProperties::Move {
                            easing: Easing::from_repr(3).unwrap(),
                            end_time: 120,
                            start_x: dec!(140),
                            start_y: dec!(180.123123),
                            end_x: dec!(200),
                            end_y: dec!(200),
                        },
                    }],
                },
            },
            Command {
                start_time: 0,
                properties: CommandProperties::Trigger {
                    trigger_type: TriggerType::Failing,
                    end_time: 10,
                    group_number: None,
                    commands: vec![Command {
                        start_time: 100,
                        properties: CommandProperties::Move {
                            easing: Easing::from_repr(3).unwrap(),
                            end_time: 120,
                            start_x: dec!(140),
                            start_y: dec!(180.123123),
                            end_x: dec!(200),
                            end_y: dec!(200),
                        },
                    }],
                },
            },
        ],
    })]);

    assert_eq!(i, s)
}

#[test]
fn storyboard_sprites_cmd_parse_back() {
    let i = "
Sprite,Pass,Centre,\"Text\\Play2-HaveFunH.png\",320,240
 F,0,-28,,1
 M,3,100,120,140,180.123123,200,200
_MX,3,100,120,140,180.123123
_MY,3,100,120,140,180.123123
 S,0,-28,,0.4
 V,8,5000,5500,0.5,2,2,0.5
 R,7,5000,5500,-0.785,0.785
 C,6,50000,50001,0,0,0,255,255,255
 P,5,300,350,H
 P,5,300,350,V
 P,5,300,350,A
 L,500,10
  L,10,10
   M,3,100,120,140,180.123123,200,200
___S,0,-28,,0.4
 T,HitSound,0,10
  L,10,10
   M,3,100,120,140,180.123123,200,200
 T,HitSoundClap,0,10
  M,3,100,120,140,180.123123,200,200
 T,HitSoundFinish,0,10
  M,3,100,120,140,180.123123,200,200
 T,HitSoundWhistle,0,10
  M,3,100,120,140,180.123123,200,200
 T,HitSoundDrumWhistle,0,10
  M,3,100,120,140,180.123123,200,200
 T,HitSoundSoft,0,10
  M,3,100,120,140,180.123123,200,200
 T,HitSoundAllSoft,0,10
  M,3,100,120,140,180.123123,200,200
 T,HitSoundDrumClap0,0,10
  M,3,100,120,140,180.123123,200,200
 T,HitSound6,0,10
  M,3,100,120,140,180.123123,200,200
 T,HitSoundPassing,0,10
  M,3,100,120,140,180.123123,200,200
 T,HitSoundFailing,0,10
  M,3,100,120,140,180.123123,200,200";
    let e: Events = i.parse().unwrap();

    assert_eq!(i, e.to_string())
}

#[test]
fn storyboard_sprites_parse() {
    let i = "Sprite,Pass,Centre,\"Text\\Play2-HaveFunH.png\",320,240
Animation,Fail,BottomCentre,\"Other\\Play3\\explosion.png\",418,108,12,31,LoopForever";
    let i: Events = i.parse().unwrap();

    let s = Events(vec![
        Event::Storyboard(Object {
            layer: Layer::Pass,
            origin: Origin::Centre,
            position: Position { x: 320, y: 240 },
            object_type: ObjectType::Sprite(
                Sprite::new(Path::new("\"Text\\Play2-HaveFunH.png\"")).unwrap(),
            ),
            commands: Vec::new(),
        }),
        Event::Storyboard(Object {
            layer: Layer::Fail,
            origin: Origin::BottomCentre,
            position: Position { x: 418, y: 108 },
            object_type: ObjectType::Animation(Animation::new(
                12,
                31,
                LoopType::LoopForever,
                Path::new("\"Other\\Play3\\explosion.png\""),
            )),
            commands: Vec::new(),
        }),
    ]);

    assert_eq!(i, s)
}

#[test]
fn storyboard_sprites_parse_back() {
    let i = "Sprite,Pass,Centre,\"Text\\Play2-HaveFunH.png\",320,240
Animation,Fail,BottomCentre,\"Other\\Play3\\explosion.png\",418,108,12,31,LoopForever";
    let e: Events = i.parse().unwrap();

    assert_eq!(i, e.to_string())
}

// TODO make all fields optional
#[test]
fn osu_file_parse() {
    let i = "osu file format v14

[General]
AudioFilename: audio.mp3
AudioLeadIn: 0
PreviewTime: 48349
Countdown: 0
SampleSet: Soft
StackLeniency: 0.2
Mode: 3
LetterboxInBreaks: 0
SpecialStyle: 0
WidescreenStoryboard: 0

[Editor]
Bookmarks: 11018,21683,32349,37683,48349,59016,69683,80349,91016
DistanceSpacing: 0.8
BeatDivisor: 12
GridSize: 8
TimelineZoom: 2

[Metadata]
Title:LOVE IS ORANGE
TitleUnicode:LOVE IS ORANGE
Artist:Orange Lounge
ArtistUnicode:Orange Lounge
Creator:Xnery
Version:Bittersweet Love
Source:beatmania IIDX 8th style
Tags:famoss 舟木智介 tomosuke funaki 徳井志津江 shizue tokui ddr dancedancerevolution
BeatmapID:3072232
BeatmapSetID:1499093

[Difficulty]
HPDrainRate:8
CircleSize:5
OverallDifficulty:8
ApproachRate:5
SliderMultiplier:1.4
SliderTickRate:1

[Events]
//Background and Video events
0,0,\"bg.jpg\",0,0

[TimingPoints]
350,333.333333333333,4,2,1,60,1,0


[HitObjects]
256,192,8016,1,0,0:0:0:0:
153,192,8183,1,2,0:0:0:0:";

    let i: OsuFile = i.parse().unwrap();

    let osu_file = OsuFile {
        version: 14,
        general: Some(General {
            audio_filename: Some("audio.mp3".to_string()),
            audio_lead_in: Some(0),
            preview_time: Some(48349),
            countdown: Some(CountdownSpeed::from_repr(0).unwrap()),
            sample_set: Some(SampleSet::Soft),
            stack_leniency: Some(dec!(0.2)),
            mode: Some(GameMode::Mania),
            letterbox_in_breaks: Some(false),
            special_style: Some(false),
            widescreen_storyboard: Some(false),
            ..General::empty()
        }),
        editor: Some(Editor {
            bookmarks: vec![
                11018, 21683, 32349, 37683, 48349, 59016, 69683, 80349, 91016,
            ],
            distance_spacing: dec!(0.8),
            beat_divisor: dec!(12),
            grid_size: 8,
            timeline_zoom: dec!(2),
        }),
        metadata: Some(Metadata {
            title: "LOVE IS ORANGE".to_string(),
            title_unicode: "LOVE IS ORANGE".to_string(),
            artist: "Orange Lounge".to_string(),
            artist_unicode: "Orange Lounge".to_string(),
            creator: "Xnery".to_string(),
            version: "Bittersweet Love".to_string(),
            source: "beatmania IIDX 8th style".to_string(),
            tags: vec![
                "famoss".to_string(),
                "舟木智介".to_string(),
                "tomosuke".to_string(),
                "funaki".to_string(),
                "徳井志津江".to_string(),
                "shizue".to_string(),
                "tokui".to_string(),
                "ddr".to_string(),
                "dancedancerevolution".to_string(),
            ],
            beatmap_id: 3072232,
            beatmap_set_id: 1499093,
        }),
        difficulty: Some(Difficulty {
            hp_drain_rate: dec!(8),
            circle_size: dec!(5),
            overall_difficulty: dec!(8),
            approach_rate: dec!(5),
            slider_multiplier: dec!(1.4),
            slider_tickrate: dec!(1),
        }),
        events: Some(Events(vec![
            Event::Comment("Background and Video events".to_string()),
            Event::NormalEvent {
                start_time: 0,
                event_params: EventParams::Background(Background {
                    filename: PathBuf::from("\"bg.jpg\""),
                    position: Position { x: 0, y: 0 },
                }),
            },
        ])),
        timing_points: Some(TimingPoints(vec![TimingPoint::new_uninherited(
            350,
            dec!(333.333333333333),
            4,
            timingpoint::SampleSet::Soft,
            SampleIndex::Index(NonZeroUsize::new(1).unwrap()),
            Volume::new(60).unwrap(),
            Effects {
                kiai_time_enabled: false,
                no_first_barline_in_taiko_mania: false,
            },
        )])),
        colours: None,
        hitobjects: Some(HitObjects(vec![
            HitObjectWrapper::HitCircle(HitCircle::new(
                Position { x: 256, y: 192 },
                8016,
                HitSound::new(false, false, false, false),
                HitSample::new(
                    hitobject::types::SampleSet::NoCustomSampleSet,
                    hitobject::types::SampleSet::NoCustomSampleSet,
                    None,
                    hitobject::types::Volume::new(None).unwrap(),
                    "".to_string(),
                ),
                false,
                0,
            )),
            HitObjectWrapper::HitCircle(HitCircle::new(
                Position { x: 153, y: 192 },
                8183,
                HitSound::new(false, true, false, false),
                HitSample::new(
                    hitobject::types::SampleSet::NoCustomSampleSet,
                    hitobject::types::SampleSet::NoCustomSampleSet,
                    None,
                    hitobject::types::Volume::new(None).unwrap(),
                    "".to_string(),
                ),
                false,
                0,
            )),
        ])),
    };

    assert_eq!(i, osu_file);
}

#[test]
fn osu_file_parse_back() {
    let i = "osu file format v14

[General]
AudioFilename: audio.mp3
AudioLeadIn: 0
PreviewTime: 48349
Countdown: 0
SampleSet: Soft
StackLeniency: 0.2
Mode: 3
LetterboxInBreaks: 0
SpecialStyle: 0
WidescreenStoryboard: 0

[Editor]
Bookmarks: 11018,21683,32349,37683,48349,59016,69683,80349,91016
DistanceSpacing: 0.8
BeatDivisor: 12
GridSize: 8
TimelineZoom: 2

[Metadata]
Title:LOVE IS ORANGE
TitleUnicode:LOVE IS ORANGE
Artist:Orange Lounge
ArtistUnicode:Orange Lounge
Creator:Xnery
Version:Bittersweet Love
Source:beatmania IIDX 8th style
Tags:famoss 舟木智介 tomosuke funaki 徳井志津江 shizue tokui ddr dancedancerevolution
BeatmapID:3072232
BeatmapSetID:1499093

[Difficulty]
HPDrainRate:8
CircleSize:5
OverallDifficulty:8
ApproachRate:5
SliderMultiplier:1.4
SliderTickRate:1

[Events]
//Background and Video events
0,0,\"bg.jpg\",0,0

[TimingPoints]
350,333.333333333333,4,2,1,60,1,0

[HitObjects]
256,192,8016,1,0,0:0:0:0:
153,192,8183,1,2,0:0:0:0:";

    let o: OsuFile = i.parse().unwrap();

    assert_eq!(i, o.to_string());
}

#[test]
fn osu_file_smallest_parse() {
    let i = "osu file format v14";

    let o: OsuFile = i.parse().unwrap();

    assert_eq!(i, o.to_string());
}
