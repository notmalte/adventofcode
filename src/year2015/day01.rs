use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: Some(part1(input)),
        part2: Some(part2(input)),
    }
}

fn part1(input: &str) -> String {
    let mut floor = 0;

    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
    }

    floor.to_string()
}

fn part2(input: &str) -> String {
    let mut floor = 0;

    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => {
                floor -= 1;
                if floor == -1 {
                    return (i + 1).to_string();
                }
            }
            _ => (),
        }
    }

    malformed()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let cases = vec![
            ("(())", "0"),
            ("()()", "0"),
            ("(((", "3"),
            ("(()(()(", "3"),
            ("))(((((", "3"),
            ("())", "-1"),
            ("))(", "-1"),
            (")))", "-3"),
            (")())())", "-3"),
        ];

        for (input, expected) in cases {
            assert_eq!(part1(input), expected);
        }
    }

    #[test]
    fn test_part2() {
        let cases = vec![(")", "1"), ("()())", "5")];

        for (input, expected) in cases {
            assert_eq!(part2(input), expected);
        }
    }
}
