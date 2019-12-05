// use std::fs::File;
// use std::io::prelude::*;

pub fn load_input(day: i32) -> String {
    match day {
        1 => include_str!("day01.txt").into(),
        2 => include_str!("day02.txt").into(),
        _ => panic!("Unknown day for input"),
    }

    /*
    let fname = format!("day{:02}.txt", day);

    let mut buffer = String::new();

    File::open(fname)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    buffer
    */
}
