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
        .map(|line| (line.parse::<i64>().unwrap() / 3) - 2)
        .sum::<i64>()
        .to_string()
}

fn part2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let mut sum = 0;

            let mut fuel = (line.parse::<i64>().unwrap() / 3) - 2;

            while fuel > 0 {
                sum += fuel;

                fuel = (fuel / 3) - 2;
            }

            sum
        })
        .sum::<i64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let cases = vec![
            ("12", "2"),
            ("14", "2"),
            ("1969", "654"),
            ("100756", "33583"),
        ];

        for (input, expected) in cases {
            assert_eq!(part1(input), expected);
        }
    }

    #[test]
    fn test_part2() {
        let cases = vec![("14", "2"), ("1969", "966"), ("100756", "50346")];

        for (input, expected) in cases {
            assert_eq!(part2(input), expected);
        }
    }
}
