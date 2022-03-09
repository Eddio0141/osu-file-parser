//! Module defining `error` types that's used for the `hitobject` related modules.

use std::{error::Error, fmt::Display, num::ParseIntError};

use crate::osu_file::OsuFileParseError;

#[derive(Debug)]
/// Error used when there is a problem parsing a str having a `F:S` format.
pub enum ColonSetParseError {
    /// When the first item is missing.
    MissingFirstItem,
    /// When the second item is missing.
    MissingSecondItem,
    /// There are more than 2 items defined.
    MoreThanTwoItems(usize),
    /// There was some problem parsing the value.
    ValueParseError {
        source: Box<dyn Error>,
        index: usize,
    },
}

impl Display for ColonSetParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err = match self {
            ColonSetParseError::MissingFirstItem => {
                "Missing the first item in the colon set.".to_string()
            }
            ColonSetParseError::MissingSecondItem => {
                "Missing the second item in the colon set.".to_string()
            }
            ColonSetParseError::MoreThanTwoItems(index) => {
                format!(
                    "There is more than 2 items in the colon set, another defined at index {index}"
                )
            }
            ColonSetParseError::ValueParseError { index, .. } => {
                format!("There was a problem parsing a value to a colon set item at index {index}.")
            }
        };

        write!(f, "{err}")
    }
}

impl Error for ColonSetParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if let ColonSetParseError::ValueParseError { source, .. } = self {
            Some(source.as_ref())
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub enum HitObjectParseError {
    MissingProperty(usize),
    ValueParseError { index: usize, err: Box<dyn Error> },
}

impl Display for HitObjectParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err = match self {
            HitObjectParseError::MissingProperty(ordinal_pos) => {
                format!("The property for index {ordinal_pos} of the object is missing.")
            }
            HitObjectParseError::ValueParseError {
                index: property_index,
                ..
            } => {
                format!("There was a problem parsing the property for index {property_index}.")
            }
        };

        write!(f, "{err}")
    }
}

impl Error for HitObjectParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if let HitObjectParseError::ValueParseError { err, .. } = self {
            Some(err.as_ref())
        } else {
            None
        }
    }
}

impl From<HitObjectParseError> for OsuFileParseError {
    fn from(err: HitObjectParseError) -> Self {
        Self::SectionParseError {
            source: Box::new(err),
        }
    }
}

impl From<VolumeParseError> for HitSampleParseError {
    fn from(err: VolumeParseError) -> Self {
        Self::ParseError(Box::new(err))
    }
}

impl From<SampleSetParseError> for HitSampleParseError {
    fn from(err: SampleSetParseError) -> Self {
        Self::ParseError(Box::new(err))
    }
}

#[derive(Debug)]
pub enum HitSampleParseError {
    MissingProperty,
    ParseError(Box<dyn Error>),
}

impl Error for HitSampleParseError {}

impl Display for HitSampleParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err = match self {
            HitSampleParseError::MissingProperty => "A property is missing",
            HitSampleParseError::ParseError(_) => "There was a problem parsing a value",
        };

        write!(f, "{err}")
    }
}

impl From<ParseIntError> for HitSampleParseError {
    fn from(err: ParseIntError) -> Self {
        Self::ParseError(Box::new(err))
    }
}

impl From<ParseIntError> for SampleSetParseError {
    fn from(err: ParseIntError) -> Self {
        Self::ValueParseError(Box::new(err))
    }
}

#[derive(Debug)]
pub enum SampleSetParseError {
    ValueHigherThanThree,
    ValueParseError(Box<dyn Error>),
}

impl Display for SampleSetParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err = match self {
            SampleSetParseError::ValueHigherThanThree => "The value parsing is higher than 3.",
            SampleSetParseError::ValueParseError(_) => "There was a problem parsing the value.",
        };

        write!(f, "{err}")
    }
}

impl Error for SampleSetParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if let SampleSetParseError::ValueParseError(err) = self {
            Some(err.as_ref())
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub enum VolumeSetError {
    VolumeTooHigh(u8),
    VolumeTooLow,
}

impl Display for VolumeSetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err = match self {
            VolumeSetError::VolumeTooHigh(got) => format!("Volume too high, got {got}."),
            VolumeSetError::VolumeTooLow => "Volume too low, got 0.".to_string(),
        };

        write!(f, "{err} Expected volume to be in between `1` ~ `100`.")
    }
}

impl Error for VolumeSetError {}

#[derive(Debug)]
pub enum VolumeParseError {
    VolumeTooHigh,
    VolumeTooLow,
    InvalidString(Box<dyn Error>),
}

impl From<ParseIntError> for VolumeParseError {
    fn from(err: ParseIntError) -> Self {
        Self::InvalidString(Box::new(err))
    }
}

impl Display for VolumeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err = match self {
            VolumeParseError::VolumeTooHigh => {
                "Volume is too high. Requires to be in the range of 1 ~ 100"
            }
            VolumeParseError::VolumeTooLow => {
                "Volume is too low. Requires to be in the range of 1 ~ 100"
            }
            VolumeParseError::InvalidString(_) => "Invalid string",
        };

        write!(f, "{err}")
    }
}

impl Error for VolumeParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if let VolumeParseError::InvalidString(err) = self {
            Some(err.as_ref())
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct CurveTypeParseError;

impl Error for CurveTypeParseError {}

impl Display for CurveTypeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error, tried to parse an invalid string as curve type.")
    }
}

#[derive(Debug)]
pub struct PipeVecParseErr(Box<dyn Error>);

impl PipeVecParseErr {
    pub fn new(err: Box<dyn Error>) -> Self {
        Self(err)
    }
}

impl Display for PipeVecParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "There was a problem parsing a pipe-separated list of values"
        )
    }
}

impl Error for PipeVecParseErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.0.as_ref())
    }
}

impl From<ParseIntError> for PipeVecParseErr {
    fn from(err: ParseIntError) -> Self {
        PipeVecParseErr(Box::new(err))
    }
}

impl From<ColonSetParseError> for PipeVecParseErr {
    fn from(err: ColonSetParseError) -> Self {
        PipeVecParseErr(Box::new(err))
    }
}
