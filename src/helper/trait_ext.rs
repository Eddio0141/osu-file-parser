pub trait MapOptStringNewLine {
    fn map_string_new_line(&mut self) -> String
    where
        Self: Iterator<Item = Option<String>>,
    {
        self.into_iter().flatten().collect::<Vec<_>>().join("\n")
    }
}

impl<I: Iterator<Item = Option<String>>> MapOptStringNewLine for I {}
