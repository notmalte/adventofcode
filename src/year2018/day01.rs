use std::collections::HashSet;

use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    input
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .sum::<i64>()
        .to_string()
}

fn part2(input: &str) -> String {
    let mut visited = HashSet::new();

    let parsed = input.split_whitespace().map(|s| s.parse::<i64>().unwrap());

    let mut sum = 0;

    visited.insert(sum);

    for n in parsed.cycle() {
        sum += n;

        if visited.contains(&sum) {
            return sum.to_string();
        }

        visited.insert(sum);
    }

    malformed("2018", "01")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let cases = vec![
            ("+1\n+1\n+1", "3"),
            ("+1\n+1\n-2", "0"),
            ("-1\n-2\n-3", "-6"),
        ];

        for (input, expected) in cases {
            assert_eq!(part1(input), expected);
        }
    }

    #[test]
    fn test_part2() {
        let cases = vec![
            ("+1\n-1", "0"),
            ("+3\n+3\n+4\n-2\n-4", "10"),
            ("-6\n+3\n+8\n+5\n-6", "5"),
            ("+7\n+7\n-2\n-7\n-4", "14"),
        ];

        for (input, expected) in cases {
            assert_eq!(part2(input), expected);
        }
    }
}
