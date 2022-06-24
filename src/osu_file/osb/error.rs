use strum_macros::{EnumString, IntoStaticStr};
use thiserror::Error;

use crate::{
    helper::macros::verbose_error_to_error,
    osu_file::{self, events::storyboard},
};

#[derive(Debug, Error, EnumString, IntoStaticStr)]
#[non_exhaustive]
pub enum ParseError {
    /// Duplicate section names defined.
    #[error("There are multiple sections defined as the same name")]
    DuplicateSections,
    /// Unknown section name defined.
    #[error("There is an unknown section")]
    UnknownSection,
    #[error(transparent)]
    #[strum(disabled)]
    VariableParseError(#[from] VariableParseError),
    #[error(transparent)]
    #[strum(disabled)]
    ObjectParseError(#[from] storyboard::error::ObjectParseError),
    #[error(transparent)]
    #[strum(disabled)]
    EventsParseError(#[from] osu_file::types::Error<osu_file::events::ParseError>),
}

verbose_error_to_error!(ParseError);

#[derive(Debug, Error, EnumString, IntoStaticStr)]
pub enum VariableParseError {
    #[error("Missing the header `$`")]
    MissingHeader,
    #[error("Missing `=` for assignment")]
    MissingEquals,
}

verbose_error_to_error!(VariableParseError);
