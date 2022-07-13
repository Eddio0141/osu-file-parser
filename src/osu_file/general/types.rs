use crate::osu_file::{VersionedDefault, VersionedFromStr, VersionedToString, MIN_VERSION};
use strum_macros::{Display, EnumString, FromRepr};

use super::error::*;

/// Speed of the countdown before the first hitobject.
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash, FromRepr)]
#[non_exhaustive]
pub enum Countdown {
    /// No countdown.
    NoCountdown,
    /// Normal speed.
    Normal,
    /// Half speed.
    Half,
    /// Double speed.
    Double,
}

impl VersionedFromStr for Countdown {
    type Err = ParseCountdownSpeedError;

    fn from_str(s: &str, version: usize) -> std::result::Result<Option<Self>, Self::Err> {
        match version {
            MIN_VERSION..=4 => Ok(None),
            _ => Countdown::from_repr(s.parse()?)
                .ok_or(ParseCountdownSpeedError::UnknownType)
                .map(Some),
        }
    }
}

impl VersionedToString for Countdown {
    fn to_string(&self, version: usize) -> Option<String> {
        match version {
            MIN_VERSION..=4 => None,
            _ => Some((*self as usize).to_string()),
        }
    }
}

impl VersionedDefault for Countdown {
    fn default(version: usize) -> Option<Self> {
        match version {
            MIN_VERSION..=4 => None,
            _ => Some(Countdown::Normal),
        }
    }
}

/// Sample set that will be used if timing points do not override it
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash, Display, EnumString)]
#[non_exhaustive]
pub enum SampleSet {
    /// The `Normal` sample set.
    Normal,
    /// The `Soft` sample set.
    Soft,
    /// The `Drum` sample set.
    Drum,
    /// The `None` sample set, used before version 14.
    /// - Converted to `Normal` in version 14.
    None,
}

impl Default for SampleSet {
    fn default() -> Self {
        SampleSet::Normal
    }
}

impl VersionedFromStr for SampleSet {
    type Err = strum::ParseError;

    fn from_str(s: &str, version: usize) -> std::result::Result<Option<Self>, Self::Err> {
        match version {
            3 => Ok(None),
            4..=13 => Ok(Some(s.parse()?)),
            _ => {
                let mut sample_set = s.parse()?;

                if let SampleSet::None = sample_set {
                    sample_set = SampleSet::Normal;
                };

                Ok(Some(sample_set))
            }
        }
    }
}

impl VersionedToString for SampleSet {
    fn to_string(&self, version: usize) -> Option<String> {
        match version {
            3 => None,
            // I dont think we have to revert to None
            _ => Some(ToString::to_string(&self)),
        }
    }
}

impl VersionedDefault for SampleSet {
    fn default(version: usize) -> Option<Self> {
        match version {
            3 => None,
            _ => Some(Self::Normal),
        }
    }
}

/// Game mode of the .osu file
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash, FromRepr)]
#[non_exhaustive]
pub enum Mode {
    /// Osu! gamemode.
    Osu,
    /// Osu!Taiko gamemode.
    Taiko,
    /// Osu!Catch gamemode.
    Catch,
    /// Osu!Mania gamemode.
    Mania,
}

impl VersionedFromStr for Mode {
    type Err = ParseGameModeError;

    fn from_str(s: &str, version: usize) -> std::result::Result<Option<Self>, Self::Err> {
        let mode = Mode::from_repr(s.parse()?).ok_or(ParseGameModeError::UnknownType)?;

        let mode = match version {
            3..=13 if mode != Mode::Osu && mode != Mode::Mania => None,
            _ => Some(mode),
        };

        Ok(mode)
    }
}

impl VersionedToString for Mode {
    fn to_string(&self, _: usize) -> Option<String> {
        Some((*self as usize).to_string())
    }
}

impl VersionedDefault for Mode {
    fn default(_: usize) -> Option<Self> {
        Some(Self::Osu)
    }
}

/// Draw order of hit circle overlays compared to hit numbers
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash, Display, EnumString)]
#[non_exhaustive]
pub enum OverlayPosition {
    /// Use skin setting.
    NoChange,
    /// Draw overlays under numbers.
    Below,
    /// Draw overlays on top of numbers.
    Above,
}

impl Default for OverlayPosition {
    fn default() -> Self {
        Self::NoChange
    }
}

impl VersionedFromStr for OverlayPosition {
    type Err = strum::ParseError;

    fn from_str(s: &str, version: usize) -> std::result::Result<Option<Self>, Self::Err> {
        match version {
            MIN_VERSION..=13 => Ok(None),
            _ => Ok(Some(s.parse()?)),
        }
    }
}

impl VersionedToString for OverlayPosition {
    fn to_string(&self, version: usize) -> Option<String> {
        match version {
            MIN_VERSION..=13 => None,
            _ => Some(ToString::to_string(&self)),
        }
    }
}

impl VersionedDefault for OverlayPosition {
    fn default(version: usize) -> Option<Self> {
        match version {
            MIN_VERSION..=13 => None,
            _ => Some(Self::NoChange),
        }
    }
}
