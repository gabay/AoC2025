use std::{env::args, fmt::Display};

mod util;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

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
        "3" => run(day3::part1, day3::part2, &data),
        "4" => run(day4::part1, day4::part2, &data),
        "5" => run(day5::part1, day5::part2, &data),
        _ => usage(),
    }
}

fn run<T: Display>(part1: impl Fn(&str) -> T, part2: impl Fn(&str) -> T, data: &str) {
    println!("Part 1: {}", part1(data));
    println!("Part 2: {}", part2(data));
}