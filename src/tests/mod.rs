mod error_line_index;
mod hitobjects;
mod osu_files;
mod storyboard;

#[cfg(test)]
use pretty_assertions::assert_eq;
use rust_decimal::Decimal;
use std::path::{Path, PathBuf};

use rust_decimal_macros::dec;

use crate::osu_file::{
    colours::{Colour, Colours, Rgb},
    difficulty::Difficulty,
    editor::Editor,
    events::{Background, Break, Event, EventParams, Events},
    general::{CountdownSpeed, GameMode, General, OverlayPosition, SampleSet},
    metadata::Metadata,
    timingpoints,
    timingpoints::{Effects, SampleIndex, TimingPoint, TimingPoints, Volume},
    types::Position,
    Version,
};

#[test]
fn general_parse_v14() {
    let i_str = "AudioFilename: test.mp3
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
    let i = General::from_str_v14(i_str).unwrap().unwrap();

    let g = General {
        audio_filename: Some(PathBuf::from("test.mp3")),
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
    assert_eq!(i_str, i.to_string_v14());
}

#[test]
fn editor_parse_v14() {
    let i_str = "Bookmarks: 11018,21683,32349,37683,48349,59016,69683,80349,91016
DistanceSpacing: 0.8
BeatDivisor: 12
GridSize: 8
TimelineZoom: 2";
    let i = Editor::from_str_v14(i_str).unwrap().unwrap();

    let e = Editor {
        bookmarks: Some(vec![
            11018, 21683, 32349, 37683, 48349, 59016, 69683, 80349, 91016,
        ]),
        distance_spacing: Some(dec!(0.8)),
        beat_divisor: Some(dec!(12)),
        grid_size: Some(8),
        timeline_zoom: Some(dec!(2)),
    };

    assert_eq!(i, e);
    assert_eq!(i_str, i.to_string_v14());
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
    let i = Metadata::from_str_v14(i_str).unwrap().unwrap();

    let m = Metadata {
        title: Some("LOVE IS ORANGE".to_string()),
        title_unicode: Some("LOVE IS ORANGE".to_string()),
        artist: Some("Orange Lounge".to_string()),
        artist_unicode: Some("Orange Lounge".to_string()),
        creator: Some("Xnery".to_string()),
        version: Some("Bittersweet Love".to_string()),
        source: Some("beatmania IIDX 8th style".to_string()),
        tags: Some(vec![
            "famoss".to_string(),
            "舟木智介".to_string(),
            "tomosuke".to_string(),
            "funaki".to_string(),
            "徳井志津江".to_string(),
            "videogame".to_string(),
            "ハードシャンソン".to_string(),
            "Tart&Toffee".to_string(),
        ]),
        beatmap_id: Some(3072232),
        beatmap_set_id: Some(1499093),
    };

    assert_eq!(i, m);
    assert_eq!(i_str, i.to_string_v14());
}

#[test]
fn difficulty_parse_v14() {
    let i_str = "HPDrainRate:8
CircleSize:5
OverallDifficulty:8
ApproachRate:5
SliderMultiplier:1.4
SliderTickRate:1";
    let i = Difficulty::from_str_v14(i_str).unwrap().unwrap();

    let d = Difficulty {
        hp_drain_rate: Some(dec!(8)),
        circle_size: Some(dec!(5)),
        overall_difficulty: Some(dec!(8)),
        approach_rate: Some(dec!(5)),
        slider_multiplier: Some(dec!(1.4)),
        slider_tickrate: Some(Decimal::ONE),
    };

    assert_eq!(i, d);
    assert_eq!(i_str, i.to_string_v14());
}

#[test]
fn colours_parse_v14() {
    let i_str = "Combo1 : 255,128,255
SliderTrackOverride : 100,99,70
SliderBorder : 120,130,140";
    let i = Colours::from_str_v14(i_str).unwrap().unwrap();

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
    assert_eq!(i_str, i.to_string_v14());
}

#[test]
fn timing_points_parse_v14() {
    let i_str = "10000,333.33,4,0,0,100,1,1
12000,-25,4,3,0,100,0,1";
    let i = TimingPoints::from_str_v14(i_str).unwrap().unwrap();

    let t = vec![
        TimingPoint::new_uninherited(
            10000,
            dec!(333.33),
            4,
            timingpoints::SampleSet::BeatmapDefault,
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
            timingpoints::SampleSet::Drum,
            SampleIndex::OsuDefaultHitsounds,
            Volume::new(100).unwrap(),
            Effects {
                kiai_time_enabled: true,
                no_first_barline_in_taiko_mania: false,
            },
        ),
    ];

    assert_eq!(i, TimingPoints(t));
    assert_eq!(i_str, i.to_string_v14());
}

#[test]
fn events_parse_v14() {
    let i_str = "0,0,\"bg2.jpg\",0,0
0,0,bg2.jpg,0,1
//Break Periods
2,100,163";
    let i = Events::from_str_v14(i_str).unwrap().unwrap();

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
                Position { x: 0, y: 1 },
            )),
        },
        Event::Comment("Break Periods".to_string()),
        Event::NormalEvent {
            start_time: 100,
            event_params: EventParams::Break(Break { end_time: 163 }),
        },
    ]);

    assert_eq!(i, e);
    assert_eq!(i_str, i.to_string_v14());
}

#[test]
fn colour_parse_error() {
    let i = "Combo1: foo";
    let err = i.parse::<Colour>().unwrap_err();

    assert_eq!(err.to_string(), "Invalid red value");
}
