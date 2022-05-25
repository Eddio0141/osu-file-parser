use crate::osu_file::OsuFile;
#[cfg(test)]
use pretty_assertions::assert_eq;

#[test]
fn v3_file() {
    let v3 = include_str!("./files/v3.osu").replace("\r\n", "\n");
    let osu_file = v3.parse::<OsuFile>().unwrap();

    assert_eq!(v3, osu_file.to_string());
}

#[test]
fn v4_file() {
    let v4 = include_str!("./files/v4.osu").replace("\r\n", "\n");
    let osu_file = v4.parse::<OsuFile>().unwrap();

    assert_eq!(v4, osu_file.to_string());
}

#[test]
fn v5_file() {
    let v5 = include_str!("./files/v5.osu").replace("\r\n", "\n");
    let osu_file = v5.parse::<OsuFile>().unwrap();

    assert_eq!(v5, osu_file.to_string());
}

#[test]
fn v6_file() {
    let v6 = include_str!("./files/v6.osu").replace("\r\n", "\n");
    let osu_file = v6.parse::<OsuFile>().unwrap();

    assert_eq!(v6, osu_file.to_string());
}

#[test]
fn v7_file() {
    let v7 = include_str!("./files/v7.osu").replace("\r\n", "\n");
    let osu_file = v7.parse::<OsuFile>().unwrap();

    assert_eq!(v7, osu_file.to_string());
}

#[test]
fn v8_file() {
    let v8 = include_str!("./files/v8.osu").replace("\r\n", "\n");
    let osu_file = v8.parse::<OsuFile>().unwrap();

    assert_eq!(v8, osu_file.to_string());
}

#[test]
fn v9_file() {
    let v9 = include_str!("./files/v9.osu").replace("\r\n", "\n");
    let osu_file = v9.parse::<OsuFile>().unwrap();

    assert_eq!(v9, osu_file.to_string());
}

#[test]
fn v10_file() {
    let v10 = include_str!("./files/v10.osu").replace("\r\n", "\n");
    let osu_file = v10.parse::<OsuFile>().unwrap();

    assert_eq!(v10, osu_file.to_string());
}

#[test]
fn v11_file() {
    let v11 = include_str!("./files/v11.osu").replace("\r\n", "\n");
    let osu_file = v11.parse::<OsuFile>().unwrap();

    assert_eq!(v11, osu_file.to_string());
}

#[test]
fn v12_file() {
    let v12 = include_str!("./files/v12.osu").replace("\r\n", "\n");
    let osu_file = v12.parse::<OsuFile>().unwrap();

    assert_eq!(v12, osu_file.to_string());
}

#[test]
fn v13_file() {
    let v13 = include_str!("./files/v13.osu").replace("\r\n", "\n");
    let osu_file = v13.parse::<OsuFile>().unwrap();

    assert_eq!(v13, osu_file.to_string());
}

#[test]
fn v14_file() {
    let v14 = include_str!("./files/v14.osu").replace("\r\n", "\n");
    let osu_file = v14.parse::<OsuFile>().unwrap();

    assert_eq!(v14, osu_file.to_string());
}
