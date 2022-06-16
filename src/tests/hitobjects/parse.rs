use crate::osu_file::{hitobjects::HitObject, VersionedFromString, VersionedToString};
use pretty_assertions::assert_eq;

#[test]
fn hitobjects_parse() {
    let hitcircle_str = "221,350,9780,1,0,0:0:0:0:";
    let slider_str = "31,85,3049,2,0,B|129:55|123:136|228:86,1,172.51,2|0,3:2|0:2,0:2:0:0:";
    let spinner_str = "256,192,33598,12,0,431279,0:0:0:0:";
    let osu_mania_hold_str = "51,192,350,128,2,849:0:0:0:0:";

    let hitcircle = HitObject::from_str(hitcircle_str, 14).unwrap().unwrap();
    let slider = HitObject::from_str(slider_str, 14).unwrap().unwrap();
    let spinner = HitObject::from_str(spinner_str, 14).unwrap().unwrap();
    let osu_mania_hold = HitObject::from_str(osu_mania_hold_str, 14)
        .unwrap()
        .unwrap();

    assert_eq!(hitcircle_str, hitcircle.to_string(14).unwrap());
    assert_eq!(slider_str, slider.to_string(14).unwrap());
    assert_eq!(spinner_str, spinner.to_string(14).unwrap());
    assert_eq!(osu_mania_hold_str, osu_mania_hold.to_string(14).unwrap());
}

#[test]
fn short_hand() {
    let hitcircle_str = "221,350,9780,1,0";
    let slider_str = "31,85,3049,2,0,B|129:55|123:136|228:86,1,172.51";

    let hitcircle = HitObject::from_str(hitcircle_str, 14).unwrap().unwrap();
    let slider = HitObject::from_str(slider_str, 14).unwrap().unwrap();

    assert_eq!(hitcircle_str, hitcircle.to_string(14).unwrap());
    assert_eq!(slider_str, slider.to_string(14).unwrap());
}
