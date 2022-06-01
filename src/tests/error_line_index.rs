use pretty_assertions::assert_eq;

use crate::osu_file::*;

#[test]
fn general() {
    let general = "AudioFilename: audio.mp3\nfoo: bar";
    let osu_file = format!("osu file format v14\n\n\n\n[General]\n{general}");
    let general_err = General::from_str(general, 14).unwrap_err();
    let osu_file_err = osu_file.parse::<OsuFile>().unwrap_err();

    assert_eq!(general_err.line_index(), 1);
    assert_eq!(osu_file_err.line_index(), 6);
}

#[test]
fn colours() {
    let colours = "Combo1 : 255,128,255\nfoobar";
    let osu_file = format!("osu file format v14\n\n\n\n[Colours]\n{colours}");
    let colours_err = Colours::from_str_v14(colours).unwrap_err();
    let osu_file_err = osu_file.parse::<OsuFile>().unwrap_err();

    assert_eq!(colours_err.line_index(), 1);
    assert_eq!(osu_file_err.line_index(), 6);
}

#[test]
fn difficulty() {
    let difficulty = "HPDrainRate:8\nfoobar";
    let osu_file = format!("osu file format v14\n\n\n\n[Difficulty]\n{difficulty}");
    let difficulty_err = Difficulty::from_str_v14(difficulty).unwrap_err();
    let osu_file_err = osu_file.parse::<OsuFile>().unwrap_err();

    assert_eq!(difficulty_err.line_index(), 1);
    assert_eq!(osu_file_err.line_index(), 6);
}

#[test]
fn editor() {
    let editor = "DistanceSpacing: 0.8\nfoobar";
    let osu_file = format!("osu file format v14\n\n\n\n[Editor]\n{editor}");
    let editor_err = Editor::from_str_v14(editor).unwrap_err();
    let osu_file_err = osu_file.parse::<OsuFile>().unwrap_err();

    assert_eq!(editor_err.line_index(), 1);
    assert_eq!(osu_file_err.line_index(), 6);
}

#[test]
fn events() {
    let events = "0,0,\"bg.jpg\",0,0\nfoobar";
    let osu_file = format!("osu file format v14\n\n\n\n[Events]\n{events}");
    let events_err = Events::from_str_v14(events).unwrap_err();
    let osu_file_err = osu_file.parse::<OsuFile>().unwrap_err();

    assert_eq!(events_err.line_index(), 1);
    assert_eq!(osu_file_err.line_index(), 6);
}

#[test]
fn hitobjects() {
    let hitobjects = "51,192,350,128,2,849:0:0:0:0:\nfoobar";
    let osu_file = format!("osu file format v14\n\n\n\n[HitObjects]\n{hitobjects}");
    let hitobjects_err = HitObjects::from_str_v14(hitobjects).unwrap_err();
    let osu_file_err = osu_file.parse::<OsuFile>().unwrap_err();

    assert_eq!(hitobjects_err.line_index(), 1);
    assert_eq!(osu_file_err.line_index(), 6);
}

#[test]
fn metadata() {
    let metadata = "Title:foo\nfoobar";
    let osu_file = format!("osu file format v14\n\n\n\n[Metadata]\n{metadata}");
    let metadata_err = Metadata::from_str_v14(metadata).unwrap_err();
    let osu_file_err = osu_file.parse::<OsuFile>().unwrap_err();

    assert_eq!(metadata_err.line_index(), 1);
    assert_eq!(osu_file_err.line_index(), 6);
}

#[test]
fn timingpoints() {
    let timingpoints = "350,333.333333333333,4,2,1,60,1,0\nfoobar";
    let osu_file = format!("osu file format v14\n\n\n\n[TimingPoints]\n{timingpoints}");
    let timingpoints_err = TimingPoints::from_str_v14(timingpoints).unwrap_err();
    let osu_file_err = osu_file.parse::<OsuFile>().unwrap_err();

    assert_eq!(timingpoints_err.line_index(), 1);
    assert_eq!(osu_file_err.line_index(), 6);
}

#[test]
fn double_sections() {
    let osu_file = "osu file format v14\n[General]\n\nAudioFilename: audio.mp3\n\n\n[TimingPoints]\n350,333.333333333333,4,2,1,60,1,0\nfoobar";
    let osu_file_err = osu_file.parse::<OsuFile>().unwrap_err();

    assert_eq!(osu_file_err.line_index(), 8);
}
