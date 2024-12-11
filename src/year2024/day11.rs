use std::collections::HashMap;

use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    Blinker::from(input).count_after_blinks(25).to_string()
}

fn part2(input: &str) -> String {
    Blinker::from(input).count_after_blinks(75).to_string()
}

struct Blinker {
    stones: Vec<usize>,
    memo: HashMap<(usize, usize), usize>,
}

impl Blinker {
    fn from(input: &str) -> Self {
        let stones = input
            .split_whitespace()
            .map(|s| {
                s.parse::<usize>()
                    .unwrap_or_else(|_| malformed("2024", "11"))
            })
            .collect();

        let memo = HashMap::new();

        Self { stones, memo }
    }

    fn count_after_blinks(&mut self, blinks: usize) -> usize {
        let mut sum = 0;

        for stone in self.stones.clone() {
            sum += self.count_after_blinks_recursive(stone, blinks);
        }

        sum
    }

    fn count_after_blinks_recursive(&mut self, stone: usize, blinks: usize) -> usize {
        if let Some(&count) = self.memo.get(&(stone, blinks)) {
            return count;
        }

        let count = if blinks <= 1 {
            Self::blink(stone).len()
        } else {
            let mut sum = 0;

            for sub_stone in Self::blink(stone) {
                sum += self.count_after_blinks_recursive(sub_stone, blinks - 1)
            }

            sum
        };

        self.memo.insert((stone, blinks), count);

        count
    }

    fn blink(stone: usize) -> Vec<usize> {
        if stone == 0 {
            vec![1]
        } else {
            let digits = stone.ilog10() + 1;

            if digits % 2 == 0 {
                let pow10 = 10usize.pow(digits / 2);
                vec![stone / pow10, stone % pow10]
            } else {
                vec![stone * 2024]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("125 17"), "55312");
    }
}
