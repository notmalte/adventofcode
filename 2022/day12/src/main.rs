use std::{env, fs};

use pathfinding::prelude::dijkstra;

fn main() {
    let current_dir = env::current_dir().unwrap();

    let input_file = current_dir.join("day12/input.txt");

    println!("Reading input from: {}", input_file.display());

    let input = fs::read_to_string(input_file).unwrap();

    println!("Part 1: {}", part1(&input));

    println!("Part 2: {}", part2(&input));
}

const START_CHAR: char = 'S';
const END_CHAR: char = 'E';

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Position(usize, usize);

struct Map {
    values: Vec<Vec<isize>>,
    width: usize,
    height: usize,
    start: Position,
    end: Position,
}

impl Map {
    fn from_input(input: &str) -> Map {
        let mut start = Option::None;
        let mut end = Option::None;

        let map_values: Vec<Vec<isize>> = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == START_CHAR {
                            start = Option::Some(Position(x, y));
                            1
                        } else if c == END_CHAR {
                            end = Option::Some(Position(x, y));
                            26
                        } else {
                            (c as isize) - 96
                        }
                    })
                    .collect()
            })
            .collect();

        let map_height = map_values.len();
        let map_width = map_values[0].len();

        Map {
            values: map_values,
            width: map_width,
            height: map_height,
            start: start.unwrap(),
            end: end.unwrap(),
        }
    }

    fn height_at(&self, position: &Position) -> isize {
        self.values[position.1][position.0]
    }
}

impl Position {
    fn neighbors(&self, map: &Map) -> Vec<Position> {
        let mut neighbors = vec![];

        if self.1 > 0 {
            neighbors.push(Position(self.0, self.1 - 1));
        }

        if self.1 < map.height - 1 {
            neighbors.push(Position(self.0, self.1 + 1));
        }

        if self.0 > 0 {
            neighbors.push(Position(self.0 - 1, self.1));
        }

        if self.0 < map.width - 1 {
            neighbors.push(Position(self.0 + 1, self.1));
        }

        neighbors
    }

    fn successors_up(&self, map: &Map) -> Vec<(Position, usize)> {
        let mut successors = vec![];

        let current_height = map.height_at(self);

        for neighbor in self.neighbors(map) {
            let neighbor_height = map.height_at(&neighbor);

            if neighbor_height <= current_height + 1 {
                successors.push((neighbor, 1));
            }
        }

        successors
    }

    fn successors_down(&self, map: &Map) -> Vec<(Position, usize)> {
        let mut successors = vec![];

        let current_height = map.height_at(self);

        for neighbor in self.neighbors(map) {
            let neighbor_height = map.height_at(&neighbor);

            if neighbor_height >= current_height - 1 {
                successors.push((neighbor, 1));
            }
        }

        successors
    }
}

fn part1(input: &str) -> usize {
    let map = Map::from_input(input);

    let result = dijkstra(&map.start, |p| p.successors_up(&map), |p| *p == map.end);

    result.unwrap().1
}

fn part2(input: &str) -> usize {
    let map = Map::from_input(input);

    let result = dijkstra(
        &map.end,
        |p| p.successors_down(&map),
        |p| map.height_at(p) == 1,
    );

    result.unwrap().1
}
