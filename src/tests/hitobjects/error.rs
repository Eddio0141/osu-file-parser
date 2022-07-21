use crate::osu_file::{hitobjects::HitObject, VersionedFromStr};

#[test]
fn missing_field_y() {
    let i = "1";
    let o = HitObject::from_str(i, 14).unwrap_err();

    assert_eq!("Missing `y` field", o.to_string());
}

#[test]
fn unknown_object() {
    let i = "0,0,0,0,0,0:0:0:0:";
    let o = HitObject::from_str(i, 14).unwrap_err();

    assert_eq!("Unknown object type", o.to_string());
}

#[test]
fn missing_obj_params() {
    let i = "0,0,0,2,0";
    let o = HitObject::from_str(i, 14).unwrap_err();

    assert_eq!("Missing `curve_type` field", o.to_string());
}
