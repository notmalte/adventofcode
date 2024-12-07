use itertools::Itertools;
use rayon::prelude::*;

use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    parse_input(input)
        .par_iter()
        .filter_map(|(result, nums)| {
            has_solution(&[Operator::Add, Operator::Multiply], *result, nums).then_some(result)
        })
        .sum::<usize>()
        .to_string()
}

fn part2(input: &str) -> String {
    parse_input(input)
        .par_iter()
        .filter_map(|(result, nums)| {
            has_solution(
                &[Operator::Add, Operator::Multiply, Operator::Concat],
                *result,
                nums,
            )
            .then_some(result)
        })
        .sum::<usize>()
        .to_string()
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

fn has_solution(ops: &[Operator], result: usize, nums: &[usize]) -> bool {
    itertools::repeat_n(ops.iter(), nums.len() - 1)
        .multi_cartesian_product()
        .any(|ops| {
            let mut n = nums[0];

            for i in 1..nums.len() {
                match ops[i - 1] {
                    Operator::Add => n += nums[i],
                    Operator::Multiply => n *= nums[i],
                    Operator::Concat => {
                        n = (n * 10usize.pow(if nums[i] == 0 {
                            1
                        } else {
                            nums[i].ilog10() + 1
                        })) + nums[i]
                    }
                }
            }

            n == result
        })
}

fn parse_input(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .map(|l| {
            let (result, nums) = l.split_once(":").unwrap_or_else(|| malformed("2024", "07"));

            (
                result.parse().unwrap_or_else(|_| malformed("2024", "07")),
                nums.split_whitespace()
                    .map(|n| n.parse().unwrap_or_else(|_| malformed("2024", "07")))
                    .collect(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
            ),
            "3749"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
            ),
            "11387"
        );
    }
}
