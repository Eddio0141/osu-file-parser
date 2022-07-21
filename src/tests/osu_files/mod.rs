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
fn v9_spaces() {
    let v9_spaces = include_str!("./files/v9_spaces.osu").replace("\r\n", "\n");
    let osu_file = v9_spaces.parse::<OsuFile>().unwrap();

    assert_eq!(v9_spaces, osu_file.to_string());
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
fn v14_2() {
    let v14_2 = include_str!("./files/v14_2.osu").replace("\r\n", "\n");
    let osu_file = v14_2.parse::<OsuFile>().unwrap();

    assert_eq!(v14_2, osu_file.to_string());
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
fn aspire_osb1() {
    let osb = include_str!("./files/aspire_osb1.osb").replace("\r\n", "\n");
    let mut osu_file = OsuFile::default(14);
    osu_file.append_osb(&osb).unwrap();

    assert_eq!(osb, osu_file.osb_to_string().unwrap());
}

#[test]
fn aspire_osb2() {
    let osb = include_str!("./files/aspire_osb2.osb").replace("\r\n", "\n");
    let mut osu_file = OsuFile::default(14);
    osu_file.append_osb(&osb).unwrap();

    assert_eq!(osb, osu_file.osb_to_string().unwrap());
}

#[test]
fn aspire_osb3() {
    let osb = include_str!("./files/aspire_osb3.osb").replace("\r\n", "\n");
    let mut osu_file = OsuFile::default(14);
    osu_file.append_osb(&osb).unwrap();

    assert_eq!(osb, osu_file.osb_to_string().unwrap());
}

#[test]
fn aspire1() {
    let i = include_str!("./files/aspire1.osu").replace("\r\n", "\n");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq!(i, o.to_string());
}

#[test]
fn aspire2() {
    let i = include_str!("./files/aspire2.osu").replace("\r\n", "\n");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq!(i, o.to_string());
}

#[test]
fn aspire3() {
    let i = include_str!("./files/aspire3.osu").replace("\r\n", "\n");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq!(i, o.to_string());
}

#[test]
fn aspire4() {
    let i = include_str!("./files/aspire4.osu").replace("\r\n", "\n");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq!(i, o.to_string());
}

#[test]
fn aspire5() {
    let i = include_str!("./files/aspire5.osu").replace("\r\n", "\n");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq!(i, o.to_string());
}

#[test]
fn aspire6() {
    let i = include_str!("./files/aspire6.osu").replace("\r\n", "\n");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq!(i, o.to_string());
}

#[test]
fn aspire7() {
    let i = include_str!("./files/aspire7.osu").replace("\r\n", "\n");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq!(i, o.to_string());
}

#[test]
fn aspire8() {
    let i = include_str!("./files/aspire8.osu").replace("\r\n", "\n");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq!(i, o.to_string());
}

#[test]
fn aspire9() {
    let i = include_str!("./files/aspire9.osu").replace("\r\n", "\n");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq!(i, o.to_string());
}

#[test]
fn aspire10() {
    let i = include_str!("./files/aspire10.osu").replace("\r\n", "\n");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq!(i, o.to_string());
}

#[test]
fn aspire11() {
    let i = include_str!("./files/aspire11.osu").replace("\r\n", "\n");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq!(i, o.to_string());
}

#[test]
fn aspire12() {
    let i = include_str!("./files/aspire12.osu").replace("\r\n", "\n");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq!(i, o.to_string());
}

#[test]
fn aspire13() {
    let i = include_str!("./files/aspire13.osu").replace("\r\n", "\n");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq!(i, o.to_string());
}

#[test]
fn aspire14() {
    let i = include_str!("./files/aspire14.osu").replace("\r\n", "\n");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq!(i, o.to_string());
}

#[test]
fn aspire15() {
    let i = include_str!("./files/aspire15.osu").replace("\r\n", "\n");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq!(i, o.to_string());
}

#[test]
fn aspire16() {
    let i = include_str!("./files/aspire16.osu").replace("\r\n", "\n");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq!(i, o.to_string());
}

#[test]
fn aspire17() {
    let i = include_str!("./files/aspire17.osu").replace("\r\n", "\n");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq!(i, o.to_string());
}

#[test]
fn aspire18() {
    let i = include_str!("./files/aspire18.osu").replace("\r\n", "\n");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq!(i, o.to_string());
}

#[test]
fn aspire19() {
    let i = include_str!("./files/aspire19.osu").replace("\r\n", "\n");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq!(i, o.to_string());
}

#[test]
fn aspire20() {
    let i = include_str!("./files/aspire20.osu").replace("\r\n", "\n");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq!(i, o.to_string());
}

#[test]
fn aspire21() {
    let i = include_str!("./files/aspire21.osu").replace("\r\n", "\n");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq!(i, o.to_string());
}

#[test]
fn aspire22() {
    let i = include_str!("./files/aspire22.osu").replace("\r\n", "\n");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq!(i, o.to_string());
}

#[test]
fn aspire23() {
    let i = include_str!("./files/aspire23.osu").replace("\r\n", "\n");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq!(i, o.to_string());
}

#[test]
fn aspire24() {
    let i = include_str!("./files/aspire24.osu").replace("\r\n", "\n");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq!(i, o.to_string());
}

#[test]
fn aspire25() {
    let i = include_str!("./files/aspire25.osu").replace("\r\n", "\n");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq!(i, o.to_string());
}

#[test]
fn aspire26() {
    let i = include_str!("./files/aspire26.osu").replace("\r\n", "\n");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq!(i, o.to_string());
}

#[test]
fn aspire27() {
    let i = include_str!("./files/aspire27.osu").replace("\r\n", "\n");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq!(i, o.to_string());
}

#[test]
fn aspire28() {
    let i = include_str!("./files/aspire28.osu").replace("\r\n", "\n");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq!(i, o.to_string());
}

#[test]
fn aspire29() {
    let i = include_str!("./files/aspire29.osu").replace("\r\n", "\n");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq!(i, o.to_string());
}

#[test]
fn error_line_index_with_leading_ws() {
    let i = include_str!("./files/leading_ws_w_err.osu");
    let o = i.parse::<OsuFile>().unwrap_err();
    assert_eq!(
        o.to_string(),
        "Line 7, Invalid colon set, expected format of `key: value`"
    );
}

#[test]
fn variable_osb() {
    let mut osu = OsuFile::default(14);
    let osb = include_str!("./files/variable.osb").replace("\r\n", "\n");

    osu.append_osb(&osb).unwrap();

    assert_eq!(osu.osb_to_string().unwrap(), osb);
}

#[test]
fn error_line_index_osb() {
    let mut osu = OsuFile::default(14);
    let osb = include_str!("./files/error_line_index.osb");

    let err = osu.append_osb(osb).unwrap_err();

    assert_eq!(err.to_string(), "Line 21, Unknown command type");
}

#[test]
fn error_line_index_variable_osb() {
    let mut osu = OsuFile::default(14);
    let osb = include_str!("./files/error_line_index_variable.osb");

    let err = osu.append_osb(osb).unwrap_err();

    assert_eq!(err.to_string(), "Line 3, Missing the header `$`");
}

#[test]
fn error_line_index_sb_in_osu() {
    let err = include_str!("./files/error_line_index_sb.osu")
        .parse::<OsuFile>()
        .unwrap_err();

    // line 45
    assert_eq!(err.to_string(), "Line 45, Unknown Command type");
}

#[test]
fn v5_timingpoint_full() {
    let i = include_str!("./files/v5_timingpoint_full.osu").replace("\r\n", "\n");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq!(i, o.to_string());
}
