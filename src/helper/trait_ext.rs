use std::fmt::Display;

pub trait MapStringNewLine {
    fn map_string_new_line<T>(&mut self) -> String
    where
        Self: Iterator<Item = T>,
        T: Display,
    {
        self.into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

pub trait MapOptStringNewLine {
    fn map_string_new_line(&mut self) -> String
    where
        Self: Iterator<Item = Option<String>>,
    {
        self.into_iter().flatten().collect::<Vec<_>>().join("\n")
    }
}

impl<T: Display, I: Iterator<Item = T>> MapStringNewLine for I {}
impl<I: Iterator<Item = Option<String>>> MapOptStringNewLine for I {}
