use crate::Answer;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;

pub fn run(day: u64, input: &str) -> Option<Answer> {
    match day {
        1 => Some(day01::run(input)),
        2 => Some(day02::run(input)),
        3 => Some(day03::run(input)),
        4 => Some(day04::run(input)),
        5 => Some(day05::run(input)),
        6 => Some(day06::run(input)),
        7 => Some(day07::run(input)),
        _ => None,
    }
}
