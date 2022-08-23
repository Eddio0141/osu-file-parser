use std::{
    fmt::{Debug, Display},
    path::{Path, PathBuf},
    str::FromStr,
};

use either::Either;
use rust_decimal_macros::dec;
use thiserror::Error;

/// Definition of the `Integer` type.
pub type Integer = i32;

pub const LATEST_VERSION: Version = 14;
pub const MIN_VERSION: Version = 3;

pub type Version = u8;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// The position of something in `osu!pixels` with the `x` `y` form.
pub struct Position {
    /// x coordinate.
    pub x: Decimal,
    /// y coordinate.
    pub y: Decimal,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            x: dec!(256).into(),
            y: dec!(192).into(),
        }
    }
}

#[derive(Debug)]
/// Error with line index.
pub struct Error<E> {
    /// Line index of the error.
    line_index: usize,
    /// The error.
    error: E,
}

impl<E: std::error::Error + 'static> std::error::Error for Error<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

impl<E> Error<E> {
    /// Returns a pretty error message with the affected line and the error.
    /// - Expensive than showing line number and error with the `Display` trait, as this iterates over the lines of the file input string.
    /// - Slightly cheaper alternative is to use the `Display` trait.
    pub fn display_error_with_line(&self, file_input: &str) -> String
    where
        E: std::fmt::Display,
    {
        let line = file_input.lines().nth(self.line_index).unwrap_or_default();

        format!("Line {}: {}, {}", self.line_index + 1, line, self.error)
    }

    /// Creates a new `Error` instance with the given line index and error.
    /// - If you have a higher error that `E` needs to convert from, use `new_into` instead.
    pub fn new(error: E, line_index: usize) -> Self {
        Self { line_index, error }
    }

    /// Creates a new `Error` instance with the given line index and error.
    /// - Shorthand for `Error::new(error.into(), line_index)`.
    pub fn new_into<E2>(error: E, line_index: usize) -> Error<E2>
    where
        E2: From<E>,
    {
        Error {
            line_index,
            error: error.into(),
        }
    }

    /// Creates a new `Error` instance with the given line index and error.
    /// - For use when you have some Result<T, E> and want to convert it to a `Error`.
    pub fn new_from_result<T>(result: Result<T, E>, line_index: usize) -> Result<T, Error<E>> {
        result.map_err(|err| Error {
            line_index,
            error: err,
        })
    }

    /// Creates a new `Error` instance with the given line index and error.
    /// - For use when you have some Result<T, E> and want to convert it into `Error<E2>`.
    pub fn new_from_result_into<T, E2>(
        result: Result<T, E>,
        line_index: usize,
    ) -> Result<T, Error<E2>>
    where
        E2: From<E>,
    {
        result.map_err(|err| Error {
            line_index,
            error: err.into(),
        })
    }

    /// Uses `Into` to convert the inner error into `E2`.
    pub fn error_into<E2>(self) -> Error<E2>
    where
        E2: From<E>,
    {
        Error {
            line_index: self.line_index,
            error: self.error.into(),
        }
    }

    /// `error_into` function for `Result` types.
    pub fn error_result_into<T, E2>(result: Result<T, Error<E>>) -> Result<T, Error<E2>>
    where
        E2: From<E>,
    {
        result.map_err(|err| Error {
            line_index: err.line_index,
            error: err.error.into(),
        })
    }

    /// Increases `Error`'s processing line using the `Result<_, Error<E>>` type.
    /// - This will also convert the inner error into `E2`.
    /// - For use when you have something return `Result<_, Error<E>>` and want to convert it to `Result<_, Error<E2>>`.
    pub fn processing_line<T, E2>(
        result: Result<T, Error<E>>,
        line_index: usize,
    ) -> Result<T, Error<E2>>
    where
        E2: From<E>,
    {
        result.map_err(|err| Error {
            line_index: err.line_index + line_index,
            error: err.error.into(),
        })
    }

    /// Get the error's line index.
    pub fn line_index(&self) -> usize {
        self.line_index
    }

    /// Get a reference to the error's error.
    pub fn error(&self) -> &E {
        &self.error
    }
}

impl<E> Display for Error<E>
where
    E: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Line {}, {}", self.line_index + 1, self.error)
    }
}

impl<E> From<E> for Error<E> {
    /// Error from parsing a single line can be converted to a `ParseError` directly.
    fn from(error: E) -> Self {
        Self {
            line_index: 0,
            error,
        }
    }
}

