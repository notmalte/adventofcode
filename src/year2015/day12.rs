use serde_json::Value;

use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    let re = regex::Regex::new(r"-?\d+").unwrap();

    re.find_iter(input)
        .map(|v| v.as_str().parse::<i64>().unwrap())
        .sum::<i64>()
        .to_string()
}

fn part2(input: &str) -> String {
    sum_not_red(&serde_json::from_str(input).unwrap_or_else(|_| malformed("2015", "12")))
        .to_string()
}

fn sum_not_red(value: &Value) -> i64 {
    match value {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(n) => n.as_i64().unwrap_or_else(|| malformed("2015", "12")),
        Value::String(_) => 0,
        Value::Array(a) => a.iter().map(sum_not_red).sum(),
        Value::Object(o) => {
            if o.values().any(|v| v == "red") {
                0
            } else {
                o.values().map(sum_not_red).sum()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let cases = vec![
            ("[1,2,3]", "6"),
            ("{\"a\":2,\"b\":4}", "6"),
            ("[[[3]]]", "3"),
            ("{\"a\":{\"b\":4},\"c\":-1}", "3"),
            ("{\"a\":[-1,1]}", "0"),
            ("[-1,{\"a\":1}]", "0"),
            ("[]", "0"),
            ("{}", "0"),
        ];

        for (input, expected) in cases {
            assert_eq!(part1(input), expected);
        }
    }

    #[test]
    fn test_part2() {
        let cases = vec![
            ("[1,2,3]", "6"),
            ("[1,{\"c\":\"red\",\"b\":2},3]", "4"),
            ("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}", "0"),
            ("[1,\"red\",5]", "6"),
        ];

        for (input, expected) in cases {
            assert_eq!(part2(input), expected);
        }
    }
}
