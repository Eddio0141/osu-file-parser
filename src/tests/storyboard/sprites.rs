use std::path::Path;

use crate::osu_file::events::storyboard::sprites::*;
use crate::osu_file::events::*;
use crate::osu_file::types::Position;

#[test]
fn storyboard_sprites_parse() {
    let i_str = "Sprite,Pass,Centre,\"Text\\Play2-HaveFunH.png\",320,240
Animation,Fail,BottomCentre,\"Other\\Play3\\explosion.png\",418,108,12,31,LoopForever";
    let i: Events = i_str.parse().unwrap();

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

    assert_eq!(i, s);
    assert_eq!(i_str, i.to_string());
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
