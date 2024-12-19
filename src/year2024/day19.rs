use std::collections::HashMap;

use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    let Input { patterns, designs } = Input::from(input);

    let mut solver = Solver::new(&patterns);

    designs
        .iter()
        .fold(0, |acc, design| {
            if solver.count(design) > 0 {
                acc + 1
            } else {
                acc
            }
        })
        .to_string()
}

fn part2(input: &str) -> String {
    let Input { patterns, designs } = Input::from(input);

    let mut solver = Solver::new(&patterns);

    designs
        .iter()
        .fold(0, |acc, design| acc + solver.count(design))
        .to_string()
}

#[derive(Debug, Clone)]
struct Input {
    patterns: Vec<Colors>,
    designs: Vec<Colors>,
}

impl Input {
    fn from(input: &str) -> Self {
        let (patterns_s, designs_s) = input
            .split_once("\n\n")
            .unwrap_or_else(|| malformed("2024", "18"));

        let patterns = patterns_s
            .split(", ")
            .map(|p| {
                Colors::new(
                    p.chars()
                        .map(|c| Color::from(c).unwrap_or_else(|| malformed("2024", "18")))
                        .collect(),
                )
            })
            .collect();

        let designs = designs_s
            .lines()
            .map(|p| {
                Colors::new(
                    p.chars()
                        .map(|c| Color::from(c).unwrap_or_else(|| malformed("2024", "18")))
                        .collect(),
                )
            })
            .collect();

        Self { patterns, designs }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl Color {
    fn from(c: char) -> Option<Self> {
        match c {
            'w' => Some(Self::White),
            'u' => Some(Self::Blue),
            'b' => Some(Self::Black),
            'r' => Some(Self::Red),
            'g' => Some(Self::Green),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Colors {
    vec: Vec<Color>,
}

impl Colors {
    fn new(vec: Vec<Color>) -> Self {
        Self { vec }
    }

    fn len(&self) -> usize {
        self.vec.len()
    }

    fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    fn strip_prefix(&self, prefix: &Colors) -> Option<Colors> {
        if self.len() >= prefix.len() && self.vec[..prefix.len()] == prefix.vec {
            Some(Colors::new(self.vec[prefix.len()..].to_vec()))
        } else {
            None
        }
    }
}

struct Solver<'a> {
    patterns: &'a [Colors],
    memo: HashMap<Colors, usize>,
}

impl<'a> Solver<'a> {
    fn new(patterns: &'a [Colors]) -> Self {
        Self {
            patterns,
            memo: HashMap::new(),
        }
    }

    fn count(&mut self, design: &Colors) -> usize {
        if let Some(&v) = self.memo.get(design) {
            return v;
        }

        let v = self.patterns.iter().fold(0, |acc, pattern| {
            acc + (if let Some(rest) = design.strip_prefix(pattern) {
                if rest.is_empty() {
                    1
                } else {
                    self.count(&rest)
                }
            } else {
                0
            })
        });

        self.memo.insert(design.clone(), v);

        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"
            ),
            "6"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"
            ),
            "16"
        );
    }
}
