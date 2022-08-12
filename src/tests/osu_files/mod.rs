use crate::{assert_eq_osu_str, osu_file::OsuFile};

#[test]
fn v3() {
    let v3 = include_str!("./files/v3.osu");
    let osu_file = v3.parse::<OsuFile>().unwrap();

    assert_eq_osu_str(v3, osu_file.to_string());
}

#[test]
fn v3_to_v14() {
    let v3 = include_str!("./files/v3.osu");
    let v3_v14 = include_str!("./files/v3_v14.osu");
    let mut osu_file = v3.parse::<OsuFile>().unwrap();
    osu_file.version = 14;

    assert_eq_osu_str(v3_v14, osu_file.to_string());
}

#[test]
fn v4() {
    let v4 = include_str!("./files/v4.osu");
    let osu_file = v4.parse::<OsuFile>().unwrap();

    assert_eq_osu_str(v4, osu_file.to_string());
}

#[test]
fn v5() {
    let v5 = include_str!("./files/v5.osu");
    let osu_file = v5.parse::<OsuFile>().unwrap();

    assert_eq_osu_str(v5, osu_file.to_string());
}

#[test]
fn v6() {
    let v6 = include_str!("./files/v6.osu");
    let osu_file = v6.parse::<OsuFile>().unwrap();

    assert_eq_osu_str(v6, osu_file.to_string());
}

#[test]
fn v7() {
    let v7 = include_str!("./files/v7.osu");
    let osu_file = v7.parse::<OsuFile>().unwrap();

    assert_eq_osu_str(v7, osu_file.to_string());
}

#[test]
fn v8() {
    let v8 = include_str!("./files/v8.osu");
    let osu_file = v8.parse::<OsuFile>().unwrap();

    assert_eq_osu_str(v8, osu_file.to_string());
}

#[test]
fn v9() {
    let v9 = include_str!("./files/v9.osu");
    let osu_file = v9.parse::<OsuFile>().unwrap();

    assert_eq_osu_str(v9, osu_file.to_string());
}

#[test]
fn v9_spaces() {
    let v9_spaces = include_str!("./files/v9_spaces.osu");
    let osu_file = v9_spaces.parse::<OsuFile>().unwrap();

    assert_eq_osu_str(v9_spaces, osu_file.to_string());
}

#[test]
fn v10() {
    let v10 = include_str!("./files/v10.osu");
    let osu_file = v10.parse::<OsuFile>().unwrap();

    assert_eq_osu_str(v10, osu_file.to_string());
}

#[test]
fn v11() {
    let v11 = include_str!("./files/v11.osu");
    let osu_file = v11.parse::<OsuFile>().unwrap();

    assert_eq_osu_str(v11, osu_file.to_string());
}

#[test]
fn v12() {
    let v12 = include_str!("./files/v12.osu");
    let osu_file = v12.parse::<OsuFile>().unwrap();

    assert_eq_osu_str(v12, osu_file.to_string());
}

#[test]
fn v13() {
    let v13 = include_str!("./files/v13.osu");
    let osu_file = v13.parse::<OsuFile>().unwrap();

    assert_eq_osu_str(v13, osu_file.to_string());
}

#[test]
fn v14() {
    let v14 = include_str!("./files/v14.osu");
    let osu_file = v14.parse::<OsuFile>().unwrap();

    assert_eq_osu_str(v14, osu_file.to_string());
}

#[test]
fn v14_2() {
    let v14_2 = include_str!("./files/v14_2.osu");
    let osu_file = v14_2.parse::<OsuFile>().unwrap();

    assert_eq_osu_str(v14_2, osu_file.to_string());
}

#[test]
fn v14_3() {
    let v14_3 = include_str!("./files/v14_3.osu");
    let osu_file = v14_3.parse::<OsuFile>().unwrap();

    assert_eq_osu_str(v14_3, osu_file.to_string());
}

#[test]
fn acid_rain() {
    let acid_rain = include_str!("./files/acid_rain.osu");
    let acid_rain_osb = include_str!("./files/acid_rain.osb");

    let mut osu_file = acid_rain.parse::<OsuFile>().unwrap();
    assert_eq_osu_str(acid_rain, osu_file.to_string());

    osu_file.append_osb(&acid_rain_osb).unwrap();
    assert_eq_osu_str(acid_rain_osb, osu_file.osb_to_string().unwrap());
}

