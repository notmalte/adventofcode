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
        .filter(|line| {
            let chars = line.chars().collect::<Vec<_>>();

            let at_least_3_vowels = chars
                .iter()
                .filter(|c| matches!(**c, 'a' | 'e' | 'i' | 'o' | 'u'))
                .count()
                >= 3;

            let mut has_double = false;
            for i in 0..chars.len() - 1 {
                if chars[i] == chars[i + 1] {
                    has_double = true;
                    break;
                }
            }

            let has_bad = line.contains("ab")
                || line.contains("cd")
                || line.contains("pq")
                || line.contains("xy");

            at_least_3_vowels && has_double && !has_bad
        })
        .count()
        .to_string()
}

fn part2(input: &str) -> String {
    input
        .lines()
        .filter(|line| {
            let chars = line.chars().collect::<Vec<_>>();

            let mut has_double_pair = false;
            for i in 0..chars.len() - 3 {
                for j in i + 2..chars.len() - 1 {
                    if chars[i] == chars[j] && chars[i + 1] == chars[j + 1] {
                        has_double_pair = true;
                        break;
                    }
                }
            }

            let mut has_double_with_gap = false;
            for i in 0..chars.len() - 2 {
                if chars[i] == chars[i + 2] {
                    has_double_with_gap = true;
                    break;
                }
            }

            has_double_pair && has_double_with_gap
        })
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let cases = vec![
            ("ugknbfddgicrmopn", "1"),
            ("aaa", "1"),
            ("jchzalrnumimnmhp", "0"),
            ("haegwjzuvuyypxyu", "0"),
            ("dvszwmarrgswjxmb", "0"),
        ];

        for (input, expected) in cases {
            assert_eq!(part1(input), expected);
        }
    }

    #[test]
    fn test_part2() {
        let cases = vec![
            ("qjhvhtzxzqqjkmpb", "1"),
            ("xxyxx", "1"),
            ("uurcxstgmygtbstg", "0"),
            ("ieodomkazucvgmuy", "0"),
        ];

        for (input, expected) in cases {
            assert_eq!(part2(input), expected);
        }
    }
}
