use crate::osu_file::OsuFile;

#[test]
fn all_files_parse_back() {
    // let v3 = include_str!("./files/v3.osu");
    // let v4 = include_str!("./files/v4.osu");
    // let v5 = include_str!("./files/v5.osu");
    let v14 = include_str!("./files/v14.osu");

    let files = vec![v14];

    for file in files {
        let osu_file = file.parse::<OsuFile>().unwrap();
        let osu_file_str = osu_file.to_string();

        assert_eq!(file, osu_file_str);
    }
}
