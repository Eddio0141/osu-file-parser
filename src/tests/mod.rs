mod hitobject;
mod osu_files;
mod storyboard;

#[cfg(test)]
use pretty_assertions::assert_eq;
use rust_decimal::Decimal;
use std::{
    num::NonZeroUsize,
    path::{Path, PathBuf},
};

use rust_decimal_macros::dec;

use crate::osu_file::{
    self,
    colours::{Colour, Colours, Rgb},
    difficulty::Difficulty,
    editor::Editor,
    events::{Background, Break, Event, EventParams, Events},
    general::{CountdownSpeed, GameMode, General, OverlayPosition, SampleSet},
    hitobject::{
        types::{ComboSkipCount, HitSample, HitSound},
        HitObject, HitObjectParams, HitObjects,
    },
    metadata::Metadata,
    timingpoint::{self, Effects, SampleIndex, TimingPoint, TimingPoints, Volume},
    types::Position,
    OsuFile,
};

#[test]
fn general_parse() {
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
    let i = i_str.parse::<General>().unwrap();

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
    assert_eq!(i_str, i.to_string());
}

#[test]
fn editor_parse() {
    let i_str = "Bookmarks: 11018,21683,32349,37683,48349,59016,69683,80349,91016
DistanceSpacing: 0.8
BeatDivisor: 12
GridSize: 8
TimelineZoom: 2";
    let i: Editor = i_str.parse().unwrap();

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
    assert_eq!(i_str, i.to_string());
}

#[test]
fn metadata_parse() {
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
    let i: Metadata = i_str.parse().unwrap();

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
    assert_eq!(i_str, i.to_string());
}

#[test]
fn difficulty_parse() {
    let i_str = "HPDrainRate:8
CircleSize:5
OverallDifficulty:8
ApproachRate:5
SliderMultiplier:1.4
SliderTickRate:1";
    let i: Difficulty = i_str.parse().unwrap();

    let d = Difficulty {
        hp_drain_rate: Some(dec!(8)),
        circle_size: Some(dec!(5)),
        overall_difficulty: Some(dec!(8)),
        approach_rate: Some(dec!(5)),
        slider_multiplier: Some(dec!(1.4)),
        slider_tickrate: Some(Decimal::ONE),
    };

    assert_eq!(i, d);
    assert_eq!(i_str, i.to_string());
}

#[test]
fn colours_parse() {
    let i_str = "Combo1 : 255,128,255
SliderTrackOverride : 100,99,70
SliderBorder : 120,130,140";
    let i: Colours = i_str.parse().unwrap();

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
    assert_eq!(i_str, i.to_string());
}

#[test]
fn timing_points_parse() {
    let i_str = "10000,333.33,4,0,0,100,1,1
12000,-25,4,3,0,100,0,1";
    let i = i_str.parse::<TimingPoints>().unwrap();

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

    assert_eq!(i, TimingPoints(t));
    assert_eq!(i_str, i.to_string());
}

#[test]
fn events_parse() {
    let i_str = "0,0,\"bg2.jpg\",0,0
0,0,bg2.jpg,0,1
//Break Periods
2,100,163";
    let i: Events = i_str.parse().unwrap();

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
    assert_eq!(i_str, i.to_string());
}

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
            bookmarks: Some(vec![
                11018, 21683, 32349, 37683, 48349, 59016, 69683, 80349, 91016,
            ]),
            distance_spacing: Some(dec!(0.8)),
            beat_divisor: Some(dec!(12)),
            grid_size: Some(8),
            timeline_zoom: Some(dec!(2)),
        }),
        metadata: Some(Metadata {
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
                "shizue".to_string(),
                "tokui".to_string(),
                "ddr".to_string(),
                "dancedancerevolution".to_string(),
            ]),
            beatmap_id: Some(3072232),
            beatmap_set_id: Some(1499093),
        }),
        difficulty: Some(Difficulty {
            hp_drain_rate: Some(dec!(8)),
            circle_size: Some(dec!(5)),
            overall_difficulty: Some(dec!(8)),
            approach_rate: Some(dec!(5)),
            slider_multiplier: Some(dec!(1.4)),
            slider_tickrate: Some(Decimal::ONE),
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
            HitObject {
                position: Position { x: 256, y: 192 },
                time: 8016,
                obj_params: HitObjectParams::HitCircle,
                new_combo: false,
                combo_skip_count: ComboSkipCount::try_from(0).unwrap(),
                hitsound: HitSound::new(false, false, false, false),
                hitsample: HitSample::new(
                    osu_file::hitobject::types::SampleSet::NoCustomSampleSet,
                    osu_file::hitobject::types::SampleSet::NoCustomSampleSet,
                    None,
                    osu_file::hitobject::types::Volume::new(None).unwrap(),
                    "".to_string(),
                ),
            },
            HitObject {
                position: Position { x: 153, y: 192 },
                time: 8183,
                obj_params: HitObjectParams::HitCircle,
                new_combo: false,
                combo_skip_count: ComboSkipCount::try_from(0).unwrap(),
                // TODO use of builder pattern
                hitsound: HitSound::new(false, true, false, false),
                hitsample: HitSample::new(
                    osu_file::hitobject::types::SampleSet::NoCustomSampleSet,
                    osu_file::hitobject::types::SampleSet::NoCustomSampleSet,
                    None,
                    osu_file::hitobject::types::Volume::new(None).unwrap(),
                    "".to_string(),
                ),
            },
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
