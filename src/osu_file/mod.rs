pub mod general;

use self::general::General;

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

pub const DELIMITER: char = ':';

pub type Integer = i32;
pub type Decimal = f32;

pub struct Editor;

pub struct Metadata;

pub struct Difficulty;

pub struct Events;

pub struct TimingPoints;

pub struct Colours;

pub struct HitObject;
