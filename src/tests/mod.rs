mod error_line_index;
mod hitobjects;
mod osu_files;
mod parsers;
mod storyboard;

use crate::osu_file::types::Decimal;
use pretty_assertions::assert_eq;
use std::path::{Path, PathBuf};

use rust_decimal_macros::dec;

use crate::osu_file::{
    colours::{Colour, Colours, Rgb},
    difficulty::Difficulty,
    editor::{self, Editor},
    events::{Background, Break, Event, Events},
    general::{Countdown, General, Mode, OverlayPosition, SampleSet},
    metadata::Metadata,
    timingpoints,
    timingpoints::{Effects, SampleIndex, TimingPoint, TimingPoints, Volume},
    types::Position,
    VersionedFromStr, VersionedToString,
};

#[test]
fn general_parse_v14() {
    let i_str = "AudioFilename: test.mp3
AudioLeadIn: 555
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
    let i = General::from_str(i_str, 14).unwrap().unwrap();

    let g = General {
        audio_filename: Some(PathBuf::from("test.mp3").into()),
        audio_lead_in: Some(555.into()),
        audio_hash: None,
        preview_time: Some(5.into()),
        countdown: Some(Countdown::Double),
        sample_set: Some(SampleSet::Soft),
        stack_leniency: Some(Decimal::from(dec!(0.9)).into()),
        mode: Some(Mode::Taiko),
        letterbox_in_breaks: Some(true.into()),
        story_fire_in_front: Some(false.into()),
        use_skin_sprites: Some(true.into()),
        always_show_playfield: Some(false.into()),
        overlay_position: Some(OverlayPosition::Above),
        skin_preference: Some("myskin".to_string().into()),
        epilepsy_warning: Some(true.into()),
        countdown_offset: Some(120.into()),
        special_style: Some(true.into()),
        widescreen_storyboard: Some(true.into()),
        samples_match_playback_rate: Some(true.into()),
        ..General::new()
    };

    assert_eq!(i, g);
    assert_eq!(i_str, i.to_string(14).unwrap());
}

#[test]
fn editor_parse_v14() {
    let i_str = "Bookmarks: 11018,21683,32349,37683,48349,59016,69683,80349,91016
DistanceSpacing: 0.8
BeatDivisor: 12
GridSize: 8
TimelineZoom: 2";
    let i = Editor::from_str(i_str, 14).unwrap().unwrap();

    let e = Editor {
        bookmarks: Some(
            vec![
                11018, 21683, 32349, 37683, 48349, 59016, 69683, 80349, 91016,
            ]
            .into(),
        ),
        distance_spacing: Some(Decimal::from(dec!(0.8)).into()),
        beat_divisor: Some(Decimal::from(dec!(12)).into()),
        grid_size: Some(8.into()),
        timeline_zoom: Some(Decimal::from(dec!(2)).into()),
        current_time: None,
    };

    assert_eq!(i, e);
    assert_eq!(i_str, i.to_string(14).unwrap());
}

#[test]
fn metadata_parse_v14() {
    let i_str = "Title:LOVE IS ORANGE
TitleUnicode:LOVE IS ORANGE
Artist:Orange Lounge
ArtistUnicode:Orange Lounge
Creator:Xnery
Version:Bittersweet Love
Source:beatmania IIDX 8th style
Tags:famoss 舟木智介 tomosuke funaki 徳井志津江 videogame ハードシャンソン Tart&Toffee
BeatmapID:3072232
BeatmapSetID:1499093";
    let i = Metadata::from_str(i_str, 14).unwrap().unwrap();

    let m = Metadata {
        title: Some("LOVE IS ORANGE".to_string().into()),
        title_unicode: Some("LOVE IS ORANGE".to_string().into()),
        artist: Some("Orange Lounge".to_string().into()),
        artist_unicode: Some("Orange Lounge".to_string().into()),
        creator: Some("Xnery".to_string().into()),
        version: Some("Bittersweet Love".to_string().into()),
        source: Some("beatmania IIDX 8th style".to_string().into()),
        tags: Some(
            vec![
                "famoss".to_string(),
                "舟木智介".to_string(),
                "tomosuke".to_string(),
                "funaki".to_string(),
                "徳井志津江".to_string(),
                "videogame".to_string(),
                "ハードシャンソン".to_string(),
                "Tart&Toffee".to_string(),
            ]
            .into(),
        ),
        beatmap_id: Some(3072232.into()),
        beatmap_set_id: Some(1499093.into()),
    };

    assert_eq!(i, m);
    assert_eq!(i_str, i.to_string(14).unwrap());
}

