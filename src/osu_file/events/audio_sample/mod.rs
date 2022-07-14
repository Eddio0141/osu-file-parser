pub mod error;

pub use error::*;
use nom::{
    branch::alt,
    bytes::streaming::tag,
    combinator::{eof, map_res},
    error::context,
    sequence::{preceded, tuple},
    Parser,
};

use crate::{
    osu_file::{
        FilePath, Integer, InvalidRepr, Version, VersionedFrom, VersionedFromRepr,
        VersionedFromStr, VersionedToString, VersionedTryFrom,
    },
    parsers::{comma, comma_field, comma_field_type},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AudioSample {
    pub time: Integer,
    pub layer: Layer,
    pub filepath: FilePath,
    pub volume: Volume,
}

impl VersionedFromStr for AudioSample {
    type Err = ParseAudioSampleError;

    fn from_str(s: &str, version: Version) -> Result<Option<Self>, Self::Err> {
        let header = context(ParseAudioSampleError::WrongEvent.into(), tag("Sample"));
        let time = context(
            ParseAudioSampleError::InvalidTime.into(),
            comma_field_type(),
        );
        let layer = context(
            ParseAudioSampleError::InvalidLayer.into(),
            map_res(
                context(
                    ParseAudioSampleError::InvalidLayer.into(),
                    comma_field_type(),
                ),
                // for now all of those unwraps are safe, can change on the future versions though
                |layer| Layer::from_repr(layer, version).map(|layer| layer.unwrap()),
            ),
        );
        let filepath = comma_field().map(|p| p.into());
        let volume = alt((
            eof.map(|_| Volume::default()),
            preceded(
                context(ParseAudioSampleError::MissingVolume.into(), comma()),
                context(
                    ParseAudioSampleError::InvalidVolume.into(),
                    map_res::<_, _, _, _, ParseVolumeError, _, _>(comma_field(), |s| {
                        Ok(Volume::from_str(s, version)?.unwrap())
                    }),
                ),
            ),
        ));

        let (_, (time, layer, filepath, volume)) = tuple((
            preceded(
                tuple((
                    header,
                    context(ParseAudioSampleError::MissingTime.into(), comma()),
                )),
                time,
            ),
            preceded(
                context(ParseAudioSampleError::MissingLayer.into(), comma()),
                layer,
            ),
            preceded(
                context(ParseAudioSampleError::MissingFilepath.into(), comma()),
                filepath,
            ),
            volume,
        ))(s)?;

        Ok(Some(AudioSample {
            time,
            layer,
            filepath,
            volume,
        }))
    }
}

impl VersionedToString for AudioSample {
    fn to_string(&self, version: Version) -> Option<String> {
        Some(format!(
            "Sample,{},{},{},{}",
            self.time,
            self.layer as usize,
            self.filepath.to_string(version).unwrap(),
            self.volume.to_string(version).unwrap()
        ))
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Volume(Option<u8>);

impl Volume {
    pub fn new(volume: u8, version: Version) -> Result<Volume, VolumeSetError> {
        <Volume as VersionedTryFrom<u8>>::try_from(volume, version).map(|volume| volume.unwrap())
    }

    pub fn get(&self) -> u8 {
        self.0.unwrap_or(100)
    }

    pub fn set(&mut self, value: u8, version: Version) -> Result<(), VolumeSetError> {
        *self = <Volume as VersionedTryFrom<u8>>::try_from(value, version)
            .map(|volume| volume.unwrap())?;

        Ok(())
    }
}

impl VersionedTryFrom<u8> for Volume {
    type Error = VolumeSetError;

    fn try_from(value: u8, _: Version) -> Result<Option<Self>, Self::Error> {
        if value > 100 {
            Err(VolumeSetError::VolumeTooHigh(value))
        } else if value == 0 {
            Err(VolumeSetError::VolumeTooLow)
        } else {
            Ok(Some(Volume(Some(value))))
        }
    }
}

impl VersionedFrom<Volume> for u8 {
    fn from(volume: Volume, _: Version) -> Option<Self> {
        Some(volume.get())
    }
}

impl VersionedFromStr for Volume {
    type Err = ParseVolumeError;

    fn from_str(s: &str, version: Version) -> Result<Option<Self>, Self::Err> {
        <Volume as VersionedTryFrom<u8>>::try_from(s.parse::<u8>()?, version)
            .map_err(|err| err.into())
    }
}

impl VersionedToString for Volume {
    fn to_string(&self, version: Version) -> Option<String> {
        <u8 as VersionedFrom<Volume>>::from(*self, version).map(|volume| volume.to_string())
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[non_exhaustive]
pub enum Layer {
    Background,
    Fail,
    Pass,
    Foreground,
}

impl VersionedFromRepr for Layer {
    fn from_repr(repr: usize, _: Version) -> Result<Option<Self>, InvalidRepr> {
        match repr {
            0 => Ok(Some(Layer::Background)),
            1 => Ok(Some(Layer::Fail)),
            2 => Ok(Some(Layer::Pass)),
            3 => Ok(Some(Layer::Foreground)),
            _ => Err(InvalidRepr),
        }
    }
}
