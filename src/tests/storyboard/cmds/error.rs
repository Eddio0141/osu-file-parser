use crate::osu_file::{events::storyboard::cmds::Command, VersionedFromStr};

#[test]
fn storyboard_cmd_errors() {
    let missing_easing = "F";
    let invalid_easing = "F,this is wrong!,123";
    let missing_start_time = "F,0";
    let invalid_start_time = "F,0,foo";
    let missing_end_time = "F,0,0";
    let invalid_end_time = "F,0,0,foo";
    let missing_command_params = "F,0,0,0";
    let invalid_event = "foo,0,0,0,0,0,0";
    let missing_loop_count = "L,0";

    assert_eq!(
        "Unknown command type",
        Command::from_str(invalid_event, 14)
            .unwrap_err()
            .to_string()
    );
    assert_eq!(
        "Missing `easing` field",
        Command::from_str(missing_easing, 14)
            .unwrap_err()
            .to_string()
    );
    assert_eq!(
        "Invalid `easing` value",
        Command::from_str(invalid_easing, 14)
            .unwrap_err()
            .to_string()
    );
    assert_eq!(
        "Missing `start_time` field",
        Command::from_str(missing_start_time, 14)
            .unwrap_err()
            .to_string()
    );
    assert_eq!(
        "Invalid `start_time` value",
        Command::from_str(invalid_start_time, 14)
            .unwrap_err()
            .to_string()
    );
    assert_eq!(
        "Missing `end_time` field",
        Command::from_str(missing_end_time, 14)
            .unwrap_err()
            .to_string()
    );
    assert_eq!(
        "Invalid `end_time` value",
        Command::from_str(invalid_end_time, 14)
            .unwrap_err()
            .to_string()
    );
    assert_eq!(
        "Missing `start_opacity` field",
        Command::from_str(missing_command_params, 14)
            .unwrap_err()
            .to_string()
    );
    assert_eq!(
        "Missing `loop_count` field",
        Command::from_str(missing_loop_count, 14)
            .unwrap_err()
            .to_string()
    );
}

#[test]
fn continuing_error() {
    let colour_invalid_red = "C,0,0,1,foo";
    let missing_green = "C,0,0,1,255";
    let invalid_continuing_red = "C,0,0,0,255,255,255,foo";
    let missing_second_field = "V,0,0,0,0.5";
    let invalid_move_x_continuing = "M,0,0,0,100,-100,foo";

    assert_eq!(
        "Invalid continuing colour value",
        Command::from_str(invalid_continuing_red, 14)
            .unwrap_err()
            .to_string()
    );
    assert_eq!(
        "Invalid `red` value",
        Command::from_str(colour_invalid_red, 14)
            .unwrap_err()
            .to_string()
    );
    assert_eq!(
        "Missing `green` field",
        Command::from_str(missing_green, 14)
            .unwrap_err()
            .to_string()
    );
    assert_eq!(
        "Missing `scale_y` field",
        Command::from_str(missing_second_field, 14)
            .unwrap_err()
            .to_string()
    );
    assert_eq!(
        "Invalid continuing move value",
        Command::from_str(invalid_move_x_continuing, 14)
            .unwrap_err()
            .to_string()
    );
}
