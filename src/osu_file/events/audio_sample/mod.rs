pub mod error;

use std::{fmt::Display, str::FromStr};

pub use error::*;
use nom::{
    branch::alt,
    bytes::streaming::tag,
    combinator::{eof, map_opt},
    error::context,
    sequence::{preceded, tuple},
    Parser,
};
use strum_macros::FromRepr;

use crate::{
    osu_file::{FilePath, Integer},
    parsers::{comma, comma_field, comma_field_type, consume_rest_type},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AudioSample {
    pub time: Integer,
    pub layer: Layer,
    pub filepath: FilePath,
    pub volume: Volume,
}

impl FromStr for AudioSample {
    type Err = AudioSampleParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let header = context(AudioSampleParseError::MissingHeader.into(), tag("Sample"));
        let time = context(
            AudioSampleParseError::InvalidTime.into(),
            comma_field_type(),
        );
        let layer = context(
            AudioSampleParseError::InvalidLayer.into(),
            map_opt(
                context(
                    AudioSampleParseError::InvalidLayer.into(),
                    comma_field_type(),
                ),
                Layer::from_repr,
            ),
        );
        let filepath = comma_field().map(|p| p.into());
        let volume = alt((
            eof.map(|_| Volume::default()),
            preceded(
                context(AudioSampleParseError::MissingVolume.into(), comma()),
                context(
                    AudioSampleParseError::InvalidVolume.into(),
                    consume_rest_type(),
                ),
            ),
        ));

        let (_, (time, layer, filepath, volume)) = tuple((
            preceded(
                tuple((
                    header,
                    context(AudioSampleParseError::MissingTime.into(), comma()),
                )),
                time,
            ),
            preceded(
                context(AudioSampleParseError::MissingLayer.into(), comma()),
                layer,
            ),
            preceded(
                context(AudioSampleParseError::MissingFilepath.into(), comma()),
                filepath,
            ),
            volume,
        ))(s)?;

        Ok(AudioSample {
            time,
            layer,
            filepath,
            volume,
        })
    }
}

impl Display for AudioSample {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Sample,{},{},{}{}",
            self.time, self.layer as usize, self.filepath, self.volume
        )
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Volume(Option<u8>);

impl Volume {
    pub fn new(volume: u8) -> Result<Volume, VolumeSetError> {
        volume.try_into()
    }

    pub fn get(&self) -> u8 {
        self.0.unwrap_or(100)
    }

    pub fn set(&mut self, value: u8) -> Result<(), VolumeSetError> {
        *self = value.try_into()?;

        Ok(())
    }
}

impl TryFrom<u8> for Volume {
    type Error = VolumeSetError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 100 {
            Err(VolumeSetError::VolumeTooHigh(value))
        } else if value == 0 {
            Err(VolumeSetError::VolumeTooLow)
        } else {
            Ok(Volume(Some(value)))
        }
    }
}

impl From<Volume> for u8 {
    fn from(volume: Volume) -> Self {
        volume.get()
    }
}

impl FromStr for Volume {
    type Err = VolumeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Volume::try_from(s.parse::<u8>()?)?)
    }
}

impl Display for Volume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", u8::from(*self))
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, FromRepr)]
#[non_exhaustive]
pub enum Layer {
    Background,
    Fail,
    Pass,
    Foreground,
}
