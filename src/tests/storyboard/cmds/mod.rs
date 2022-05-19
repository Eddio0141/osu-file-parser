mod error;

use std::path::{Path, PathBuf};

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::osu_file::Version;
use crate::osu_file::events::storyboard::cmds::*;
use crate::osu_file::events::storyboard::sprites::*;
use crate::osu_file::events::storyboard::types::*;
use crate::osu_file::events::*;
use crate::osu_file::types::Position;

#[test]
fn storyboard_sprites_cmd_parse() {
    let i_str = "Sprite,Pass,Centre,\"Text\\Play2-HaveFunH.png\",320,240
 F,0,-28,,1
 M,3,100,120,140,180.123123,200,200
 MX,3,100,120,140,180.123123
 MY,3,100,120,140,180.123123
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
   S,0,-28,,0.4
 T,HitSound,0,10
  L,10,10
   M,3,100,120,140,180.123123,200,200
Animation,Fail,BottomCentre,\"Other\\Play3\\explosion.png\",418,108,12,31,LoopForever
 T,HitSoundClap,0,10
 T,HitSoundFinish,0,10
 T,HitSoundWhistle,0,10
 T,HitSoundDrumWhistle,0,10
 T,HitSoundSoft,0,10
 T,HitSoundAllSoft,0,10
 T,HitSoundDrumClap0,0,10
 T,HitSound6,0,10
 T,HitSoundPassing,0,10
 T,HitSoundFailing,0,10";
    let i: Events = Events::from_str_v14(i_str).unwrap().unwrap();

    let s = Events(vec![
        Event::Storyboard(Object {
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
                        end_time: None,
                        start_opacity: Decimal::ONE,
                        continuing_opacities: Vec::new(),
                    },
                },
                Command {
                    start_time: 100,
                    properties: CommandProperties::Move {
                        easing: Easing::from_repr(3).unwrap(),
                        end_time: Some(120),
                        positions_xy: ContinuingFields::new(
                            (dec!(140), dec!(180.123123)),
                            vec![(dec!(200), Some(dec!(200)))],
                        )
                        .unwrap(),
                    },
                },
                Command {
                    start_time: 100,
                    properties: CommandProperties::MoveX {
                        easing: Easing::from_repr(3).unwrap(),
                        end_time: Some(120),
                        start_x: dec!(140),
                        continuing_x: vec![dec!(180.123123)],
                    },
                },
                Command {
                    start_time: 100,
                    properties: CommandProperties::MoveY {
                        easing: Easing::from_repr(3).unwrap(),
                        end_time: Some(120),
                        start_y: dec!(140),
                        continuing_y: vec![dec!(180.123123)],
                    },
                },
                Command {
                    start_time: -28,
                    properties: CommandProperties::Scale {
                        easing: Easing::from_repr(0).unwrap(),
                        end_time: None,
                        start_scale: dec!(0.4),
                        continuing_scales: Vec::new(),
                    },
                },
                Command {
                    start_time: 5000,
                    properties: CommandProperties::VectorScale {
                        easing: Easing::from_repr(8).unwrap(),
                        end_time: Some(5500),
                        scales_xy: ContinuingFields::new(
                            (dec!(0.5), dec!(2)),
                            vec![(dec!(2), Some(dec!(0.5)))],
                        )
                        .unwrap(),
                    },
                },
                Command {
                    start_time: 5000,
                    properties: CommandProperties::Rotate {
                        easing: Easing::from_repr(7).unwrap(),
                        end_time: Some(5500),
                        start_rotation: dec!(-0.785),
                        continuing_rotations: vec![dec!(0.785)],
                    },
                },
                Command {
                    start_time: 50000,
                    properties: CommandProperties::Colour {
                        easing: Easing::from_repr(6).unwrap(),
                        end_time: Some(50001),
                        colours: Colours::new((0, 0, 0), vec![(255, Some(255), Some(255))])
                            .unwrap(),
                    },
                },
                Command {
                    start_time: 300,
                    properties: CommandProperties::Parameter {
                        easing: Easing::from_repr(5).unwrap(),
                        end_time: Some(350),
                        parameter: Parameter::ImageFlipHorizontal,
                        continuing_parameters: Vec::new(),
                    },
                },
                Command {
                    start_time: 300,
                    properties: CommandProperties::Parameter {
                        easing: Easing::from_repr(5).unwrap(),
                        end_time: Some(350),
                        parameter: Parameter::ImageFlipVertical,
                        continuing_parameters: Vec::new(),
                    },
                },
                Command {
                    start_time: 300,
                    properties: CommandProperties::Parameter {
                        easing: Easing::from_repr(5).unwrap(),
                        end_time: Some(350),
                        parameter: Parameter::UseAdditiveColourBlending,
                        continuing_parameters: Vec::new(),
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
                                            end_time: Some(120),
                                            positions_xy: ContinuingFields::new(
                                                (dec!(140), dec!(180.123123)),
                                                vec![(dec!(200), Some(dec!(200)))],
                                            )
                                            .unwrap(),
                                        },
                                    },
                                    Command {
                                        start_time: -28,
                                        properties: CommandProperties::Scale {
                                            easing: Easing::from_repr(0).unwrap(),
                                            end_time: None,
                                            start_scale: dec!(0.4),
                                            continuing_scales: Vec::new(),
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
                        end_time: Some(10),
                        group_number: None,
                        commands: vec![Command {
                            start_time: 10,
                            properties: CommandProperties::Loop {
                                loop_count: 10,
                                commands: vec![Command {
                                    start_time: 100,
                                    properties: CommandProperties::Move {
                                        easing: Easing::from_repr(3).unwrap(),
                                        end_time: Some(120),
                                        positions_xy: ContinuingFields::new(
                                            (dec!(140), dec!(180.123123)),
                                            vec![(dec!(200), Some(dec!(200)))],
                                        )
                                        .unwrap(),
                                    },
                                }],
                            },
                        }],
                    },
                },
            ],
        }),
        Event::Storyboard(Object {
            layer: Layer::Fail,
            origin: Origin::BottomCentre,
            position: Position { x: 418, y: 108 },
            object_type: ObjectType::Animation(Animation {
                frame_count: 12,
                frame_delay: 31,
                loop_type: LoopType::LoopForever,
                filepath: PathBuf::from("\"Other\\Play3\\explosion.png\""),
            }),
            commands: vec![
                Command {
                    start_time: 0,
                    properties: CommandProperties::Trigger {
                        trigger_type: TriggerType::HitSound {
                            sample_set: None,
                            additions_sample_set: None,
                            addition: Some(Addition::Clap),
                            custom_sample_set: None,
                        },
                        end_time: Some(10),
                        group_number: None,
                        commands: Vec::new(),
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
                        end_time: Some(10),
                        group_number: None,
                        commands: Vec::new(),
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
                        end_time: Some(10),
                        group_number: None,
                        commands: Vec::new(),
                    },
                },
                Command {
                    start_time: 0,
                    properties: CommandProperties::Trigger {
                        trigger_type: TriggerType::HitSound {
                            sample_set: Some(SampleSet::Drum),
                            additions_sample_set: None,
                            addition: Some(Addition::Whistle),
                            custom_sample_set: None,
                        },
                        end_time: Some(10),
                        group_number: None,
                        commands: Vec::new(),
                    },
                },
                Command {
                    start_time: 0,
                    properties: CommandProperties::Trigger {
                        trigger_type: TriggerType::HitSound {
                            sample_set: Some(SampleSet::Soft),
                            additions_sample_set: None,
                            addition: None,
                            custom_sample_set: None,
                        },
                        end_time: Some(10),
                        group_number: None,
                        commands: Vec::new(),
                    },
                },
                Command {
                    start_time: 0,
                    properties: CommandProperties::Trigger {
                        trigger_type: TriggerType::HitSound {
                            sample_set: Some(SampleSet::All),
                            additions_sample_set: Some(SampleSet::Soft),
                            addition: None,
                            custom_sample_set: None,
                        },
                        end_time: Some(10),
                        group_number: None,
                        commands: Vec::new(),
                    },
                },
                Command {
                    start_time: 0,
                    properties: CommandProperties::Trigger {
                        trigger_type: TriggerType::HitSound {
                            sample_set: Some(SampleSet::Drum),
                            additions_sample_set: None,
                            addition: Some(Addition::Clap),
                            custom_sample_set: Some(0),
                        },
                        end_time: Some(10),
                        group_number: None,
                        commands: Vec::new(),
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
                        end_time: Some(10),
                        group_number: None,
                        commands: Vec::new(),
                    },
                },
                Command {
                    start_time: 0,
                    properties: CommandProperties::Trigger {
                        trigger_type: TriggerType::Passing,
                        end_time: Some(10),
                        group_number: None,
                        commands: Vec::new(),
                    },
                },
                Command {
                    start_time: 0,
                    properties: CommandProperties::Trigger {
                        trigger_type: TriggerType::Failing,
                        end_time: Some(10),
                        group_number: None,
                        commands: Vec::new(),
                    },
                },
            ],
        }),
    ]);

    assert_eq!(i, s);
    assert_eq!(i_str, i.to_string_v14());
}