/// Contains `to_string` that provides version specific output.
pub trait VersionedToString {
    /// Returns a string representation of the object.
    /// - The output is version specific.
    /// - Returns Some if the version is supported, otherwise None.
    fn to_string(&self, version: Version) -> Option<String>;
}

/// Contains `from_str` that provides version specific parsing.
pub trait VersionedFromStr: Sized {
    type Err;

    /// Parses a string into an object.
    /// - The output of the object is version specific.
    /// - Returns Some if the version is supported, otherwise None.
    fn from_str(s: &str, version: Version) -> std::result::Result<Option<Self>, Self::Err>;
}

/// Contains `default` that provides version specific default values.
pub trait VersionedDefault: Sized {
    /// Returns a default value for the object.
    /// - The output is version specific.
    /// - Returns Some if the version is supported, otherwise None.
    fn default(version: Version) -> Option<Self>;
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
/// File path type that is used in most of the crate.
pub struct FilePath(PathBuf);

impl FilePath {
    pub fn get(&self) -> &Path {
        &self.0
    }

    pub fn set<P>(&mut self, path: P)
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref().to_owned();

        self.0 = path;
    }
}

impl VersionedToString for FilePath {
    /// Returns a string representation of the file path.
    /// - It will contain quotes if the path contains spaces.
    fn to_string(&self, _: Version) -> Option<String> {
        let quotes = {
            let path = self.0.to_string_lossy();

            path.contains(' ') && !(path.starts_with('"') && path.ends_with('"'))
        };
        let path = self.0.display();

        let path = if quotes {
            format!("\"{path}\"")
        } else {
            path.to_string()
        };

        Some(path)
    }
}

impl<P: AsRef<Path>> From<P> for FilePath {
    fn from(path: P) -> Self {
        let path = path.as_ref().to_owned();

        FilePath(path)
    }
}

#[derive(Debug, Error)]
#[error("Invalid repr value")]
/// Error when the repr value is invalid.
/// Used for [`VersionedFromRepr`];
pub struct InvalidRepr;

/// Contains `from_repr` that provides version specific parsing of a value.
pub trait VersionedFromRepr: Sized {
    /// Creates an instance of `Self` from a value representation.
    /// - Will return `Err` if the representation is invalid.
    /// - Will return `Ok(None)` if the representation is valid but the version doesn't use that variant or `Self` entirely.
    fn from_repr(repr: usize, version: Version) -> Result<Option<Self>, InvalidRepr>;
}

/// Contains `from` that provides version specific type conversion.
pub trait VersionedFrom<T>: Sized {
    fn from(value: T, version: Version) -> Option<Self>;
}

/// Contains `try_from` that provides version specific type conversion.
pub trait VersionedTryFrom<T>: Sized {
    type Error;

    fn try_from(value: T, version: Version) -> Result<Option<Self>, Self::Error>;
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
/// A wrapper around `rust_decimal::Decimal` that can be either a `Decimal` or a `String`.
/// - If parsed from a string, it will be a `Decimal` if the string is a valid decimal, otherwise it will be a `String`.
pub struct Decimal(Either<rust_decimal::Decimal, String>);

impl Decimal {
    pub fn new(value: rust_decimal::Decimal) -> Self {
        Self(Either::Left(value))
    }

    pub fn new_from_str(value: &str) -> Self {
        Self(Either::Right(value.to_owned()))
    }

    /// Tries to convert the `Right` value to a `rust_decimal::Decimal`.
    pub fn try_make_decimal(&mut self) -> Result<(), rust_decimal::Error> {
        if let Either::Right(value) = &mut self.0 {
            let value = rust_decimal::Decimal::from_str(value)?;

            self.0 = Either::Left(value);
        }

        Ok(())
    }

    pub fn get(&self) -> &Either<rust_decimal::Decimal, String> {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut Either<rust_decimal::Decimal, String> {
        &mut self.0
    }
}

impl FromStr for Decimal {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Decimal::from(s))
    }
}

impl Display for Decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<rust_decimal::Decimal> for Decimal {
    fn from(value: rust_decimal::Decimal) -> Self {
        Self(Either::Left(value))
    }
}

impl From<i32> for Decimal {
    fn from(value: i32) -> Self {
        Self(Either::Left(rust_decimal::Decimal::from(value)))
    }
}

impl From<&str> for Decimal {
    fn from(value: &str) -> Self {
        let value = match value.parse() {
            Ok(value) => Either::Left(value),
            Err(_) => Either::Right(value.to_owned()),
        };

        Self(value)
    }
}

impl Default for Decimal {
    fn default() -> Self {
        Self(Either::Left(rust_decimal::Decimal::default()))
    }
}
