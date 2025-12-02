use std::{env::args, fmt::Display};

mod util;
mod day1;
mod day2;

fn usage() {
    println!("Usage: cargo run <DAY>");
    println!("* the input is provided in a file named DAY (e.g. 3)");
}

fn main() {
    let Some(day) = args().last() else {
        usage();
        return;
    };
    let Some(data) = util::read(&day) else {
        usage();
        return;
    };
    match day.as_str() {
        "1" => run(day1::part1, day1::part2, &data),
        "2" => run(day2::part1, day2::part2, &data),
        _ => usage(),
    }
}

fn run<T: Display>(part1: impl Fn(&str) -> T, part2: impl Fn(&str) -> T, data: &str) {
    println!("Part 1: {}", part1(data));
    println!("Part 2: {}", part2(data));
}