#[test]
fn colours() {
    let i = "C,0,0,0,255,255,255,255,255,255,0";
    let i = i.parse::<Command>().unwrap();

    let cmd = Command {
        start_time: 0,
        properties: CommandProperties::Colour {
            easing: Easing::from_repr(0).unwrap(),
            end_time: Some(0),
            colours: Colours {
                start: (255, 255, 255),
                continuing: vec![(255, Some(255), Some(255)), (0, None, None)],
            },
        },
    };

    assert_eq!(i, cmd);
}

#[test]
fn parameters() {
    let i = "P,0,0,,H,V,A";
    let i = i.parse::<Command>().unwrap();

    let cmd = Command {
        start_time: 0,
        properties: CommandProperties::Parameter {
            easing: Easing::from_repr(0).unwrap(),
            end_time: None,
            parameter: Parameter::ImageFlipHorizontal,
            continuing_parameters: vec![
                Parameter::ImageFlipVertical,
                Parameter::UseAdditiveColourBlending,
            ],
        },
    };

    assert_eq!(i, cmd);
}

#[test]
fn trigger_group_number() {
    let i = "T,HitSound,0,0,5";
    let i = i.parse::<Command>().unwrap();

    let cmd = Command {
        start_time: 0,
        properties: CommandProperties::Trigger {
            trigger_type: TriggerType::HitSound {
                sample_set: None,
                additions_sample_set: None,
                addition: None,
                custom_sample_set: None,
            },
            end_time: Some(0),
            group_number: Some(5),
            commands: Vec::new(),
        },
    };

    assert_eq!(i, cmd);
}

#[test]
fn move_command() {
    let i = "M,0,0,0,-5,10,55";
    let i = i.parse::<Command>().unwrap();

    let cmd = Command {
        start_time: 0,
        properties: CommandProperties::Move {
            easing: Easing::from_repr(0).unwrap(),
            end_time: Some(0),
            positions_xy: ContinuingFields::new((dec!(-5), dec!(10)), vec![(dec!(55), None)])
                .unwrap(),
        },
    };

    assert_eq!(i, cmd);
}

#[test]
fn fade_chain() {
    let i = "F,0,0,0,1,0,0.5,0,0.25,0";
    let i = i.parse::<Command>().unwrap();

    let cmd = Command {
        start_time: 0,
        properties: CommandProperties::Fade {
            easing: Easing::from_repr(0).unwrap(),
            end_time: Some(0),
            start_opacity: Decimal::ONE,
            continuing_opacities: vec![
                Decimal::ZERO,
                dec!(0.5),
                Decimal::ZERO,
                dec!(0.25),
                Decimal::ZERO,
            ],
        },
    };

    assert_eq!(i, cmd);
}
