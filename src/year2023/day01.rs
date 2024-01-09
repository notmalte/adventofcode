use std::collections::HashMap;

use crate::Answer;

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
            let mut digits = vec![];

            for char in line.chars() {
                if char.is_ascii_digit() {
                    digits.push(char.to_digit(10).unwrap() as u64);
                }
            }

            (10 * digits[0]) + digits[digits.len() - 1]
        })
        .sum::<u64>()
        .to_string()
}

fn part2(input: &str) -> String {
    let digit_strings = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    input
        .lines()
        .map(|line| {
            let mut digits = vec![];

            let chars: Vec<_> = line.chars().collect();

            for i in 0..chars.len() {
                if chars[i].is_ascii_digit() {
                    digits.push(chars[i].to_digit(10).unwrap() as u64);
                } else {
                    for (key, value) in digit_strings.iter() {
                        if i + key.len() > chars.len() {
                            continue;
                        }

                        let substring: String = chars[i..i + key.len()].iter().collect();

                        if substring == *key {
                            digits.push(*value);
                        }
                    }
                }
            }

            (10 * digits[0]) + digits[digits.len() - 1]
        })
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet"), "142");
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2("two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen"),
            "281"
        );
    }
}
