use crate::osu_file::hitobjects::HitObject;

#[test]
fn hitobjects_parse() {
    let hitcircle_str = "221,350,9780,1,0,0:0:0:0:";
    let slider_str = "31,85,3049,2,0,B|129:55|123:136|228:86,1,172.51,2|0,3:2|0:2,0:2:0:0:";
    let spinner_str = "256,192,33598,12,0,431279,0:0:0:0:";
    let osu_mania_hold_str = "51,192,350,128,2,849:0:0:0:0:";

    let hitcircle: HitObject = hitcircle_str.parse().unwrap();
    let slider: HitObject = slider_str.parse().unwrap();
    let spinner: HitObject = spinner_str.parse().unwrap();
    let osu_mania_hold: HitObject = osu_mania_hold_str.parse().unwrap();

    assert_eq!(hitcircle_str, hitcircle.to_string());
    assert_eq!(slider_str, slider.to_string());
    assert_eq!(spinner_str, spinner.to_string());
    assert_eq!(osu_mania_hold_str, osu_mania_hold.to_string());
}

#[test]
fn short_hand() {
    let hitcircle_str = "221,350,9780,1,0";
    let slider_str = "31,85,3049,2,0,B|129:55|123:136|228:86,1,172.51";

    let hitcircle: HitObject = hitcircle_str.parse().unwrap();
    let slider: HitObject = slider_str.parse().unwrap();

    assert_eq!(hitcircle_str, hitcircle.to_string());
    assert_eq!(slider_str, slider.to_string());
}
