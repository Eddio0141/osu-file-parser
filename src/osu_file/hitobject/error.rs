use std::{error::Error, fmt::Display, num::ParseIntError};

use crate::osu_file::OsuFileParseError;

#[derive(Debug)]
pub struct ComboSkipCountParseError;

impl Display for ComboSkipCountParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "There was a problem parsing a value to a 3 bit value")
    }
}

impl Error for ComboSkipCountParseError {}

impl From<SampleSetParseError> for ColonSetParseError {
    fn from(err: SampleSetParseError) -> Self {
        ColonSetParseError::ValueParseError(Box::new(err))
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

impl From<ParseIntError> for ColonSetParseError {
    fn from(err: ParseIntError) -> Self {
        ColonSetParseError::ValueParseError(Box::new(err))
    }
}

#[derive(Debug)]
pub enum HitObjectParseError {
    MissingProperty(usize),
    ValueParseError {
        property_index: usize,
        err: Box<dyn Error>,
    },
}

impl Display for HitObjectParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err = match self {
            HitObjectParseError::MissingProperty(ordinal_pos) => {
                format!("The property for index {ordinal_pos} of the object is missing")
            }
            HitObjectParseError::ValueParseError { property_index, .. } => {
                format!("There was a problem parsing the property for index {property_index}")
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
pub enum VolumeParseError {
    VolumeTooHigh,
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
                "Volume is too high. Requires to be in the range of 0 ~ 100"
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
pub enum ColonSetParseError {
    MissingFirstItem,
    MissingSecondItem,
    MoreThanTwoItems,
    ValueParseError(Box<dyn Error>),
}

impl Display for ColonSetParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err = match self {
            ColonSetParseError::MissingFirstItem => "Missing the first item in the colon set",
            ColonSetParseError::MissingSecondItem => "Missing the second item in the colon set",
            ColonSetParseError::MoreThanTwoItems => "There is more than 2 items in the colon set",
            ColonSetParseError::ValueParseError(_) => {
                "There is a problem parsing a value to a colon set item"
            }
        };

        write!(f, "{err}")
    }
}

impl Error for ColonSetParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if let ColonSetParseError::ValueParseError(err) = self {
            Some(err.as_ref())
        } else {
            None
        }
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
