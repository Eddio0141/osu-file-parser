use rust_decimal_macros::dec;

use crate::osu_file::{
    colours::{Colour, Rgb},
    difficulty::Difficulty,
    editor::Editor,
    events::{Background, Break, Event, EventParams},
    general::{CountdownSpeed, GameMode, General, OverlayPosition, SampleSet},
    metadata::Metadata,
    timingpoint::{self, Effects, SampleIndex, TimingPoint, Volume},
    Position,
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
        audio_filename: "test.mp3".to_string(),
        audio_lead_in: 555,
        audio_hash: "no.mp3".to_string(),
        preview_time: 5,
        countdown: CountdownSpeed::Double,
        sample_set: SampleSet::Soft,
        stack_leniency: dec!(0.9),
        mode: GameMode::Taiko,
        letterbox_in_breaks: true,
        story_fire_in_front: false,
        use_skin_sprites: true,
        always_show_playfield: false,
        overlay_position: OverlayPosition::Above,
        skin_preference: "myskin".to_string(),
        epilepsy_warning: true,
        countdown_offset: 120,
        special_style: true,
        widescreen_storyboard: true,
        samples_match_playback_rate: true,
    };

    assert_eq!(i, g);
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
fn events_parse() {
    let i = "0,0,\"bg2.jpg\",0,0
0,0,bg2.jpg,0,0
//Break Periods
2,100,163";
    let i: Vec<Event> = i.lines().map(|event| event.parse().unwrap()).collect();

    let e = vec![
        Event::NormalEvent {
            start_time: 0,
            event_params: EventParams::Background(Background::new(
                "\"bg2.jpg\"".to_string(),
                Position { x: 0, y: 0 },
            )),
        },
        Event::NormalEvent {
            start_time: 0,
            event_params: EventParams::Background(Background::new(
                "bg2.jpg".to_string(),
                Position { x: 0, y: 0 },
            )),
        },
        Event::Comment("Break Periods".to_string()),
        Event::NormalEvent {
            start_time: 100,
            event_params: EventParams::Break(Break { end_time: 163 }),
        },
    ];

    assert_eq!(i, e);
}
