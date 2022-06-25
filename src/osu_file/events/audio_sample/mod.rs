pub mod error;

use std::{fmt::Display, path::PathBuf, str::FromStr};

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
    osu_file::Integer,
    parsers::{comma, comma_field, comma_field_type, consume_rest_type},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AudioSample {
    pub time: Integer,
    // TODO i might have to make a separate layer type
    pub layer: Layer,
    // TODO make all file paths their own type, this would also solve problem of doing to_string double quotes stuff
    pub filepath: PathBuf,
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
                |v| Layer::from_repr(v),
            ),
        );
        let filepath = comma_field().map(PathBuf::from);
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
            self.time,
            self.layer as usize,
            self.filepath.to_string_lossy(),
            self.volume
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

impl Default for Volume {
    fn default() -> Self {
        Volume(None)
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
        write!(f, "{}", u8::from(*self).to_string())
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
