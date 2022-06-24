use crate::osu_file::OsuFile;
use pretty_assertions::assert_eq;

#[test]
fn v3() {
    let v3 = include_str!("./files/v3.osu").replace("\r\n", "\n");
    let osu_file = v3.parse::<OsuFile>().unwrap();

    assert_eq!(v3, osu_file.to_string());
}

#[test]
fn v3_to_v14() {
    let v3 = include_str!("./files/v3.osu").replace("\r\n", "\n");
    let v3_v14 = include_str!("./files/v3_v14.osu").replace("\r\n", "\n");
    let mut osu_file = v3.parse::<OsuFile>().unwrap();
    osu_file.version = 14;

    assert_eq!(v3_v14, osu_file.to_string());
}

#[test]
fn v4() {
    let v4 = include_str!("./files/v4.osu").replace("\r\n", "\n");
    let osu_file = v4.parse::<OsuFile>().unwrap();

    assert_eq!(v4, osu_file.to_string());
}

#[test]
fn v5() {
    let v5 = include_str!("./files/v5.osu").replace("\r\n", "\n");
    let osu_file = v5.parse::<OsuFile>().unwrap();

    assert_eq!(v5, osu_file.to_string());
}

#[test]
fn v6() {
    let v6 = include_str!("./files/v6.osu").replace("\r\n", "\n");
    let osu_file = v6.parse::<OsuFile>().unwrap();

    assert_eq!(v6, osu_file.to_string());
}

#[test]
fn v7() {
    let v7 = include_str!("./files/v7.osu").replace("\r\n", "\n");
    let osu_file = v7.parse::<OsuFile>().unwrap();

    assert_eq!(v7, osu_file.to_string());
}

#[test]
fn v8() {
    let v8 = include_str!("./files/v8.osu").replace("\r\n", "\n");
    let osu_file = v8.parse::<OsuFile>().unwrap();

    assert_eq!(v8, osu_file.to_string());
}

#[test]
fn v9() {
    let v9 = include_str!("./files/v9.osu").replace("\r\n", "\n");
    let osu_file = v9.parse::<OsuFile>().unwrap();

    assert_eq!(v9, osu_file.to_string());
}

#[test]
fn v10() {
    let v10 = include_str!("./files/v10.osu").replace("\r\n", "\n");
    let osu_file = v10.parse::<OsuFile>().unwrap();

    assert_eq!(v10, osu_file.to_string());
}

#[test]
fn v11() {
    let v11 = include_str!("./files/v11.osu").replace("\r\n", "\n");
    let osu_file = v11.parse::<OsuFile>().unwrap();

    assert_eq!(v11, osu_file.to_string());
}

#[test]
fn v12() {
    let v12 = include_str!("./files/v12.osu").replace("\r\n", "\n");
    let osu_file = v12.parse::<OsuFile>().unwrap();

    assert_eq!(v12, osu_file.to_string());
}

#[test]
fn v13() {
    let v13 = include_str!("./files/v13.osu").replace("\r\n", "\n");
    let osu_file = v13.parse::<OsuFile>().unwrap();

    assert_eq!(v13, osu_file.to_string());
}

#[test]
fn v14() {
    let v14 = include_str!("./files/v14.osu").replace("\r\n", "\n");
    let osu_file = v14.parse::<OsuFile>().unwrap();

    assert_eq!(v14, osu_file.to_string());
}

#[test]
fn acid_rain() {
    let acid_rain = include_str!("./files/acid_rain.osu").replace("\r\n", "\n");
    let acid_rain_osb = include_str!("./files/acid_rain.osb").replace("\r\n", "\n");

    let mut osu_file = acid_rain.parse::<OsuFile>().unwrap();
    assert_eq!(acid_rain, osu_file.to_string());

    osu_file.append_osb(&acid_rain_osb).unwrap();
    assert_eq!(acid_rain_osb, osu_file.osb_to_string().unwrap());
}
