use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    let parsed = parse_input(input);

    for i in 0..parsed.len() {
        for j in i + 1..parsed.len() {
            if parsed[i] + parsed[j] == 2020 {
                return (parsed[i] * parsed[j]).to_string();
            }
        }
    }

    malformed("2020", "01")
}

fn part2(input: &str) -> String {
    let parsed = parse_input(input);

    for i in 0..parsed.len() {
        for j in i + 1..parsed.len() {
            for k in j + 1..parsed.len() {
                if parsed[i] + parsed[j] + parsed[k] == 2020 {
                    return (parsed[i] * parsed[j] * parsed[k]).to_string();
                }
            }
        }
    }

    malformed("2020", "01")
}

fn parse_input(input: &str) -> Vec<u64> {
    input.lines().map(|l| l.parse::<u64>().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("1721\n979\n366\n299\n675\n1456"), "514579");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("1721\n979\n366\n299\n675\n1456"), "241861950");
    }
}
