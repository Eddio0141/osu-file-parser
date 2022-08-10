#[cfg(test)]
mod tests;

mod helper;
pub mod osu_file;
pub use osu_file::*;
mod parsers;

/// Trims the given osu file string into something that can be tested for equality.
/// - Ignores all empty lines and key value pair's spacing between the key and comma.
/// - Deletes `\u{feff}` characters.
pub fn osu_str_trimmer(s: &str) -> String {
    s.replace("\r\n", "\n")
        .lines()
        .filter_map(|s| {
            // remove the weird \u{feff} characters
            let s = s.trim().replace('\u{feff}', "");

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
}

/// Tests equality of two osu file strings.
pub fn osu_str_eq<L: AsRef<str>, R: AsRef<str>>(left: L, right: R) -> bool {
    let left = osu_str_trimmer(left.as_ref());
    let right = osu_str_trimmer(right.as_ref());

    left == right
}

/// Asserts that two osu file strings are equal.
pub fn assert_eq_osu_str<L: AsRef<str>, R: AsRef<str>>(left: L, right: R) {
    let left = osu_str_trimmer(left.as_ref());
    let right = osu_str_trimmer(right.as_ref());

    pretty_assertions::assert_eq!(left, right)
}
