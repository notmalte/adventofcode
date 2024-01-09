use std::collections::HashSet;

use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    let parsed = parse_input(input);

    let mut delivered = HashSet::new();

    let mut x = 0;
    let mut y = 0;

    delivered.insert((x, y));

    for direction in parsed {
        match direction {
            Direction::North => y += 1,
            Direction::East => x += 1,
            Direction::South => y -= 1,
            Direction::West => x -= 1,
        }

        delivered.insert((x, y));
    }

    delivered.len().to_string()
}

fn part2(input: &str) -> String {
    let parsed = parse_input(input);

    let mut delivered = HashSet::new();

    let a = parsed.iter().step_by(2).collect::<Vec<_>>();
    let b = parsed.iter().skip(1).step_by(2).collect::<Vec<_>>();

    for carrier in [a, b] {
        let mut x = 0;
        let mut y = 0;

        delivered.insert((x, y));

        for direction in carrier {
            match direction {
                Direction::North => y += 1,
                Direction::East => x += 1,
                Direction::South => y -= 1,
                Direction::West => x -= 1,
            }

            delivered.insert((x, y));
        }
    }

    delivered.len().to_string()
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn parse_input(input: &str) -> Vec<Direction> {
    input
        .chars()
        .map(|c| match c {
            '^' => Direction::North,
            '>' => Direction::East,
            'v' => Direction::South,
            '<' => Direction::West,
            _ => malformed("2015", "03"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let cases = vec![(">", "2"), ("^>v<", "4"), ("^v^v^v^v^v", "2")];

        for (input, expected) in cases {
            assert_eq!(part1(input), expected);
        }
    }

    #[test]
    fn test_part2() {
        let cases = vec![("^v", "3"), ("^>v<", "3"), ("^v^v^v^v^v", "11")];

        for (input, expected) in cases {
            assert_eq!(part2(input), expected);
        }
    }
}
