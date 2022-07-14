use crate::osu_file::{
    Version, VersionedDefault, VersionedFromStr, VersionedToString, MIN_VERSION,
};
use strum_macros::FromRepr;

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

    fn from_str(s: &str, version: Version) -> std::result::Result<Option<Self>, Self::Err> {
        match version {
            MIN_VERSION..=4 => Ok(None),
            _ => Countdown::from_repr(s.parse()?)
                .ok_or(ParseCountdownSpeedError::UnknownType)
                .map(Some),
        }
    }
}

impl VersionedToString for Countdown {
    fn to_string(&self, version: Version) -> Option<String> {
        match version {
            MIN_VERSION..=4 => None,
            _ => Some((*self as usize).to_string()),
        }
    }
}

impl VersionedDefault for Countdown {
    fn default(version: Version) -> Option<Self> {
        match version {
            MIN_VERSION..=4 => None,
            _ => Some(Countdown::Normal),
        }
    }
}

/// Sample set that will be used if timing points do not override it
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
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

impl VersionedToString for SampleSet {
    fn to_string(&self, version: Version) -> Option<String> {
        match version {
            3 => None,
            // I dont think we have to revert to None
            _ => Some(
                match self {
                    SampleSet::Normal => "Normal",
                    SampleSet::Soft => "Soft",
                    SampleSet::Drum => "Drum",
                    SampleSet::None => "None",
                }
                .to_string(),
            ),
        }
    }
}

impl Default for SampleSet {
    fn default() -> Self {
        SampleSet::Normal
    }
}

impl VersionedFromStr for SampleSet {
    type Err = ParseSampleSetError;

    fn from_str(s: &str, version: Version) -> std::result::Result<Option<Self>, Self::Err> {
        let sample_set_from_str = |s| match s {
            "Normal" => Ok(SampleSet::Normal),
            "Soft" => Ok(SampleSet::Soft),
            "Drum" => Ok(SampleSet::Drum),
            "None" => Ok(SampleSet::None),
            _ => Err(ParseSampleSetError::UnknownVariant),
        };

        match version {
            3 => Ok(None),
            4..=13 => sample_set_from_str(s).map(Some),
            _ => {
                let mut sample_set = sample_set_from_str(s)?;

                if let SampleSet::None = sample_set {
                    sample_set = SampleSet::Normal;
                };

                Ok(Some(sample_set))
            }
        }
    }
}

impl VersionedDefault for SampleSet {
    fn default(version: Version) -> Option<Self> {
        match version {
            3 => None,
            _ => Some(Self::Normal),
        }
    }
}

/// Game mode of the .osu file
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
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

    fn from_str(s: &str, version: Version) -> std::result::Result<Option<Self>, Self::Err> {
        let mode = s.parse::<usize>()?;

        let mode = match mode {
            0 => Ok(Mode::Osu),
            1 => Ok(Mode::Taiko),
            2 => Ok(Mode::Catch),
            3 => Ok(Mode::Mania),
            _ => Err(ParseGameModeError::UnknownVariant),
        }?;

        let mode = match version {
            3..=13 if mode != Mode::Osu && mode != Mode::Mania => None,
            _ => Some(mode),
        };

        Ok(mode)
    }
}

impl VersionedToString for Mode {
    fn to_string(&self, _: Version) -> Option<String> {
        Some((*self as usize).to_string())
    }
}

impl VersionedDefault for Mode {
    fn default(_: Version) -> Option<Self> {
        Some(Self::Osu)
    }
}

/// Draw order of hit circle overlays compared to hit numbers
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
#[non_exhaustive]
pub enum OverlayPosition {
    /// Use skin setting.
    NoChange,
    /// Draw overlays under numbers.
    Below,
    /// Draw overlays on top of numbers.
    Above,
}

impl VersionedToString for OverlayPosition {
    fn to_string(&self, version: Version) -> Option<String> {
        match version {
            MIN_VERSION..=13 => None,
            _ => Some(
                match self {
                    OverlayPosition::NoChange => "NoChange",
                    OverlayPosition::Below => "Below",
                    OverlayPosition::Above => "Above",
                }
                .to_string(),
            ),
        }
    }
}

impl Default for OverlayPosition {
    fn default() -> Self {
        Self::NoChange
    }
}

impl VersionedFromStr for OverlayPosition {
    type Err = OverlayPositionParseError;

    fn from_str(s: &str, version: Version) -> std::result::Result<Option<Self>, Self::Err> {
        match version {
            MIN_VERSION..=13 => Ok(None),
            _ => match s {
                "NoChange" => Ok(Some(OverlayPosition::NoChange)),
                "Below" => Ok(Some(OverlayPosition::Below)),
                "Above" => Ok(Some(OverlayPosition::Above)),
                _ => Err(OverlayPositionParseError::UnknownVariant),
            },
        }
    }
}

impl VersionedDefault for OverlayPosition {
    fn default(version: Version) -> Option<Self> {
        match version {
            MIN_VERSION..=13 => None,
            _ => Some(Self::NoChange),
        }
    }
}
