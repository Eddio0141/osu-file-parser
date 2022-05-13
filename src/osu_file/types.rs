use std::fmt::{Debug, Display};

/// Definition of the `Integer` type.
pub type Integer = i32;

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

// TODO way of combining the Error types together as well as line_number being calculated
pub struct Error<E> {
    /// Line number of the error.
    pub line_number: usize,
    /// The error.
    pub error: E,
}

impl<E> Debug for Error<E>
where
    E: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.error.fmt(f)
    }
}

impl<E> Error<E> {
    /// Shows a pretty error message with the affected line and the error.
    /// - Expensive than showing line number and error with the `Display` trait, as this iterates over the lines of the file input string.
    pub fn display_error_with_line(
        &self,
        f: &mut std::fmt::Formatter,
        file_input: &str,
    ) -> std::fmt::Result
    where
        E: std::fmt::Display,
    {
        let line = file_input.lines().nth(self.line_number).unwrap_or_default();

        writeln!(f, "Line {}: {}", self.line_number + 1, line)?;
        writeln!(f, "{}", self.error)
    }

    /// Combines other `Error` with a higher level error and line number of where it was being processed.
    pub fn combine<E2>(self, line_number: usize) -> Error<E2>
    where
        E2: From<E>,
    {
        Error {
            line_number: self.line_number + line_number,
            error: E2::from(self.error),
        }
    }
}

impl<E> Display for Error<E>
where
    E: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Line {}", self.line_number + 1)?;
        writeln!(f, "{}", self.error)
    }
}

impl<E> From<E> for Error<E> {
    /// Error from parsing a single line can be converted to a `ParseError` directly.
    fn from(error: E) -> Self {
        Self {
            line_number: 0,
            error,
        }
    }
}
