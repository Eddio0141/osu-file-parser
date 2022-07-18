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

#[test]
fn aspire_osb() {
    let osb1 = include_str!("./files/aspire1_osb.osb").replace("\r\n", "\n");
    let osb2 = include_str!("./files/aspire2_osb.osb").replace("\r\n", "\n");
    let osb3 = include_str!("./files/aspire3_osb.osb").replace("\r\n", "\n");

    let mut osu_file = OsuFile::default(14);
    osu_file.append_osb(&osb1).unwrap();
    assert_eq!(osb1, osu_file.osb_to_string().unwrap());

    let mut osu_file = OsuFile::default(14);
    osu_file.append_osb(&osb2).unwrap();
    assert_eq!(osb2, osu_file.osb_to_string().unwrap());

    let mut osu_file = OsuFile::default(14);
    osu_file.append_osb(&osb3).unwrap();
    assert_eq!(osb3, osu_file.osb_to_string().unwrap());
}

#[test]
fn aspire() {
    let aspire1 = include_str!("./files/aspire1.osu").replace("\r\n", "\n");
    let aspire2 = include_str!("./files/aspire2.osu").replace("\r\n", "\n");
    let aspire3 = include_str!("./files/aspire3.osu").replace("\r\n", "\n");
    let aspire4 = include_str!("./files/aspire4.osu").replace("\r\n", "\n");
    let aspire5 = include_str!("./files/aspire5.osu").replace("\r\n", "\n");
    let aspire6 = include_str!("./files/aspire6.osu").replace("\r\n", "\n");
    let aspire7 = include_str!("./files/aspire7.osu").replace("\r\n", "\n");
    let aspire8 = include_str!("./files/aspire8.osu").replace("\r\n", "\n");
    let aspire9 = include_str!("./files/aspire9.osu").replace("\r\n", "\n");
    let aspire10 = include_str!("./files/aspire10.osu").replace("\r\n", "\n");
    let aspire11 = include_str!("./files/aspire11.osu").replace("\r\n", "\n");
    let aspire12 = include_str!("./files/aspire12.osu").replace("\r\n", "\n");
    let aspire13 = include_str!("./files/aspire13.osu").replace("\r\n", "\n");
    let aspire14 = include_str!("./files/aspire14.osu").replace("\r\n", "\n");
    let aspire15 = include_str!("./files/aspire15.osu").replace("\r\n", "\n");
    let aspire16 = include_str!("./files/aspire16.osu").replace("\r\n", "\n");
    let aspire17 = include_str!("./files/aspire17.osu").replace("\r\n", "\n");
    let aspire18 = include_str!("./files/aspire18.osu").replace("\r\n", "\n");
    let aspire19 = include_str!("./files/aspire19.osu").replace("\r\n", "\n");
    let aspire20 = include_str!("./files/aspire20.osu").replace("\r\n", "\n");
    let aspire21 = include_str!("./files/aspire21.osu").replace("\r\n", "\n");
    let aspire22 = include_str!("./files/aspire22.osu").replace("\r\n", "\n");
    let aspire23 = include_str!("./files/aspire23.osu").replace("\r\n", "\n");
    let aspire24 = include_str!("./files/aspire24.osu").replace("\r\n", "\n");
    let aspire25 = include_str!("./files/aspire25.osu").replace("\r\n", "\n");
    let aspire26 = include_str!("./files/aspire26.osu").replace("\r\n", "\n");
    let aspire27 = include_str!("./files/aspire27.osu").replace("\r\n", "\n");
    let aspire28 = include_str!("./files/aspire28.osu").replace("\r\n", "\n");
    let aspire29 = include_str!("./files/aspire29.osu").replace("\r\n", "\n");

    let osu_file = aspire1.parse::<OsuFile>().unwrap();
    assert_eq!(aspire1, osu_file.to_string());

    let osu_file = aspire2.parse::<OsuFile>().unwrap();
    assert_eq!(aspire2, osu_file.to_string());

    let osu_file = aspire3.parse::<OsuFile>().unwrap();
    assert_eq!(aspire3, osu_file.to_string());

    let osu_file = aspire4.parse::<OsuFile>().unwrap();
    assert_eq!(aspire4, osu_file.to_string());

    let osu_file = aspire5.parse::<OsuFile>().unwrap();
    assert_eq!(aspire5, osu_file.to_string());

    let osu_file = aspire6.parse::<OsuFile>().unwrap();
    assert_eq!(aspire6, osu_file.to_string());

    let osu_file = aspire7.parse::<OsuFile>().unwrap();
    assert_eq!(aspire7, osu_file.to_string());

    let osu_file = aspire8.parse::<OsuFile>().unwrap();
    assert_eq!(aspire8, osu_file.to_string());

    let osu_file = aspire9.parse::<OsuFile>().unwrap();
    assert_eq!(aspire9, osu_file.to_string());

    let osu_file = aspire10.parse::<OsuFile>().unwrap();
    assert_eq!(aspire10, osu_file.to_string());

    let osu_file = aspire11.parse::<OsuFile>().unwrap();
    assert_eq!(aspire11, osu_file.to_string());

    let osu_file = aspire12.parse::<OsuFile>().unwrap();
    assert_eq!(aspire12, osu_file.to_string());

    let osu_file = aspire13.parse::<OsuFile>().unwrap();
    assert_eq!(aspire13, osu_file.to_string());

    let osu_file = aspire14.parse::<OsuFile>().unwrap();
    assert_eq!(aspire14, osu_file.to_string());

    let osu_file = aspire15.parse::<OsuFile>().unwrap();
    assert_eq!(aspire15, osu_file.to_string());

    let osu_file = aspire16.parse::<OsuFile>().unwrap();
    assert_eq!(aspire16, osu_file.to_string());

    let osu_file = aspire17.parse::<OsuFile>().unwrap();
    assert_eq!(aspire17, osu_file.to_string());

    let osu_file = aspire18.parse::<OsuFile>().unwrap();
    assert_eq!(aspire18, osu_file.to_string());

    let osu_file = aspire19.parse::<OsuFile>().unwrap();
    assert_eq!(aspire19, osu_file.to_string());

    let osu_file = aspire20.parse::<OsuFile>().unwrap();
    assert_eq!(aspire20, osu_file.to_string());

    let osu_file = aspire21.parse::<OsuFile>().unwrap();
    assert_eq!(aspire21, osu_file.to_string());

    let osu_file = aspire22.parse::<OsuFile>().unwrap();
    assert_eq!(aspire22, osu_file.to_string());

    let osu_file = aspire23.parse::<OsuFile>().unwrap();
    assert_eq!(aspire23, osu_file.to_string());

    let osu_file = aspire24.parse::<OsuFile>().unwrap();
    assert_eq!(aspire24, osu_file.to_string());

    let osu_file = aspire25.parse::<OsuFile>().unwrap();
    assert_eq!(aspire25, osu_file.to_string());

    let osu_file = aspire26.parse::<OsuFile>().unwrap();
    assert_eq!(aspire26, osu_file.to_string());

    let osu_file = aspire27.parse::<OsuFile>().unwrap();
    assert_eq!(aspire27, osu_file.to_string());

    let osu_file = aspire28.parse::<OsuFile>().unwrap();
    assert_eq!(aspire28, osu_file.to_string());

    let osu_file = aspire29.parse::<OsuFile>().unwrap();
    assert_eq!(aspire29, osu_file.to_string());
}
