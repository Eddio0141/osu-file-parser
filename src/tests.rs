use rust_decimal_macros::dec;

use crate::osu_file::{general::{CountdownSpeed, GameMode, General, OverlayPosition, SampleSet}, editor::Editor};

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
SamplesMatchPlaybackRate: 1"
        .replace("\n", "\r\n");
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
TimelineZoom: 2".replace("\n", "\r\n");
    let i: Editor = i.parse().unwrap();

    let e = Editor {
        bookmarks: vec![11018,21683,32349,37683,48349,59016,69683,80349,91016],
        distance_spacing: dec!(0.8),
        beat_divisor: dec!(12),
        grid_size: 8,
        timeline_zoom: dec!(2),
    };

    assert_eq!(i, e);
}