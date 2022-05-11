use crate::osu_file::OsuFile;

#[test]
fn all_files_parse_back() {
    let v3 = include_str!("./files/v3.osu");
    let v4 = include_str!("./files/v4.osu");

    let files = vec![v3, v4];

    for file in files {
        let osu_file = file.parse::<OsuFile>().unwrap();
        let osu_file_str = osu_file.to_string();

        assert_eq!(file, osu_file_str);
    }
}
