use strum_macros::{EnumString, IntoStaticStr};
use thiserror::Error;

use crate::{helper::macros::verbose_error_to_error, osu_file};

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
    ParseError(#[from] crate::osu_file::types::Error<osu_file::events::ParseError>),
}

verbose_error_to_error!(ParseError);
