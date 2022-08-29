use crate::{
    InvalidRepr, Version, VersionedFrom, VersionedFromRepr, VersionedFromStr, VersionedToString,
};

use super::{ParseLayerLegacyError, ParseOriginTypeLegacyError};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[non_exhaustive]
pub enum OriginTypeLegacy {
    TopLeft,
    Centre,
    CentreLeft,
    TopRight,
    BottomCentre,
    TopCentre,
    Custom,
    CentreRight,
    BottomLeft,
    BottomRight,
}

impl VersionedToString for OriginTypeLegacy {
    fn to_string(&self, version: Version) -> Option<String> {
        let s = <usize as VersionedFrom<OriginTypeLegacy>>::from(*self, version).unwrap();

        Some(s.to_string())
    }
}

impl VersionedFrom<OriginTypeLegacy> for usize {
    fn from(value: OriginTypeLegacy, _: Version) -> Option<Self> {
        match value {
            OriginTypeLegacy::TopLeft => Some(0),
            OriginTypeLegacy::Centre => Some(1),
            OriginTypeLegacy::CentreLeft => Some(2),
            OriginTypeLegacy::TopRight => Some(3),
            OriginTypeLegacy::BottomCentre => Some(4),
            OriginTypeLegacy::TopCentre => Some(5),
            OriginTypeLegacy::Custom => Some(6),
            OriginTypeLegacy::CentreRight => Some(7),
            OriginTypeLegacy::BottomLeft => Some(8),
            OriginTypeLegacy::BottomRight => Some(9),
        }
    }
}

impl VersionedFromRepr for OriginTypeLegacy {
    fn from_repr(repr: usize, _: Version) -> Result<Option<Self>, InvalidRepr> {
        let repr = match repr {
            0 => OriginTypeLegacy::TopLeft,
            1 => OriginTypeLegacy::Centre,
            2 => OriginTypeLegacy::CentreLeft,
            3 => OriginTypeLegacy::TopRight,
            4 => OriginTypeLegacy::BottomCentre,
            5 => OriginTypeLegacy::TopCentre,
            6 => OriginTypeLegacy::Custom,
            7 => OriginTypeLegacy::CentreRight,
            8 => OriginTypeLegacy::BottomLeft,
            9 => OriginTypeLegacy::BottomRight,
            _ => return Err(InvalidRepr),
        };

        Ok(Some(repr))
    }
}

impl VersionedFromStr for OriginTypeLegacy {
    type Err = ParseOriginTypeLegacyError;

    fn from_str(s: &str, version: Version) -> std::result::Result<Option<Self>, Self::Err> {
        let s = s.parse()?;

        OriginTypeLegacy::from_repr(s, version).map_err(|e| e.into())
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[non_exhaustive]
pub enum LayerLegacy {
    Background,
    Fail,
    Pass,
    Foreground,
    Overlay,
    Video,
}

impl VersionedFromRepr for LayerLegacy {
    fn from_repr(repr: usize, _: Version) -> Result<Option<Self>, InvalidRepr> {
        let repr = match repr {
            0 => LayerLegacy::Background,
            1 => LayerLegacy::Fail,
            2 => LayerLegacy::Pass,
            3 => LayerLegacy::Foreground,
            4 => LayerLegacy::Overlay,
            5 => LayerLegacy::Video,
            _ => return Err(InvalidRepr),
        };

        Ok(Some(repr))
    }
}

impl VersionedFromStr for LayerLegacy {
    type Err = ParseLayerLegacyError;

    fn from_str(s: &str, version: Version) -> std::result::Result<Option<Self>, Self::Err> {
        let s = s.parse()?;

        LayerLegacy::from_repr(s, version).map_err(|e| e.into())
    }
}

impl VersionedFrom<LayerLegacy> for usize {
    fn from(value: LayerLegacy, _: Version) -> Option<Self> {
        match value {
            LayerLegacy::Background => Some(0),
            LayerLegacy::Fail => Some(1),
            LayerLegacy::Pass => Some(2),
            LayerLegacy::Foreground => Some(3),
            LayerLegacy::Overlay => Some(4),
            LayerLegacy::Video => Some(5),
        }
    }
}

impl VersionedToString for LayerLegacy {
    fn to_string(&self, version: Version) -> Option<String> {
        let s = <usize as VersionedFrom<LayerLegacy>>::from(*self, version).unwrap();

        Some(s.to_string())
    }
}