#[test]
fn aspire_osb1() {
    let osb = include_str!("./files/aspire_osb1.osb");
    let mut osu_file = OsuFile::default(14);
    osu_file.append_osb(&osb).unwrap();

    assert_eq_osu_str(osb, osu_file.osb_to_string().unwrap());
}

#[test]
fn aspire_osb2() {
    let osb = include_str!("./files/aspire_osb2.osb");
    let mut osu_file = OsuFile::default(14);
    osu_file.append_osb(&osb).unwrap();

    assert_eq_osu_str(osb, osu_file.osb_to_string().unwrap());
}

#[test]
fn aspire_osb3() {
    let osb = include_str!("./files/aspire_osb3.osb");
    let mut osu_file = OsuFile::default(14);
    osu_file.append_osb(&osb).unwrap();

    assert_eq_osu_str(osb, osu_file.osb_to_string().unwrap());
}

#[test]
fn aspire1() {
    let i = include_str!("./files/aspire1.osu");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq_osu_str(i, o.to_string());
}

#[test]
fn aspire2() {
    let i = include_str!("./files/aspire2.osu");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq_osu_str(i, o.to_string());
}

#[test]
fn aspire3() {
    let i = include_str!("./files/aspire3.osu");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq_osu_str(i, o.to_string());
}

#[test]
fn aspire4() {
    let i = include_str!("./files/aspire4.osu");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq_osu_str(i, o.to_string());
}

#[test]
fn aspire5() {
    let i = include_str!("./files/aspire5.osu");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq_osu_str(i, o.to_string());
}

#[test]
fn aspire6() {
    let i = include_str!("./files/aspire6.osu");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq_osu_str(i, o.to_string());
}

#[test]
fn aspire7() {
    let i = include_str!("./files/aspire7.osu");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq_osu_str(i, o.to_string());
}

#[test]
fn aspire8() {
    let i = include_str!("./files/aspire8.osu");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq_osu_str(i, o.to_string());
}

#[test]
fn aspire9() {
    let i = include_str!("./files/aspire9.osu");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq_osu_str(i, o.to_string());
}

#[test]
fn aspire10() {
    let i = include_str!("./files/aspire10.osu");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq_osu_str(i, o.to_string());
}

#[test]
fn aspire11() {
    let i = include_str!("./files/aspire11.osu");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq_osu_str(i, o.to_string());
}

#[test]
fn aspire12() {
    let i = include_str!("./files/aspire12.osu");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq_osu_str(i, o.to_string());
}

#[test]
fn aspire13() {
    let i = include_str!("./files/aspire13.osu");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq_osu_str(i, o.to_string());
}

#[test]
fn aspire14() {
    let i = include_str!("./files/aspire14.osu");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq_osu_str(i, o.to_string());
}

#[test]
fn aspire15() {
    let i = include_str!("./files/aspire15.osu");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq_osu_str(i, o.to_string());
}

#[test]
fn aspire16() {
    let i = include_str!("./files/aspire16.osu");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq_osu_str(i, o.to_string());
}

#[test]
fn aspire17() {
    let i = include_str!("./files/aspire17.osu");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq_osu_str(i, o.to_string());
}

#[test]
fn aspire18() {
    let i = include_str!("./files/aspire18.osu");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq_osu_str(i, o.to_string());
}

#[test]
fn aspire19() {
    let i = include_str!("./files/aspire19.osu");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq_osu_str(i, o.to_string());
}

#[test]
fn aspire20() {
    let i = include_str!("./files/aspire20.osu");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq_osu_str(i, o.to_string());
}

#[test]
fn aspire21() {
    let i = include_str!("./files/aspire21.osu");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq_osu_str(i, o.to_string());
}

#[test]
fn aspire22() {
    let i = include_str!("./files/aspire22.osu");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq_osu_str(i, o.to_string());
}

#[test]
fn aspire23() {
    let i = include_str!("./files/aspire23.osu");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq_osu_str(i, o.to_string());
}

#[test]
fn aspire24() {
    let i = include_str!("./files/aspire24.osu");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq_osu_str(i, o.to_string());
}

#[test]
fn aspire25() {
    let i = include_str!("./files/aspire25.osu");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq_osu_str(i, o.to_string());
}

