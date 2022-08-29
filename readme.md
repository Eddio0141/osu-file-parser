# osu-file-parser
---
[![crates.io](https://img.shields.io/crates/d/osu-file-parser)](https://crates.io/crates/osu-file-parser)
[![Documentation](https://img.shields.io/docsrs/osu-file-parser)](https://docs.rs/osu-file-parser)

A crate to parse an osu! beatmap file.

# How to use

```rust
use osu_file_parser::*;

let osu_file_str = include_str!("./tests/osu_files/files/acid_rain.osu");
// parse the .osu file
let mut osu_file = osu_file_str.parse::<OsuFile>().unwrap();

let osb_str = include_str!("./tests/osu_files/files/acid_rain.osb");
// .osb file can also be parsed and appended to the `OsuFile` instance
osu_file.append_osb(osb_str).unwrap();

// you can use `assert_eq_osu_str` to assert that the parsed .osu file is equal to the original .osu file
assert_eq_osu_str(&osu_file.to_string(), osu_file_str);
assert_eq_osu_str(&osu_file.osb_to_string().unwrap(), osb_str);
```

# General information

## Alternative traits
- Most of the types in the crate uses the `VersionedToString`, `VersionedFromStr` and `VersionedDefault` traits as replacements for the `Display`, `FromStr` and `Default` traits.
- Those traits take an extra `version` parameter to choose what version output to use.
- If the type doesn't exist in certain versions, the output will be `None`.

## Errors
- Structs that takes lines of string as input can return errors containing information of where the error occurred and what the error was.
- The error type is wrapped in `Error` in those cases.
- `Error` has methods that tells you where the error happened in the input string and what the error was.
