use crate::osu_file::events::storyboard::cmds::Command;

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
        invalid_event.parse::<Command>().unwrap_err().to_string()
    );
    assert_eq!(
        "Missing the Easing field",
        missing_easing.parse::<Command>().unwrap_err().to_string()
    );
    assert_eq!(
        "Invalid easing: this is wrong!",
        invalid_easing.parse::<Command>().unwrap_err().to_string()
    );
    assert_eq!(
        "Missing the StartTime field",
        missing_start_time
            .parse::<Command>()
            .unwrap_err()
            .to_string()
    );
    assert_eq!(
        "Tried parsing a str foo as an integer",
        invalid_start_time
            .parse::<Command>()
            .unwrap_err()
            .to_string()
    );
    assert_eq!(
        "Missing the EndTime field",
        missing_end_time.parse::<Command>().unwrap_err().to_string()
    );
    assert_eq!(
        "Tried parsing a str foo as an integer",
        invalid_end_time.parse::<Command>().unwrap_err().to_string()
    );
    assert_eq!(
        "Missing additional command fields",
        missing_command_params
            .parse::<Command>()
            .unwrap_err()
            .to_string()
    );
    assert_eq!(
        "Missing the LoopCount field",
        missing_loop_count
            .parse::<Command>()
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
        invalid_continuing_red
            .parse::<Command>()
            .unwrap_err()
            .to_string()
    );
    assert_eq!(
        "Invalid `red` value",
        colour_invalid_red
            .parse::<Command>()
            .unwrap_err()
            .to_string()
    );
    assert_eq!(
        "Missing `green` field",
        missing_green.parse::<Command>().unwrap_err().to_string()
    );
    assert_eq!(
        "Missing `scale_y` field",
        missing_second_field
            .parse::<Command>()
            .unwrap_err()
            .to_string()
    );
    assert_eq!(
        "Invalid continuing move value",
        invalid_move_x_continuing
            .parse::<Command>()
            .unwrap_err()
            .to_string()
    );
}
