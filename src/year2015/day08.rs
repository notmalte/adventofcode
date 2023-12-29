use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let chars = line.chars().collect::<Vec<_>>();

            let count_in_code = chars.len() as u64;

            let mut count = 0;
            let mut i = 1;

            while i < chars.len() - 1 {
                count += 1;

                if chars[i] == '\\' {
                    match chars[i + 1] {
                        '\\' | '"' => {
                            i += 1;
                        }
                        'x' => {
                            i += 3;
                        }
                        _ => malformed("2015", "08"),
                    }
                }

                i += 1;
            }

            count_in_code - count
        })
        .sum::<u64>()
        .to_string()
}

fn part2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let chars = line.chars().collect::<Vec<_>>();

            let mut count = 2;

            for c in chars {
                if c == '\\' || c == '"' {
                    count += 1;
                }
            }

            count
        })
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("\"\"\n\"abc\"\n\"aaa\\\"aaa\"\n\"\\x27\""), "12");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("\"\"\n\"abc\"\n\"aaa\\\"aaa\"\n\"\\x27\""), "19");
    }
}
