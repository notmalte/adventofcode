use std::collections::HashSet;

use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    let map = Map::from(input);

    let mut sum = 0;

    for x in 0..map.width {
        for y in 0..map.height {
            if map.topo[y][x] == 0 {
                let paths = map.find_peaks(vec![(x, y)]);

                let mut set = HashSet::new();

                for path in paths {
                    set.insert(*path.last().unwrap());
                }

                sum += set.len();
            }
        }
    }

    sum.to_string()
}

fn part2(input: &str) -> String {
    let map = Map::from(input);

    let mut sum = 0;

    for x in 0..map.width {
        for y in 0..map.height {
            if map.topo[y][x] == 0 {
                let paths = map.find_peaks(vec![(x, y)]);

                sum += paths.len();
            }
        }
    }

    sum.to_string()
}

#[derive(Debug, Clone)]
struct Map {
    topo: Vec<Vec<u8>>,
    height: usize,
    width: usize,
}

impl Map {
    fn from(input: &str) -> Self {
        let topo: Vec<Vec<u8>> = input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap_or_else(|| malformed("2024", "08")) as u8)
                    .collect()
            })
            .collect();

        let height = topo.len();
        let width = topo[0].len();

        Self {
            topo,
            height,
            width,
        }
    }

    fn find_peaks(&self, path: Vec<(usize, usize)>) -> Vec<Vec<(usize, usize)>> {
        if path.len() == 10 {
            return vec![path];
        }

        let (x, y) = *path.last().unwrap();
        let v = self.topo[y][x];

        let mut paths = Vec::new();

        let candidates = [
            (x > 0).then(|| (x - 1, y)),
            (x < self.width - 1).then(|| (x + 1, y)),
            (y > 0).then(|| (x, y - 1)),
            (y < self.height - 1).then(|| (x, y + 1)),
        ];

        for (x, y) in candidates.into_iter().flatten() {
            if self.topo[y][x] == v + 1 {
                let mut new_path = path.clone();
                new_path.push((x, y));
                paths.append(&mut self.find_peaks(new_path));
            }
        }

        paths
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
            ),
            "36"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
            ),
            "81"
        );
    }
}
