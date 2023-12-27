use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    parse_input(input)
        .iter()
        .map(|dims| {
            let smallest = [dims[0] * dims[1], dims[1] * dims[2], dims[2] * dims[0]]
                .into_iter()
                .min()
                .unwrap();

            let area = 2 * dims[0] * dims[1] + 2 * dims[1] * dims[2] + 2 * dims[2] * dims[0];

            area + smallest
        })
        .sum::<u64>()
        .to_string()
}

fn part2(input: &str) -> String {
    parse_input(input)
        .iter()
        .map(|dims| {
            let volume = dims[0] * dims[1] * dims[2];

            let smallest = [
                2 * dims[0] + 2 * dims[1],
                2 * dims[1] + 2 * dims[2],
                2 * dims[2] + 2 * dims[0],
            ]
            .into_iter()
            .min()
            .unwrap();

            volume + smallest
        })
        .sum::<u64>()
        .to_string()
}

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    let re = regex::Regex::new(r"(\d+)x(\d+)x(\d+)").unwrap();

    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap_or_else(|| malformed("2015", "02"));

            let l = caps[1].parse::<u64>().unwrap();
            let w = caps[2].parse::<u64>().unwrap();
            let h = caps[3].parse::<u64>().unwrap();

            vec![l, w, h]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let cases = vec![("2x3x4", "58"), ("1x1x10", "43")];

        for (input, expected) in cases {
            assert_eq!(part1(input), expected);
        }
    }

    #[test]
    fn test_part2() {
        let cases = vec![("2x3x4", "34"), ("1x1x10", "14")];

        for (input, expected) in cases {
            assert_eq!(part2(input), expected);
        }
    }
}
