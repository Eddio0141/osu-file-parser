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