#[test]
fn difficulty_parse_v14() {
    let i_str = "HPDrainRate:8
CircleSize:5
OverallDifficulty:8
ApproachRate:5
SliderMultiplier:1.4
SliderTickRate:1";
    let i = Difficulty::from_str(i_str, 14).unwrap().unwrap();

    let d = Difficulty {
        hp_drain_rate: Some(Decimal::from(dec!(8)).into()),
        circle_size: Some(Decimal::from(dec!(5)).into()),
        overall_difficulty: Some(Decimal::from(dec!(8)).into()),
        approach_rate: Some(Decimal::from(dec!(5)).into()),
        slider_multiplier: Some(Decimal::from(dec!(1.4)).into()),
        slider_tickrate: Some(Decimal::from(rust_decimal::Decimal::ONE).into()),
    };

    assert_eq!(i, d);
    assert_eq!(i_str, i.to_string(14).unwrap());
}

#[test]
fn colours_parse_v14() {
    let i_str = "Combo1 : 255,128,255
SliderTrackOverride : 100,99,70
SliderBorder : 120,130,140";
    let i = Colours::from_str(i_str, 14).unwrap().unwrap();

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

    assert_eq!(i, Colours(c));
    assert_eq!(i_str, i.to_string(14).unwrap());
}

#[test]
fn timing_points_parse_v14() {
    let i_str = "10000,333.33,4,0,0,100,1,1
12000,-25,4,3,0,100,0,1";
    let i = TimingPoints::from_str(i_str, 14).unwrap().unwrap();

    let t = vec![
        TimingPoint::new_uninherited(
            10000,
            dec!(333.33).into(),
            4,
            timingpoints::SampleSet::BeatmapDefault,
            SampleIndex::OsuDefaultHitsounds,
            Volume::new(100, 14).unwrap(),
            Effects::new(true, false),
        ),
        TimingPoint::new_inherited(
            12000,
            dec!(4),
            4,
            timingpoints::SampleSet::Drum,
            SampleIndex::OsuDefaultHitsounds,
            Volume::new(100, 14).unwrap(),
            Effects::new(true, false),
        ),
    ];

    assert_eq!(i, TimingPoints(t));
    assert_eq!(i_str, i.to_string(14).unwrap());
}

#[test]
fn events_parse_v14() {
    let i_str = "0,0,\"bg2.jpg\",0,0
0,0,bg2.jpg,0,1
//Break Periods
2,100,163";
    let i = Events::from_str(i_str, 14).unwrap().unwrap();

    let e = Events(vec![
        Event::Background(Background {
            commands: Vec::new(),
            start_time: 0,
            file_name: Path::new("\"bg2.jpg\"").into(),
            position: Some(Position {
                x: dec!(0).into(),
                y: dec!(0).into(),
            }),
        }),
        Event::Background(Background {
            commands: Vec::new(),
            start_time: 0,
            file_name: Path::new("bg2.jpg").into(),
            position: Some(Position {
                x: dec!(0).into(),
                y: dec!(1).into(),
            }),
        }),
        Event::Comment("Break Periods".to_string()),
        Event::Break(Break::new(100, 163)),
    ]);

    assert_eq!(i, e);
    assert_eq!(i_str, i.to_string(14).unwrap());
}

#[test]
fn colour_parse_error() {
    let i = "Combo1: foo";
    let err = Colour::from_str(i, 14).unwrap_err();

    assert_eq!(err.to_string(), "Invalid red value");
}

#[test]
fn editor_bookmarks_error() {
    let i = "0,foo";
    let i_err = editor::Bookmarks::from_str(i, 14).unwrap_err();
    assert_eq!(i_err.to_string(), "invalid digit found in string");

    let i = "0-";
    let i_err = editor::Bookmarks::from_str(i, 14).unwrap_err();
    assert_eq!(i_err.to_string(), "invalid digit found in string");

    let i = "0,,";
    let i_err = editor::Bookmarks::from_str(i, 14).unwrap_err();
    assert_eq!(
        i_err.to_string(),
        "Invalid comma list, expected format of `key: value, value, value, ...`"
    );
}
