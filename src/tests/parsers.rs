use nom::{
    bytes::complete::{tag, take_while, take_till},
    multi::separated_list0,
    Parser,
};

use crate::parsers::get_colon_field_value_lines;

#[test]
fn colon_field_value() {
    let (_, fields) = get_colon_field_value_lines("source:\ntags:\n").unwrap();

    assert_eq!(fields[0].0, "source");
    assert_eq!(fields[1].0, "tags");
}

#[test]
fn take_till_new_line() {
    let parser = take_till::<_, _, nom::error::Error<_>>(|c| c == '\n' || c == '\r');

    let (_, result) = parser("abc\r\ndef").unwrap();
    let (_, result2) = parser("\r\ntest").unwrap();

    assert_eq!(result, "abc");
    assert_eq!(result2, "");
}

#[test]
fn pipe_vec_parse() {
    let mut pipe_vec = separated_list0::<_, _, _, nom::error::Error<_>, _, _>(
        tag("|"),
        take_while(|c: char| !['|', ',', '\r', '\n'].contains(&c)),
    );

    let i = "1|2|3,foo";
    let (_, v) = pipe_vec.parse(i).unwrap();

    assert_eq!(v, vec!["1", "2", "3"]);
}
