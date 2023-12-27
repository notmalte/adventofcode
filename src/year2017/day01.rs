use crate::Answer;

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    let chars = input.chars().collect::<Vec<_>>();

    let mut sum = 0;

    for i in 0..chars.len() {
        if chars[i] == chars[(i + 1) % chars.len()] {
            sum += chars[i].to_digit(10).unwrap();
        }
    }

    sum.to_string()
}

fn part2(input: &str) -> String {
    let chars = input.chars().collect::<Vec<_>>();

    let offset = chars.len() / 2;

    let mut sum = 0;

    for i in 0..chars.len() {
        if chars[i] == chars[(i + offset) % chars.len()] {
            sum += chars[i].to_digit(10).unwrap();
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let cases = vec![
            ("1122", "3"),
            ("1111", "4"),
            ("1234", "0"),
            ("91212129", "9"),
        ];

        for (input, expected) in cases {
            assert_eq!(part1(input), expected);
        }
    }

    #[test]
    fn test_part2() {
        let cases = vec![
            ("1212", "6"),
            ("1221", "0"),
            ("123425", "4"),
            ("123123", "12"),
            ("12131415", "4"),
        ];

        for (input, expected) in cases {
            assert_eq!(part2(input), expected);
        }
    }
}
