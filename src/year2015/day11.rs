use itertools::Itertools;

use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    const LIMIT: u64 = 1_000_000_000;

    let mut password = increment_password(input.to_string());

    for _ in 0..LIMIT {
        if is_valid_password(&password) {
            return password;
        }

        password = increment_password(password);
    }

    malformed("2015", "11")
}

fn part2(input: &str) -> String {
    part1(&part1(input))
}

fn is_valid_password(password: &String) -> bool {
    let chars = password.chars().collect::<Vec<_>>();

    let three_increasing_letters = chars.iter().tuple_windows().any(|(&a, &b, &c)| {
        (a < 'z') && ((a as u8 + 1) == (b as u8)) && ((b as u8 + 1) == (c as u8))
    });

    let no_confusing_letters = !chars.iter().any(|&c| c == 'i' || c == 'o' || c == 'l');

    let mut count = 0;
    let mut i = 0;

    while i < chars.len() - 1 {
        if chars[i] == chars[i + 1] {
            count += 1;
            i += 1;
        }
        i += 1;
    }

    let two_pairs = count >= 2;

    three_increasing_letters && no_confusing_letters && two_pairs
}

fn increment_password(password: String) -> String {
    let mut chars = password.chars().collect::<Vec<_>>();

    for i in (0..chars.len()).rev() {
        if chars[i] < 'z' {
            chars[i] = (chars[i] as u8 + 1) as char;

            return chars.iter().collect();
        }

        chars[i] = 'a';
    }

    malformed("2015", "11")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("abcdefgh"), "abcdffaa");
        assert_eq!(part1("ghijklmn"), "ghjaabcc");
    }
}
