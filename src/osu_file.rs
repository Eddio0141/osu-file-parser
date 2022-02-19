/// An .osu file represented as a struct
pub struct OsuFile {
    version: u64,
    general: General,
    editor: Editor,
    metadata: Metadata,
    difficulty: Difficulty,
    events: Events,
    timing_points: TimingPoints,
    colours: Colours,
    hitobjects: Vec<HitObject>,
}

pub struct General;

pub struct Editor;

pub struct Metadata;

pub struct Difficulty;

pub struct Events;

pub struct TimingPoints;

pub struct Colours;

pub struct HitObject;
