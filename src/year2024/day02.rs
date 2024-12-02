use itertools::Itertools;

use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    let parsed = parse_input(input);

    parsed.iter().filter(|v| is_safe(v)).count().to_string()
}

fn part2(input: &str) -> String {
    let parsed = parse_input(input);

    parsed
        .iter()
        .filter(|&report| {
            is_safe(report)
                || (0..report.len()).any(|i| {
                    let mut clone = report.clone();
                    clone.remove(i);
                    is_safe(&clone)
                })
        })
        .count()
        .to_string()
}

fn is_safe(report: &[i64]) -> bool {
    report.iter().tuple_windows().all(|(a, b, c)| {
        let ab = a - b;
        let bc = b - c;

        (1..=3).contains(&ab.abs())
            && (1..=3).contains(&bc.abs())
            && (ab.is_positive() == bc.is_positive())
    })
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|s| {
            s.split_whitespace()
                .map(|s| s.parse().unwrap_or_else(|_| malformed("2024", "02")))
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            ),
            "2"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            ),
            "4"
        );
    }
}
