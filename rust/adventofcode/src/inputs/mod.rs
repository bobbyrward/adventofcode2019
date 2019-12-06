pub fn load_input(day: i32) -> String {
    match day {
        1 => include_str!("day01.txt").into(),
        2 => include_str!("day02.txt").into(),
        3 => include_str!("day03.txt").into(),
        4 => "206938-679128".into(),
        5 => include_str!("day05.txt").into(),
        6 => include_str!("day06.txt").into(),
        _ => panic!("Unknown day for input"),
    }
}
