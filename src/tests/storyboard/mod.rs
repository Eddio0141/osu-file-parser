use crate::{
    events::{
        types::{LayerLegacy, OriginTypeLegacy},
        AnimationLegacy, SampleLegacy, SpriteLegacy, Volume,
    },
    Position, VersionedFromStr, VersionedToString,
};

mod cmds;
mod sprites;

#[test]
fn sprite_legacy_parse() {
    let i = "4,0,1,\"Text\\Play2-HaveFunH.png\",320,240";
    let o = <SpriteLegacy as VersionedFromStr>::from_str(i, 3)
        .unwrap()
        .unwrap();

    let s = SpriteLegacy {
        layer: LayerLegacy::Background,
        origin: OriginTypeLegacy::Centre,
        file_name: "\"Text\\Play2-HaveFunH.png\"".into(),
        position: Some(Position {
            x: 320.into(),
            y: 240.into(),
        }),
        commands: Vec::new(),
    };

    assert_eq!(o, s);
    assert_eq!(
        i,
        <SpriteLegacy as VersionedToString>::to_string(&o, 3).unwrap()
    );
}

#[test]
fn animation_legacy() {
    let i = "5,0,1,\"Other\\Play3\\explosion.png\",418,108";
    let o = <AnimationLegacy as VersionedFromStr>::from_str(i, 3)
        .unwrap()
        .unwrap();

    let s = AnimationLegacy {
        layer: LayerLegacy::Background,
        origin: OriginTypeLegacy::Centre,
        file_name: "\"Other\\Play3\\explosion.png\"".into(),
        position: Some(Position {
            x: 418.into(),
            y: 108.into(),
        }),
        commands: Vec::new(),
    };

    assert_eq!(o, s);
    assert_eq!(
        i,
        <AnimationLegacy as VersionedToString>::to_string(&o, 3).unwrap()
    );
}

#[test]
fn sample_legacy() {
    let i = "6,55,0,\"Text\\Play2-HaveFunH.png\",60";
    let o = <SampleLegacy as VersionedFromStr>::from_str(i, 3)
        .unwrap()
        .unwrap();

    let s = SampleLegacy {
        layer: LayerLegacy::Background,
        file_name: "\"Text\\Play2-HaveFunH.png\"".into(),
        time: 55.into(),
        volume: Some(Volume::new(60, 3).unwrap()),
        commands: Vec::new(),
    };

    let i_without_volume = "6,55,0,\"Text\\Play2-HaveFunH.png\"";
    let o_without_volume = <SampleLegacy as VersionedFromStr>::from_str(i_without_volume, 3)
        .unwrap()
        .unwrap();
    let s_without_volume = SampleLegacy {
        layer: LayerLegacy::Background,
        file_name: "\"Text\\Play2-HaveFunH.png\"".into(),
        time: 55.into(),
        volume: None,
        commands: Vec::new(),
    };

    assert_eq!(o, s);
    assert_eq!(
        i,
        <SampleLegacy as VersionedToString>::to_string(&o, 3).unwrap()
    );

    assert_eq!(o_without_volume, s_without_volume);
    assert_eq!(
        i_without_volume,
        <SampleLegacy as VersionedToString>::to_string(&o_without_volume, 3).unwrap()
    );
}
