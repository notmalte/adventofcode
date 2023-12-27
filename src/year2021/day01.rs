use crate::Answer;

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    let parsed = parse_input(input);

    let mut sum = 0;

    for i in 1..parsed.len() {
        if parsed[i] > parsed[i - 1] {
            sum += 1;
        }
    }

    sum.to_string()
}

fn part2(input: &str) -> String {
    let parsed = parse_input(input);

    let mut sum = 0;

    for i in 3..parsed.len() {
        if parsed[i] > parsed[i - 3] {
            sum += 1;
        }
    }

    sum.to_string()
}

fn parse_input(input: &str) -> Vec<u64> {
    input.lines().map(|l| l.parse::<u64>().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1("199\n200\n208\n210\n200\n207\n240\n269\n260\n263"),
            "7"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2("199\n200\n208\n210\n200\n207\n240\n269\n260\n263"),
            "5"
        );
    }
}
