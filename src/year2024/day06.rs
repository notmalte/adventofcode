use rayon::prelude::*;

use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    let map = Map::from(input);

    let mut current_pos = map.starting_pos;
    let mut current_dir = map.starting_dir;

    let mut visited = vec![vec![false; map.width]; map.height];
    visited[current_pos.1][current_pos.0] = true;

    loop {
        let target_pos = (
            current_pos.0 as isize + current_dir.vector().0,
            current_pos.1 as isize + current_dir.vector().1,
        );

        if !(0..map.width as isize).contains(&target_pos.0)
            || !(0..map.height as isize).contains(&target_pos.1)
        {
            break;
        }

        let target_pos = (target_pos.0 as usize, target_pos.1 as usize);

        if map.obstacles[target_pos.1][target_pos.0] {
            current_dir = current_dir.clockwise()
        } else {
            current_pos = target_pos;
            visited[current_pos.1][current_pos.0] = true;
        }
    }

    visited
        .iter()
        .fold(0, |acc, row| {
            acc + row.iter().fold(0, |acc, &v| if v { acc + 1 } else { acc })
        })
        .to_string()
}

fn part2(input: &str) -> String {
    let map = Map::from(input);

    map.obstacles
        .par_iter()
        .enumerate()
        .fold_with(0, |acc, (y, row)| {
            acc + row
                .par_iter()
                .enumerate()
                .fold_with(0, |acc, (x, &v)| {
                    if v || (x == map.starting_pos.0 && y == map.starting_pos.1) {
                        acc
                    } else {
                        let mut current_pos = map.starting_pos;
                        let mut current_dir = map.starting_dir;

                        let mut transitions = Vec::new();

                        loop {
                            let target_pos = (
                                current_pos.0 as isize + current_dir.vector().0,
                                current_pos.1 as isize + current_dir.vector().1,
                            );

                            if !(0..map.width as isize).contains(&target_pos.0)
                                || !(0..map.height as isize).contains(&target_pos.1)
                            {
                                return acc;
                            }

                            let target_pos = (target_pos.0 as usize, target_pos.1 as usize);

                            if transitions.contains(&(current_pos, target_pos)) {
                                return acc + 1;
                            }

                            if map.obstacles[target_pos.1][target_pos.0]
                                || (x == target_pos.0 && y == target_pos.1)
                            {
                                current_dir = current_dir.clockwise()
                            } else {
                                transitions.push((current_pos, target_pos));

                                current_pos = target_pos;
                            }
                        }
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

#[derive(Debug)]
struct Map {
    obstacles: Vec<Vec<bool>>,
    height: usize,
    width: usize,
    starting_pos: (usize, usize),
    starting_dir: Direction,
}

impl Map {
    fn from(input: &str) -> Self {
        let chars: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();

        let obstacles: Vec<Vec<_>> = chars
            .iter()
            .map(|row| row.iter().map(|&c| c == '#').collect())
            .collect();

        let (starting_pos, starting_dir) = chars
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(|(x, &c)| {
                    if c != '.' && c != '#' {
                        Some(((x, y), Direction::from(c)))
                    } else {
                        None
                    }
                })
            })
            .unwrap_or_else(|| malformed("2024", "06"));

        let height = obstacles.len();
        let width = obstacles[0].len();

        if obstacles.iter().any(|row| row.len() != width) {
            malformed("2024", "06")
        }

        Self {
            obstacles,
            height,
            width,
            starting_pos,
            starting_dir,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn from(c: char) -> Self {
        match c {
            '^' => Self::North,
            '>' => Self::East,
            'v' => Self::South,
            '<' => Self::West,
            _ => panic!("not a direction"),
        }
    }

    fn clockwise(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn vector(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
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
                "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            ),
            "41"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            ),
            "6"
        );
    }
}
