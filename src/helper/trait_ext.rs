use std::fmt::Display;

use crate::osu_file::VersionedToString;

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

pub trait MapStringNewLineVersion {
    fn map_string_new_line<T>(&mut self, version: usize) -> String
    where
        Self: Iterator<Item = T>,
        T: VersionedToString,
    {
        self.into_iter()
            .filter_map(|v| v.to_string(version))
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
impl<T: VersionedToString, I: Iterator<Item = T>> MapStringNewLineVersion for I {}
impl<I: Iterator<Item = Option<String>>> MapOptStringNewLine for I {}
