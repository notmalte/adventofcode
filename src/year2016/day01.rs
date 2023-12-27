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

    let mut x = 0;
    let mut y = 0;

    let mut dir = 0;

    for (d, n) in parsed {
        match d {
            "R" => dir += 1,
            "L" => dir -= 1,
            _ => malformed("2016", "01"),
        }

        dir = (dir + 4) % 4;

        match dir {
            0 => y += n,
            1 => x += n,
            2 => y -= n,
            3 => x -= n,
            _ => malformed("2016", "01"),
        }
    }

    (x.abs() + y.abs()).to_string()
}

fn part2(input: &str) -> String {
    let parsed = parse_input(input);

    let mut visited = HashSet::new();

    let mut x = 0i64;
    let mut y = 0i64;

    let mut dir = 0;

    for (d, n) in parsed {
        match d {
            "R" => dir += 1,
            "L" => dir -= 1,
            _ => malformed("2016", "01"),
        }

        dir = (dir + 4) % 4;

        for _ in 0..n {
            match dir {
                0 => y += 1,
                1 => x += 1,
                2 => y -= 1,
                3 => x -= 1,
                _ => malformed("2016", "01"),
            }

            if !visited.insert((x, y)) {
                return (x.abs() + y.abs()).to_string();
            }
        }
    }

    malformed("2016", "01")
}

fn parse_input(input: &str) -> Vec<(&str, i64)> {
    input
        .split(", ")
        .map(|s| {
            let (dir, n) = s.split_at(1);

            (
                dir,
                n.parse::<i64>().unwrap_or_else(|_| malformed("2016", "01")),
            )
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let cases = vec![
            ("R2, L3", "5"),
            ("R2, R2, R2", "2"),
            ("R5, L5, R5, R3", "12"),
        ];

        for (input, expected) in cases {
            assert_eq!(part1(input), expected);
        }
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("R8, R4, R4, R8"), "4");
    }
}
