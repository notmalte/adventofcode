use regex::Regex;

use crate::Answer;

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(input)
        .map(|cap| cap[1].parse::<u64>().unwrap() * cap[2].parse::<u64>().unwrap())
        .sum::<u64>()
        .to_string()
}

fn part2(input: &str) -> String {
    let re = Regex::new(r"(do\(\))|(don't\(\))|(mul\((?<lhs>\d{1,3}),(?<rhs>\d{1,3})\))").unwrap();

    let mut enabled = true;
    let mut sum = 0;

    for cap in re.captures_iter(input) {
        match cap.get(0).unwrap().as_str() {
            "do()" => {
                enabled = true;
            }
            "don't()" => {
                enabled = false;
            }
            _ => {
                if enabled {
                    sum += cap["lhs"].parse::<u64>().unwrap() * cap["rhs"].parse::<u64>().unwrap();
                }
            }
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            "161"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            "48"
        );
    }
}
