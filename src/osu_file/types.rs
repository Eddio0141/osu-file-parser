use std::{
    fmt::{Debug, Display},
    path::{Path, PathBuf},
};

use thiserror::Error;

/// Definition of the `Integer` type.
pub type Integer = i32;

pub const LATEST_VERSION: Version = 14;
pub const MIN_VERSION: Version = 3;

pub type Version = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// The position of something in `osu!pixels` with the `x` `y` form.
pub struct Position {
    /// x coordinate.
    pub x: Integer,
    /// y coordinate.
    pub y: Integer,
}

impl Default for Position {
    fn default() -> Self {
        Self { x: 256, y: 192 }
    }
}

#[derive(Debug)]
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

pub trait VersionedToString {
    fn to_string(&self, version: Version) -> Option<String>;
}

pub trait VersionedFromStr: Sized {
    type Err;

    fn from_str(s: &str, version: Version) -> std::result::Result<Option<Self>, Self::Err>;
}

pub trait VersionedDefault: Sized {
    fn default(version: Version) -> Option<Self>;
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
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
pub struct InvalidRepr;

pub trait VersionedFromRepr: Sized {
    /// Creates an instance of `Self` from a value representation.
    /// - Will return `Err` if the representation is invalid.
    /// - Will return `Ok(None)` if the representation is valid but the version doesn't use that variant or `Self` entirely.
    fn from_repr(repr: usize, version: Version) -> Result<Option<Self>, InvalidRepr>;
}
