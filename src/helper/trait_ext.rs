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
    fn map_string_new_line<T>(&mut self, version: usize) -> Option<String>
    where
        Self: Iterator<Item = T>,
        T: VersionedToString,
    {
        let v = self
            .into_iter()
            .map(|v| v.to_string(version))
            .collect::<Option<Vec<_>>>();

        v.map(|v| v.join("\n"))
    }
}

pub trait FromIterator<A> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = A>;
}

impl<T> FromIterator<Option<T>> for Option<Vec<T>>
where
    T: Clone,
{
    fn from_iter<I: IntoIterator<Item = Option<T>>>(iter: I) -> Self {
        let s = iter.into_iter().collect::<Vec<_>>();
        if let Some(v) = s.get(0) {
            if v.is_some() {
                Some(s.into_iter().map(|v| v.unwrap()).collect())
            } else {
                None
            }
        } else {
            Some(Vec::new())
        }
    }
}

impl<T: Display> MapStringNewLine for std::slice::Iter<'_, T> {}
impl<T: VersionedToString> MapStringNewLineVersion for std::slice::Iter<'_, T> {}
impl<T: VersionedToString> MapStringNewLineVersion for std::vec::IntoIter<T> {}
