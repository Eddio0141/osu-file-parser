use crate::osu_file::OsuFile;
#[cfg(test)]
use pretty_assertions::assert_eq;

// TODO idea: instead of error containing string for affected section, store whole line for user to see, maybe with line number
// extension of error type to use the line number and input string to show the user what went wrong
#[test]
fn all_files_parse_back() {
    // let v3 = include_str!("./files/v3.osu");
    // let v4 = include_str!("./files/v4.osu");
    let v5 = include_str!("./files/v5.osu");
    let v14 = include_str!("./files/v14.osu");

    let files = vec![v5, v14];

    for file in files {
        let file = file.replace("\r\n", "\n");

        let osu_file = file.parse::<OsuFile>().unwrap();
        let osu_file_str = osu_file.to_string();

        assert_eq!(file, osu_file_str);
    }
}
