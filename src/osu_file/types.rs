use std::fmt::{Debug, Display};

/// Definition of the `Integer` type.
pub type Integer = i32;

pub const LATEST_VERSION: u8 = 14;
pub const MIN_VERSION: u8 = 3;

pub const SECTION_DELIMITER: &str = ":";

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
    pub line_index: usize,
    /// The error.
    pub error: E,
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

    pub fn from_err_with_line(error: E, line_number: usize) -> Self {
        Self {
            line_index: line_number,
            error,
        }
    }

    /// Combines other `Error` with a higher level error and line number of where it was being processed.
    pub fn combine<E2>(self, line_number: usize) -> Error<E2>
    where
        E2: From<E>,
    {
        Error {
            line_index: self.line_index + line_number,
            error: E2::from(self.error),
        }
    }

    pub fn combine_result<T, E2>(
        inner: Result<T, Error<E>>,
        line_number: usize,
    ) -> Result<T, Error<E2>>
    where
        E2: From<E>,
    {
        match inner {
            Ok(ok) => Ok(ok),
            Err(err) => Err(Error {
                line_index: line_number + err.line_index,
                error: err.error.into(),
            }),
        }
    }

    pub fn from_combine<E2>(inner_error: E2, line_number: usize) -> Error<E>
    where
        E: From<E2>,
    {
        Error {
            line_index: line_number,
            error: inner_error.into(),
        }
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
