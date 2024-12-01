use std::collections::HashMap;

use itertools::Itertools;

use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    let (mut left, mut right) = parse_input(input);

    left.sort();
    right.sort();

    left.iter()
        .zip_eq(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum::<i64>()
        .to_string()
}

fn part2(input: &str) -> String {
    let (left, right) = parse_input(input);

    let mut map = HashMap::new();

    for n in right {
        *map.entry(n).or_insert(0) += 1;
    }

    let mut sum = 0;

    for n in left {
        sum += map.get(&n).unwrap_or(&0) * n;
    }

    sum.to_string()
}

fn parse_input(input: &str) -> (Vec<i64>, Vec<i64>) {
    input
        .lines()
        .map(|s| {
            let split = s.split_whitespace().collect_vec();

            if split.len() != 2 {
                malformed("2024", "01")
            }

            (
                split[0]
                    .parse::<i64>()
                    .unwrap_or_else(|_| malformed("2024", "01")),
                split[1]
                    .parse::<i64>()
                    .unwrap_or_else(|_| malformed("2024", "01")),
            )
        })
        .unzip()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "3   4
4   3
2   5
1   3
3   9
3   3"
            ),
            "11"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "3   4
4   3
2   5
1   3
3   9
3   3"
            ),
            "31"
        );
    }
}
