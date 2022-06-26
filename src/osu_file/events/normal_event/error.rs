use strum_macros::{EnumString, IntoStaticStr};
use thiserror::Error;

use crate::helper::macros::verbose_error_to_error;

#[non_exhaustive]
#[derive(Debug, Error, IntoStaticStr, EnumString)]
pub enum BackgroundParseError {
    #[error("Wrong event type")]
    WrongEventType,
    #[error("Missing `start_time` field")]
    MissingStartTime,
    #[error("Invalid `start_time` value")]
    InvalidStartTime,
    #[error("Missing `file_name` field")]
    MissingFileName,
    #[error("Missing `x` field")]
    MissingX,
    #[error("Invalid `x` value")]
    InvalidX,
    #[error("Missing `y` field")]
    MissingY,
    #[error("Invalid `y` value")]
    InvalidY,
}

verbose_error_to_error!(BackgroundParseError);

#[non_exhaustive]
#[derive(Debug, Error, IntoStaticStr, EnumString)]
pub enum VideoParseError {
    #[error("Wrong event type")]
    WrongEventType,
    #[error("Missing `start_time` field")]
    MissingStartTime,
    #[error("Invalid `start_time` value")]
    InvalidStartTime,
    #[error("Missing `file_name` field")]
    MissingFileName,
    #[error("Missing `x` field")]
    MissingX,
    #[error("Invalid `x` value")]
    InvalidX,
    #[error("Missing `y` field")]
    MissingY,
    #[error("Invalid `y` value")]
    InvalidY,
}

verbose_error_to_error!(VideoParseError);

#[non_exhaustive]
#[derive(Debug, Error, IntoStaticStr, EnumString)]
pub enum BreakParseError {
    #[error("Wrong event type")]
    WrongEventType,
    #[error("Missing `start_time` field")]
    MissingStartTime,
    #[error("Invalid `start_time` value")]
    InvalidStartTime,
    #[error("Missing `end_time` field")]
    MissingEndTime,
    #[error("Invalid `end_time` value")]
    InvalidEndTime,
}

verbose_error_to_error!(BreakParseError);

#[non_exhaustive]
#[derive(Debug, Error, IntoStaticStr, EnumString)]
pub enum ColourTransformationParseError {
    #[error("Wrong event type")]
    WrongEventType,
    #[error("Missing `start_time` field")]
    MissingStartTime,
    #[error("Invalid `start_time` value")]
    InvalidStartTime,
    #[error("Missing `end_time` field")]
    MissingEndTime,
    #[error("Invalid `end_time` value")]
    InvalidEndTime,
    #[error("Missing `red` field")]
    MissingRed,
    #[error("Invalid `red` value")]
    InvalidRed,
    #[error("Missing `green` field")]
    MissingGreen,
    #[error("Invalid `green` value")]
    InvalidGreen,
    #[error("Missing `blue` field")]
    MissingBlue,
    #[error("Invalid `blue` value")]
    InvalidBlue,
}

verbose_error_to_error!(ColourTransformationParseError);
