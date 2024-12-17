use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
};

use pathfinding::prelude::dijkstra;

use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

const MOVE_COST: usize = 1;
const TURN_COST: usize = 1000;

fn part1(input: &str) -> String {
    let map = Map::from(input);

    let pos = Position {
        x: map.start.0,
        y: map.start.1,
        d: Direction::East,
    };

    let (_, optimum) = dijkstra(
        &pos,
        |p| map.successors(p),
        |p| p.x == map.end.0 && p.y == map.end.1,
    )
    .unwrap_or_else(|| malformed("2024", "16"));

    optimum.to_string()
}

fn part2(input: &str) -> String {
    let map = Map::from(input);

    let (optimal_paths, _) = map
        .optimal_paths()
        .unwrap_or_else(|| malformed("2024", "16"));

    let mut set = HashSet::new();

    for path in optimal_paths {
        for pos in path {
            set.insert((pos.x, pos.y));
        }
    }

    set.len().to_string()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Position {
    x: usize,
    y: usize,
    d: Direction,
}

impl Position {
    fn cw(&self) -> Self {
        Self {
            d: self.d.cw(),
            ..*self
        }
    }

    fn ccw(&self) -> Self {
        Self {
            d: self.d.ccw(),
            ..*self
        }
    }

    fn fwd(&self, map: &Map) -> Option<Self> {
        let (dx, dy) = self.d.v();
        let x = self.x as isize + dx;
        let y = self.y as isize + dy;

        ((0..map.width as isize).contains(&x) && (0..map.height as isize).contains(&y)).then(|| {
            let x = x as usize;
            let y = y as usize;
            Self { x, y, d: self.d }
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn cw(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn ccw(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn v(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    walls: Vec<Vec<bool>>,
    height: usize,
    width: usize,
    start: (usize, usize),
    end: (usize, usize),
}

impl Map {
    fn optimal_paths(&self) -> Option<(Vec<Vec<Position>>, usize)> {
        let mut visited = HashMap::new();
        let mut optimal_paths: Option<(Vec<Vec<Position>>, usize)> = None;

        let mut queue = BinaryHeap::new();
        queue.push(Reverse((
            0,
            vec![Position {
                x: self.start.0,
                y: self.start.1,
                d: Direction::East,
            }],
        )));

        while let Some(Reverse((current_cost, current_path))) = queue.pop() {
            if optimal_paths
                .as_ref()
                .is_some_and(|(_, best_cost)| current_cost > *best_cost)
            {
                continue;
            }

            let current_pos = current_path.last().unwrap();

            if current_pos.x == self.end.0 && current_pos.y == self.end.1 {
                if let Some((best_paths, best_cost)) = &mut optimal_paths {
                    match current_cost.cmp(best_cost) {
                        Ordering::Less => {
                            optimal_paths = Some((vec![current_path], current_cost));
                        }
                        Ordering::Equal => {
                            best_paths.push(current_path);
                        }
                        Ordering::Greater => {}
                    }
                } else {
                    optimal_paths = Some((vec![current_path], current_cost));
                }
                continue;
            }

            visited.insert(*current_pos, current_cost);

            for (next_pos, move_cost) in self.successors(current_pos) {
                let new_cost = current_cost + move_cost;

                if visited
                    .get(&next_pos)
                    .is_none_or(|&old_cost| old_cost >= new_cost)
                {
                    let mut new_path = current_path.clone();
                    new_path.push(next_pos);
                    queue.push(Reverse((new_cost, new_path)));
                }
            }
        }

        optimal_paths
    }

    fn successors(&self, pos: &Position) -> Vec<(Position, usize)> {
        let mut s = vec![(pos.cw(), TURN_COST), (pos.ccw(), TURN_COST)];

        if let Some(fwd) = pos.fwd(self) {
            if !self.walls[fwd.y][fwd.x] {
                s.push((fwd, MOVE_COST));
            }
        }

        s
    }

    fn from(input: &str) -> Self {
        let mut walls = Vec::new();
        let mut start = None;
        let mut end = None;

        for (y, row_s) in input.lines().enumerate() {
            let mut row = Vec::new();

            for (x, c) in row_s.chars().enumerate() {
                match c {
                    'S' => {
                        if start.is_some() {
                            malformed("2024", "16");
                        }

                        start = Some((x, y));
                        row.push(false);
                    }
                    'E' => {
                        if end.is_some() {
                            malformed("2024", "16");
                        }

                        end = Some((x, y));
                        row.push(false);
                    }
                    '#' => {
                        row.push(true);
                    }
                    '.' => {
                        row.push(false);
                    }
                    _ => malformed("2024", "16"),
                };
            }

            walls.push(row);
        }

        let height = walls.len();
        let width = walls[0].len();

        let Some(start) = start else {
            malformed("2024", "16")
        };

        let Some(end) = end else {
            malformed("2024", "16")
        };

        Self {
            walls,
            height,
            width,
            start,
            end,
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
                "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"
            ),
            "7036"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"
            ),
            "45"
        );
    }
}
