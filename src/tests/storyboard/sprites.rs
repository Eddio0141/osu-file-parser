use std::path::{Path, PathBuf};

use either::Either;
use rust_decimal_macros::dec;

use crate::osu_file::events::storyboard::sprites::*;
use crate::osu_file::events::Event;
use crate::osu_file::types::Position;
use crate::osu_file::{Events, VersionedFromStr, VersionedToString};

#[test]
fn storyboard_sprites_parse() {
    let i_str = "Sprite,Pass,Centre,\"Text\\Play2-HaveFunH.png\",320,240
Animation,Fail,BottomCentre,\"Other\\Play3\\explosion.png\",418,108,12,31,LoopForever";
    let i = Events::from_str(i_str, 14).unwrap().unwrap();

    let s = Events(vec![
        Event::StoryboardObject(Object {
            layer: Layer::Pass,
            origin: Origin {
                type_: Either::Left(OriginType::Centre),
                shorthand: false,
            },
            position: Position {
                x: dec!(320).into(),
                y: dec!(240).into(),
            },
            object_type: ObjectType::Sprite(
                Sprite::new(Path::new("\"Text\\Play2-HaveFunH.png\"")).unwrap(),
            ),
            commands: Vec::new(),
        }),
        Event::StoryboardObject(Object {
            layer: Layer::Fail,
            origin: Origin {
                type_: Either::Left(OriginType::BottomCentre),
                shorthand: false,
            },
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
            commands: Vec::new(),
        }),
    ]);

    assert_eq!(i, s);
    assert_eq!(i_str, i.to_string(14).unwrap());
}

#[test]
fn frame_file_names() {
    let animation = Object {
        layer: Layer::Background,
        origin: Origin {
            type_: Either::Left(OriginType::BottomCentre),
            shorthand: false,
        },
        position: Position {
            x: dec!(0).into(),
            y: dec!(0).into(),
        },
        object_type: ObjectType::Animation(Animation {
            frame_count: 4,
            frame_delay: rust_decimal::Decimal::ZERO,
            loop_type: LoopType::LoopForever,
            filepath: "testfile.png".into(),
        }),
        commands: Vec::new(),
    };

    if let ObjectType::Animation(animation) = &animation.object_type {
        let file_names = animation.frame_file_names();

        assert_eq!(
            file_names,
            vec![
                PathBuf::from("testfile0.png"),
                "testfile1.png".into(),
                "testfile2.png".into(),
                "testfile3.png".into(),
            ]
        );
    } else {
        unreachable!();
    }
}
