#[cfg(test)]
mod tests;

mod helper;
pub mod osu_file;
pub use osu_file::*;
mod parsers;

/// Function to test equality of two osu file strings.
/// - Ignores all empty lines and key value pair's spacing between the key and comma.
/// - Deletes `\u{feff}` characters.
pub fn assert_eq_osu_str<L: AsRef<str>, R: AsRef<str>>(left: L, right: R) {
    let trimmer = |s: &str| {
        s.lines()
            .filter_map(|s| {
                // remove the weird \u{feff} characters
                let s = s.trim().replace("\u{feff}", "");

                if s.is_empty() {
                    return None;
                }

                if s.contains(':') {
                    let mut header = true;
                    let mut value = false;

                    return Some(
                        s.chars()
                            .filter(|c| {
                                if !header && !value && *c == ':' {
                                    header = false;
                                    value = true;
                                    return true;
                                }

                                if header && c.is_whitespace() {
                                    return false;
                                }

                                true
                            })
                            .collect::<String>(),
                    );
                }

                Some(s)
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    let left = trimmer(left.as_ref());
    let right = trimmer(right.as_ref());

    pretty_assertions::assert_eq!(left, right);
}
