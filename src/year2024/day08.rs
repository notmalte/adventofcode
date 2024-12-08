use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    let map = Map::from(input);

    let mut antinodes = HashSet::new();

    for locs in map.antennas.values() {
        for v in locs.iter().permutations(2) {
            let a = (v[0].0 as isize, v[0].1 as isize);
            let b = (v[1].0 as isize, v[1].1 as isize);

            let antinode = (b.0 + (b.0 - a.0), b.1 + (b.1 - a.1));

            if (0..map.width as isize).contains(&antinode.0)
                && (0..map.height as isize).contains(&antinode.1)
            {
                antinodes.insert(antinode);
            }
        }
    }

    antinodes.len().to_string()
}

fn part2(input: &str) -> String {
    let map = Map::from(input);

    let mut antinodes = HashSet::new();

    for locs in map.antennas.values() {
        for v in locs.iter().permutations(2) {
            let a = (v[0].0 as isize, v[0].1 as isize);
            let b = (v[1].0 as isize, v[1].1 as isize);

            let d = (b.0 - a.0, b.1 - a.1);

            let mut antinode = (b.0, b.1);

            while (0..map.width as isize).contains(&antinode.0)
                && (0..map.height as isize).contains(&antinode.1)
            {
                antinodes.insert(antinode);
                antinode = (antinode.0 + d.0, antinode.1 + d.1)
            }
        }
    }

    antinodes.len().to_string()
}

#[derive(Debug)]
struct Map {
    antennas: HashMap<char, Vec<(usize, usize)>>,
    height: usize,
    width: usize,
}

impl Map {
    fn from(input: &str) -> Self {
        let mut antennas = HashMap::new();

        let height = input.lines().count();
        let width = input
            .lines()
            .next()
            .unwrap_or_else(|| malformed("2024", "08"))
            .chars()
            .count();

        for (y, l) in input.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                if c != '.' {
                    antennas.entry(c).or_insert(Vec::new()).push((x, y));
                }
            }
        }

        Self {
            antennas,
            height,
            width,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
            ),
            "14"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
            ),
            "34"
        );
    }
}
