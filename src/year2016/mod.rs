use crate::Answer;

pub mod day01;
pub mod day02;

pub fn run(day: u64, input: &str) -> Option<Answer> {
    match day {
        1 => Some(day01::run(input)),
        2 => Some(day02::run(input)),
        _ => None,
    }
}
