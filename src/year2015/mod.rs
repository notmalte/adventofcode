use crate::Answer;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;

pub fn run(day: u64, input: &str) -> Option<Answer> {
    match day {
        1 => Some(day01::run(input)),
        2 => Some(day02::run(input)),
        3 => Some(day03::run(input)),
        4 => Some(day04::run(input)),
        _ => None,
    }
}
