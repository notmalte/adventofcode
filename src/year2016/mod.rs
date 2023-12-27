use crate::Answer;

pub mod day01;

pub fn run(day: u64, input: &str) -> Option<Answer> {
    match day {
        1 => Some(day01::run(input)),
        _ => None,
    }
}
