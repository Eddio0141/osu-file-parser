use crate::osu_file::hitobject::HitObject;

#[test]
fn invalid_int() {
    let i = "";
    let o = i.parse::<HitObject>().unwrap_err();

    assert_eq!("Failed to parse `` as an integer", o.to_string());
}

#[test]
fn missing_field_y() {
    let i = "1";
    let o = i.parse::<HitObject>().unwrap_err();

    assert_eq!("The hitobject is missing the `Y` field", o.to_string());
}

#[test]
fn invalid_int2() {
    let i = "1,foo";
    let o = i.parse::<HitObject>().unwrap_err();

    assert_eq!("Failed to parse `foo` as an integer", o.to_string());
}

#[test]
fn invalid_decimal() {
    let i = "0,0,0,2,0,B|0:0,0,foo";
    let o = i.parse::<HitObject>().unwrap_err();

    assert_eq!("Failed to parse `foo` as a decimal", o.to_string());
}

#[test]
fn unknown_object() {
    let i = "0,0,0,0,0,0:0:0:0:";
    let o = i.parse::<HitObject>().unwrap_err();

    assert_eq!("Unknown object type", o.to_string());
}

#[test]
fn missing_obj_params() {
    let i = "0,0,0,2,0";
    let o = i.parse::<HitObject>().unwrap_err();

    assert_eq!("Missing object params", o.to_string());
}
