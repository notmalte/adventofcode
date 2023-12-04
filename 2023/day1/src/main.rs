use std::{collections::HashMap, env, fs};

fn main() {
    let current_dir = env::current_dir().unwrap();

    let input_file = current_dir.join("day1/input.txt");

    println!("Reading input from: {}", input_file.display());

    let input = fs::read_to_string(input_file).unwrap();

    println!("Part 1: {}", part1(&input));

    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> isize {
    input
        .lines()
        .map(|line| {
            let mut digits = vec![];

            for char in line.chars() {
                if char.is_digit(10) {
                    digits.push(char.to_digit(10).unwrap() as isize);
                }
            }

            (10 * digits[0]) + digits[digits.len() - 1]
        })
        .sum()
}

fn part2(input: &str) -> isize {
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
                if chars[i].is_digit(10) {
                    digits.push(chars[i].to_digit(10).unwrap() as isize);
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
        .sum()
}
