use strum_macros::{EnumString, IntoStaticStr};
use thiserror::Error;

use crate::{helper::macros::verbose_error_to_error, osu_file::events};

#[derive(Debug, Error, EnumString, IntoStaticStr)]
#[non_exhaustive]
pub enum ParseError {
    /// Unexpected line before any section.
    #[error("Unexpected line before any section")]
    UnexpectedLine,
    /// Duplicate section names defined.
    #[error("There are multiple sections defined as the same name")]
    DuplicateSections,
    /// Unknown section name defined.
    #[error("There is an unknown section")]
    UnknownSection,
    #[error(transparent)]
    #[strum(disabled)]
    ParseVariableError(#[from] ParseVariableError),
    #[error(transparent)]
    #[strum(disabled)]
    ParseEventsError(#[from] events::ParseError),
}

verbose_error_to_error!(ParseError);

#[derive(Debug, Error, EnumString, IntoStaticStr)]
#[non_exhaustive]
pub enum ParseVariableError {
    #[error("Missing the header `$`")]
    MissingHeader,
    #[error("Missing `=` for assignment")]
    MissingEquals,
}

verbose_error_to_error!(ParseVariableError);
