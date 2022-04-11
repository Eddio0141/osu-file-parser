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
        "Unknown event: foo",
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
        "Missing command fields",
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
