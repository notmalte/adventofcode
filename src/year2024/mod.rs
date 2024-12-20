use crate::Answer;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;

pub fn run(day: u64, input: &str) -> Option<Answer> {
    match day {
        1 => Some(day01::run(input)),
        2 => Some(day02::run(input)),
        3 => Some(day03::run(input)),
        4 => Some(day04::run(input)),
        5 => Some(day05::run(input)),
        6 => Some(day06::run(input)),
        7 => Some(day07::run(input)),
        8 => Some(day08::run(input)),
        9 => Some(day09::run(input)),
        10 => Some(day10::run(input)),
        11 => Some(day11::run(input)),
        12 => Some(day12::run(input)),
        13 => Some(day13::run(input)),
        14 => Some(day14::run(input)),
        15 => Some(day15::run(input)),
        16 => Some(day16::run(input)),
        17 => Some(day17::run(input)),
        18 => Some(day18::run(input)),
        19 => Some(day19::run(input)),
        20 => Some(day20::run(input)),
        _ => None,
    }
}