#[test]
fn aspire26() {
    let i = include_str!("./files/aspire26.osu");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq_osu_str(i, o.to_string());
}

#[test]
fn aspire27() {
    let i = include_str!("./files/aspire27.osu");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq_osu_str(i, o.to_string());
}

#[test]
fn aspire28() {
    let i = include_str!("./files/aspire28.osu");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq_osu_str(i, o.to_string());
}

#[test]
fn aspire29() {
    let i = include_str!("./files/aspire29.osu");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq_osu_str(i, o.to_string());
}

#[test]
fn error_line_index_with_leading_ws() {
    let i = include_str!("./files/leading_ws_w_err.osu");
    let o = i.parse::<OsuFile>().unwrap_err();
    assert_eq_osu_str(
        o.to_string(),
        "Line 7, Invalid colon set, expected format of `key: value`",
    );
}

#[test]
fn variable_osb() {
    let mut osu = OsuFile::default(14);
    let osb = include_str!("./files/variable.osb");

    osu.append_osb(&osb).unwrap();

    assert_eq_osu_str(osu.osb_to_string().unwrap(), osb);
}

#[test]
fn variable2_osb() {
    let mut osu = OsuFile::default(14);
    let osb = include_str!("./files/variable2.osb");

    osu.append_osb(&osb).unwrap();

    assert_eq_osu_str(osu.osb_to_string().unwrap(), osb);
}

#[test]
fn error_line_index_osb() {
    let mut osu = OsuFile::default(14);
    let osb = include_str!("./files/error_line_index.osb");

    let err = osu.append_osb(osb).unwrap_err();

    assert_eq_osu_str(err.to_string(), "Line 21, Unknown command type");
}

#[test]
fn error_line_index_variable_osb() {
    let mut osu = OsuFile::default(14);
    let osb = include_str!("./files/error_line_index_variable.osb");

    let err = osu.append_osb(osb).unwrap_err();

    assert_eq_osu_str(err.to_string(), "Line 3, Missing the header `$`");
}

#[test]
fn error_line_index_sb_in_osu() {
    let err = include_str!("./files/error_line_index_sb.osu")
        .parse::<OsuFile>()
        .unwrap_err();

    // line 46
    assert_eq_osu_str(err.to_string(), "Line 46, Unknown command type");
}

#[test]
fn v5_timingpoint_full() {
    let i = include_str!("./files/v5_timingpoint_full.osu");
    let o = i.parse::<OsuFile>().unwrap();
    assert_eq_osu_str(i, o.to_string());
}

#[test]
fn osb() {
    let mut osu = OsuFile::default(14);
    let osb = include_str!("./files/osb.osb");

    osu.append_osb(&osb).unwrap();

    assert_eq_osu_str(osu.osb_to_string().unwrap(), osb);
}

#[test]
fn osb_2() {
    let mut osu = OsuFile::default(14);
    let osb = include_str!("./files/osb_2.osb");

    osu.append_osb(&osb).unwrap();

    assert_eq_osu_str(osu.osb_to_string().unwrap(), osb);
}

#[test]
fn missing_effects_field() {
    let i = include_str!("./files/missing_effects_field.osu");
    let o = i.parse::<OsuFile>().unwrap();

    assert_eq_osu_str(i, o.to_string());
}

#[test]
fn oblivion_aspire() {
    let i = include_str!("./files/oblivion_aspire.osu");
    let o = i.parse::<OsuFile>().unwrap();

    assert_eq_osu_str(i, o.to_string());
}

#[test]
fn match_test() {
    let i = include_str!("./files/match_test.osu");
    let o = i.parse::<OsuFile>().unwrap();

    assert_eq_osu_str(i, o.to_string());
}

#[test]
fn match_test2() {
    let i = include_str!("./files/match_test2.osu");
    let o = i.parse::<OsuFile>().unwrap();

    assert_eq_osu_str(i, o.to_string());
}

#[test]
fn match_test_osb() {
    let mut osu = OsuFile::default(14);
    let osb = include_str!("./files/match_test.osb");

    osu.append_osb(&osb).unwrap();

    assert_eq_osu_str(osu.osb_to_string().unwrap(), osb);
}

#[test]
fn combo_blue() {
    let i = include_str!("./files/combo_blue.osu");
    let o = i.parse::<OsuFile>().unwrap();

    assert_eq_osu_str(i, o.to_string());
}
