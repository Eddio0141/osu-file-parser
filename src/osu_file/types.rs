use std::fmt::{Debug, Display};

/// Definition of the `Integer` type.
pub type Integer = i32;

pub const LATEST_VERSION: u8 = 14;
pub const MIN_VERSION: u8 = 3;

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

pub trait Version: Sized {
    type ParseError;

    fn from_str_v3(s: &str) -> std::result::Result<Option<Self>, Self::ParseError>
    where
        Self: Sized;

    fn to_string_v3(&self) -> String;

    fn from_str_v4(s: &str) -> std::result::Result<Option<Self>, Self::ParseError>
    where
        Self: Sized,
    {
        Self::from_str_v3(s)
    }

    fn to_string_v4(&self) -> String {
        self.to_string_v3()
    }

    fn from_str_v5(s: &str) -> std::result::Result<Option<Self>, Self::ParseError>
    where
        Self: Sized,
    {
        Self::from_str_v4(s)
    }

    fn to_string_v5(&self) -> String {
        self.to_string_v4()
    }

    fn from_str_v6(s: &str) -> std::result::Result<Option<Self>, Self::ParseError>
    where
        Self: Sized,
    {
        Self::from_str_v5(s)
    }

    fn to_string_v6(&self) -> String {
        self.to_string_v5()
    }

    fn from_str_v7(s: &str) -> std::result::Result<Option<Self>, Self::ParseError>
    where
        Self: Sized,
    {
        Self::from_str_v6(s)
    }

    fn to_string_v7(&self) -> String {
        self.to_string_v6()
    }

    fn from_str_v8(s: &str) -> std::result::Result<Option<Self>, Self::ParseError>
    where
        Self: Sized,
    {
        Self::from_str_v7(s)
    }

    fn to_string_v8(&self) -> String {
        self.to_string_v7()
    }

    fn from_str_v9(s: &str) -> std::result::Result<Option<Self>, Self::ParseError>
    where
        Self: Sized,
    {
        Self::from_str_v8(s)
    }

    fn to_string_v9(&self) -> String {
        self.to_string_v8()
    }

    fn from_str_v10(s: &str) -> std::result::Result<Option<Self>, Self::ParseError>
    where
        Self: Sized,
    {
        Self::from_str_v9(s)
    }

    fn to_string_v10(&self) -> String {
        self.to_string_v9()
    }

    fn from_str_v11(s: &str) -> std::result::Result<Option<Self>, Self::ParseError>
    where
        Self: Sized,
    {
        Self::from_str_v10(s)
    }

    fn to_string_v11(&self) -> String {
        self.to_string_v10()
    }

    fn from_str_v12(s: &str) -> std::result::Result<Option<Self>, Self::ParseError>
    where
        Self: Sized,
    {
        Self::from_str_v11(s)
    }

    fn to_string_v12(&self) -> String {
        self.to_string_v11()
    }

    fn from_str_v13(s: &str) -> std::result::Result<Option<Self>, Self::ParseError>
    where
        Self: Sized,
    {
        Self::from_str_v12(s)
    }

    fn to_string_v13(&self) -> String {
        self.to_string_v12()
    }

    fn from_str_v14(s: &str) -> std::result::Result<Option<Self>, Self::ParseError>
    where
        Self: Sized,
    {
        Self::from_str_v13(s)
    }

    fn to_string_v14(&self) -> String {
        self.to_string_v13()
    }
}
