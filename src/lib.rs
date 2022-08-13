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
    let mut builder = Vec::new();
    let mut section_values_inner = Vec::new();
    let mut in_sections = false;
    let mut first_line = true;
    let mut section_coloned = true;

    let section_values_sort_and_push =
        |section_values_inner: &mut Vec<String>, builder: &mut Vec<String>| {
            section_values_inner.sort();
            builder.append(section_values_inner);
        };

    for line in s.lines() {
        let line = line.trim().replace('\u{feff}', "");

        if line.is_empty() {
            continue;
        }

        if !in_sections {
            if line.starts_with('[') && line.ends_with(']') {
                in_sections = true;
            } else {
                // data before the sections are irrelevant after first line with file format specifier
                if first_line && line.starts_with("osu file format v") {
                    builder.push(line);
                }

                first_line = false;

                continue;
            }

            builder.push(line);

            continue;
        } else {
            if line.starts_with('[') && line.ends_with(']') {
                section_values_sort_and_push(&mut section_values_inner, &mut builder);
                builder.push(line);
                section_coloned = true;

                continue;
            }

            if line.contains(':') {
                let mut header = true;
                let mut value = false;

                let line = line
                    .chars()
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
                    .collect::<String>();

                if section_coloned {
                    section_values_inner.push(line);
                } else {
                    builder.push(line);
                }
            } else {
                section_coloned = false;
                builder.append(&mut section_values_inner);
                builder.push(line);
            }
        }
    }

    section_values_sort_and_push(&mut section_values_inner, &mut builder);

    builder.join("\n")
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
