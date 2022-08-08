mod error;

use std::path::Path;

use rust_decimal_macros::dec;

use crate::osu_file::events::storyboard::cmds::*;
use crate::osu_file::events::storyboard::sprites::*;
use crate::osu_file::events::storyboard::types::*;
use crate::osu_file::events::Event;
use crate::osu_file::types::Position;
use crate::osu_file::Events;
use crate::osu_file::VersionedFromRepr;
use crate::osu_file::{VersionedFromStr, VersionedToString};

use pretty_assertions::assert_eq;

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
 T,Passing,0,10
 T,Failing,0,10";
    let i: Events = Events::from_str(i_str, 14).unwrap().unwrap();

    let s = Events(vec![
        Event::StoryboardObject(Object {
            layer: Layer::Pass,
            origin: Origin::Centre,
            position: Position {
                x: dec!(320).into(),
                y: dec!(240).into(),
            },
            object_type: ObjectType::Sprite(
                Sprite::new(Path::new("\"Text\\Play2-HaveFunH.png\"")).unwrap(),
            ),
            commands: vec![
                Command {
                    start_time: Some(-28),
                    properties: CommandProperties::Fade {
                        easing: Easing::from_repr(0, 14).unwrap().unwrap(),
                        end_time: None,
                        start_opacity: rust_decimal::Decimal::ONE.into(),
                        continuing_opacities: Vec::new(),
                    },
                },
                Command {
                    start_time: Some(100),
                    properties: CommandProperties::Move {
                        easing: Easing::from_repr(3, 14).unwrap().unwrap(),
                        end_time: Some(120),
                        positions_xy: ContinuingFields::new(
                            (dec!(140).into(), dec!(180.123123).into()),
                            vec![(dec!(200).into(), Some(dec!(200).into()))],
                        )
                        .unwrap(),
                    },
                },
                Command {
                    start_time: Some(100),
                    properties: CommandProperties::MoveX {
                        easing: Easing::from_repr(3, 14).unwrap().unwrap(),
                        end_time: Some(120),
                        start_x: dec!(140).into(),
                        continuing_x: vec![dec!(180.123123).into()],
                    },
                },
                Command {
                    start_time: Some(100),
                    properties: CommandProperties::MoveY {
                        easing: Easing::from_repr(3, 14).unwrap().unwrap(),
                        end_time: Some(120),
                        start_y: dec!(140).into(),
                        continuing_y: vec![dec!(180.123123).into()],
                    },
                },
                Command {
                    start_time: Some(-28),
                    properties: CommandProperties::Scale {
                        easing: Easing::from_repr(0, 14).unwrap().unwrap(),
                        end_time: None,
                        start_scale: dec!(0.4).into(),
                        continuing_scales: Vec::new(),
                    },
                },
                Command {
                    start_time: Some(5000),
                    properties: CommandProperties::VectorScale {
                        easing: Easing::from_repr(8, 14).unwrap().unwrap(),
                        end_time: Some(5500),
                        scales_xy: ContinuingFields::new(
                            (dec!(0.5).into(), dec!(2).into()),
                            vec![(dec!(2).into(), Some(dec!(0.5).into()))],
                        )
                        .unwrap(),
                    },
                },
                Command {
                    start_time: Some(5000),
                    properties: CommandProperties::Rotate {
                        easing: Easing::from_repr(7, 14).unwrap().unwrap(),
                        end_time: Some(5500),
                        start_rotation: dec!(-0.785).into(),
                        continuing_rotations: vec![dec!(0.785).into()],
                    },
                },
                Command {
                    start_time: Some(50000),
                    properties: CommandProperties::Colour {
                        easing: Easing::from_repr(6, 14).unwrap().unwrap(),
                        end_time: Some(50001),
                        colours: Colours::new((0, 0, 0), vec![(255, Some(255), Some(255))])
                            .unwrap(),
                    },
                },
                Command {
                    start_time: Some(300),
                    properties: CommandProperties::Parameter {
                        easing: Easing::from_repr(5, 14).unwrap().unwrap(),
                        end_time: Some(350),
                        parameter: Parameter::ImageFlipHorizontal,
                        continuing_parameters: Vec::new(),
                    },
                },
                Command {
                    start_time: Some(300),
                    properties: CommandProperties::Parameter {
                        easing: Easing::from_repr(5, 14).unwrap().unwrap(),
                        end_time: Some(350),
                        parameter: Parameter::ImageFlipVertical,
                        continuing_parameters: Vec::new(),
                    },
                },
                Command {
                    start_time: Some(300),
                    properties: CommandProperties::Parameter {
                        easing: Easing::from_repr(5, 14).unwrap().unwrap(),
                        end_time: Some(350),
                        parameter: Parameter::UseAdditiveColourBlending,
                        continuing_parameters: Vec::new(),
                    },
                },
                Command {
                    start_time: Some(500),
                    properties: CommandProperties::Loop {
                        loop_count: 10,
                        commands: vec![Command {
                            start_time: Some(10),
                            properties: CommandProperties::Loop {
                                loop_count: 10,
                                commands: vec![
                                    Command {
                                        start_time: Some(100),
                                        properties: CommandProperties::Move {
                                            easing: Easing::from_repr(3, 14).unwrap().unwrap(),
                                            end_time: Some(120),
                                            positions_xy: ContinuingFields::new(
                                                (dec!(140).into(), dec!(180.123123).into()),
                                                vec![(dec!(200).into(), Some(dec!(200).into()))],
                                            )
                                            .unwrap(),
                                        },
                                    },
                                    Command {
                                        start_time: Some(-28),
                                        properties: CommandProperties::Scale {
                                            easing: Easing::from_repr(0, 14).unwrap().unwrap(),
                                            end_time: None,
                                            start_scale: dec!(0.4).into(),
                                            continuing_scales: Vec::new(),
                                        },
                                    },
                                ],
                            },
                        }],
                    },
                },
                Command {
                    start_time: Some(0),
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
                            start_time: Some(10),
                            properties: CommandProperties::Loop {
                                loop_count: 10,
                                commands: vec![Command {
                                    start_time: Some(100),
                                    properties: CommandProperties::Move {
                                        easing: Easing::from_repr(3, 14).unwrap().unwrap(),
                                        end_time: Some(120),
                                        positions_xy: ContinuingFields::new(
                                            (dec!(140).into(), dec!(180.123123).into()),
                                            vec![(dec!(200).into(), Some(dec!(200).into()))],
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
        Event::StoryboardObject(Object {
            layer: Layer::Fail,
            origin: Origin::BottomCentre,
            position: Position {
                x: dec!(418).into(),
                y: dec!(108).into(),
            },
            object_type: ObjectType::Animation(Animation {
                frame_count: 12,
                frame_delay: dec!(31),
                loop_type: LoopType::LoopForever,
                filepath: "\"Other\\Play3\\explosion.png\"".into(),
            }),
            commands: vec![
                Command {
                    start_time: Some(0),
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
                    start_time: Some(0),
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
                    start_time: Some(0),
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
                    start_time: Some(0),
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
                    start_time: Some(0),
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
                    start_time: Some(0),
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
                    start_time: Some(0),
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
                    start_time: Some(0),
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
                    start_time: Some(0),
                    properties: CommandProperties::Trigger {
                        trigger_type: TriggerType::Passing,
                        end_time: Some(10),
                        group_number: None,
                        commands: Vec::new(),
                    },
                },
                Command {
                    start_time: Some(0),
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
    assert_eq!(i_str, i.to_string(14).unwrap());
}

#[test]
fn colours() {
    let i = "C,0,0,0,255,255,255,255,255,255,0";
    let i = Command::from_str(i, 14).unwrap().unwrap();

    let cmd = Command {
        start_time: Some(0),
        properties: CommandProperties::Colour {
            easing: Easing::from_repr(0, 14).unwrap().unwrap(),
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
    let i = Command::from_str(i, 14).unwrap().unwrap();

    let cmd = Command {
        start_time: Some(0),
        properties: CommandProperties::Parameter {
            easing: Easing::from_repr(0, 14).unwrap().unwrap(),
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
fn trigger() {
    // we test the 4 possibilities
    // has group number, has end time
    // has group number, no end time
    // no group number, has end time
    // no group number, no end time
    let everything_str = "T,HitSound,0,1,2";
    let everything = Command::from_str(everything_str, 14).unwrap().unwrap();
    let group_str = "T,HitSound,0,,1";
    let group = Command::from_str(group_str, 14).unwrap().unwrap();
    let end_time_str = "T,HitSound,0,1";
    let end_time = Command::from_str(end_time_str, 14).unwrap().unwrap();
    let nothing_str = "T,HitSound,0";
    let nothing = Command::from_str(nothing_str, 14).unwrap().unwrap();

    assert_eq!(everything_str, everything.to_string(14).unwrap());
    assert_eq!(group_str, group.to_string(14).unwrap());
    assert_eq!(end_time_str, end_time.to_string(14).unwrap());
    assert_eq!(nothing_str, nothing.to_string(14).unwrap());
}

#[test]
fn trigger_group_number() {
    let i = "T,HitSound,0,0,5";
    let i = Command::from_str(i, 14).unwrap().unwrap();

    let cmd = Command {
        start_time: Some(0),
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
    let i = Command::from_str(i, 14).unwrap().unwrap();

    let cmd = Command {
        start_time: Some(0),
        properties: CommandProperties::Move {
            easing: Easing::from_repr(0, 14).unwrap().unwrap(),
            end_time: Some(0),
            positions_xy: ContinuingFields::new(
                (dec!(-5).into(), dec!(10).into()),
                vec![(dec!(55).into(), None)],
            )
            .unwrap(),
        },
    };

    assert_eq!(i, cmd);
}

#[test]
fn fade_chain() {
    let i = "F,0,0,0,1,0,0.5,0,0.25,0";
    let i = Command::from_str(i, 14).unwrap().unwrap();

    let cmd = Command {
        start_time: Some(0),
        properties: CommandProperties::Fade {
            easing: Easing::from_repr(0, 14).unwrap().unwrap(),
            end_time: Some(0),
            start_opacity: rust_decimal::Decimal::ONE.into(),
            continuing_opacities: vec![
                rust_decimal::Decimal::ZERO.into(),
                dec!(0.5).into(),
                rust_decimal::Decimal::ZERO.into(),
                dec!(0.25).into(),
                rust_decimal::Decimal::ZERO.into(),
            ],
        },
    };

    assert_eq!(i, cmd);
}
