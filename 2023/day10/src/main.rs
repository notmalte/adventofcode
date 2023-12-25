use itertools::Itertools;
use std::{env, fs};

fn main() {
    let current_dir = env::current_dir().unwrap();

    let input_file = current_dir.join("day10/input.txt");

    println!("Reading input from: {}", input_file.display());

    let input = fs::read_to_string(input_file).unwrap();

    println!("Part 1: {}", part1(&input));

    println!("Part 2: {}", part2(&input));
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Ground,
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Start,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn from_xy(dx: isize, dy: isize) -> Direction {
        match (dx, dy) {
            (0, -1) => Direction::North,
            (0, 1) => Direction::South,
            (1, 0) => Direction::East,
            (-1, 0) => Direction::West,
            _ => panic!("Invalid direction: ({}, {})", dx, dy),
        }
    }
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '.' => Tile::Ground,
            '|' => Tile::NorthSouth,
            '-' => Tile::EastWest,
            'L' => Tile::NorthEast,
            'J' => Tile::NorthWest,
            'F' => Tile::SouthEast,
            '7' => Tile::SouthWest,
            'S' => Tile::Start,
            _ => panic!("Unknown tile: {}", c),
        }
    }

    fn north(&self) -> bool {
        match self {
            Tile::NorthSouth | Tile::NorthEast | Tile::NorthWest | Tile::Start => true,
            _ => false,
        }
    }

    fn south(&self) -> bool {
        match self {
            Tile::NorthSouth | Tile::SouthEast | Tile::SouthWest | Tile::Start => true,
            _ => false,
        }
    }

    fn east(&self) -> bool {
        match self {
            Tile::EastWest | Tile::NorthEast | Tile::SouthEast | Tile::Start => true,
            _ => false,
        }
    }

    fn west(&self) -> bool {
        match self {
            Tile::EastWest | Tile::NorthWest | Tile::SouthWest | Tile::Start => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
struct TileGrid {
    tiles: Vec<Vec<Tile>>,
}

impl TileGrid {
    fn from_str(input: &str) -> TileGrid {
        TileGrid {
            tiles: input
                .lines()
                .map(|line| line.chars().map(|c| Tile::from_char(c)).collect())
                .collect(),
        }
    }

    fn connects(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        let (fx, fy) = from;
        let (tx, ty) = to;

        let dx = tx as isize - fx as isize;
        let dy = ty as isize - fy as isize;

        if !((dx.abs() == 1 && fy == ty) || (dy.abs() == 1 && fx == tx)) {
            return false;
        }

        let from_tile = self[(fx, fy)];
        let to_tile = self[(tx, ty)];

        if dx == 0 && dy == -1 && from_tile.north() && to_tile.south() {
            return true;
        }

        if dx == 0 && dy == 1 && from_tile.south() && to_tile.north() {
            return true;
        }

        if dx == 1 && dy == 0 && from_tile.east() && to_tile.west() {
            return true;
        }

        if dx == -1 && dy == 0 && from_tile.west() && to_tile.east() {
            return true;
        }

        false
    }

    fn next(&self, previous: (usize, usize), current: (usize, usize)) -> (usize, usize) {
        let (px, py) = previous;
        let (cx, cy) = current;

        let dx = cx as isize - px as isize;
        let dy = cy as isize - py as isize;

        if !self.connects(previous, current) {
            panic!("Tiles do not connect: {:?} -> {:?}", previous, current);
        }

        let previous_tile = self[previous];
        let current_tile = self[current];

        if dx == 0 && dy == -1 && previous_tile.north() && current_tile.south() {
            return match current_tile {
                Tile::NorthSouth => (cx, cy - 1),
                Tile::SouthEast => (cx + 1, cy),
                Tile::SouthWest => (cx - 1, cy),
                _ => panic!("Invalid tile: {:?}", current_tile),
            };
        }

        if dx == 0 && dy == 1 && previous_tile.south() && current_tile.north() {
            return match current_tile {
                Tile::NorthSouth => (cx, cy + 1),
                Tile::NorthEast => (cx + 1, cy),
                Tile::NorthWest => (cx - 1, cy),
                _ => panic!("Invalid tile: {:?}", current_tile),
            };
        }

        if dx == 1 && dy == 0 && previous_tile.east() && current_tile.west() {
            return match current_tile {
                Tile::EastWest => (cx + 1, cy),
                Tile::NorthWest => (cx, cy - 1),
                Tile::SouthWest => (cx, cy + 1),
                _ => panic!("Invalid tile: {:?}", current_tile),
            };
        }

        if dx == -1 && dy == 0 && previous_tile.west() && current_tile.east() {
            return match current_tile {
                Tile::EastWest => (cx - 1, cy),
                Tile::NorthEast => (cx, cy - 1),
                Tile::SouthEast => (cx, cy + 1),
                _ => panic!("Invalid tile: {:?}", current_tile),
            };
        }

        panic!("Invalid tiles: {:?} -> {:?}", previous, current);
    }

    fn find_start(&self) -> (usize, usize) {
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if *tile == Tile::Start {
                    return (x, y);
                }
            }
        }

        panic!("No start tile found");
    }

    fn find_loop(&self) -> Vec<(usize, usize)> {
        let start = self.find_start();

        let max_x = self.tiles[start.1].len() - 1;
        let max_y = self.tiles.len() - 1;

        let first;

        if start.1 > 0 && self.connects(start, (start.0, start.1 - 1)) {
            first = (start.0, start.1 - 1);
        } else if start.1 < max_y && self.connects(start, (start.0, start.1 + 1)) {
            first = (start.0, start.1 + 1);
        } else if start.0 > 0 && self.connects(start, (start.0 - 1, start.1)) {
            first = (start.0 - 1, start.1);
        } else if start.0 < max_x && self.connects(start, (start.0 + 1, start.1)) {
            first = (start.0 + 1, start.1);
        } else {
            panic!("Start tile does not connect to any other tile");
        }

        let mut loop_vec = vec![start];

        let mut previous = start;
        let mut current = first;

        loop {
            if current == start {
                return loop_vec;
            }

            loop_vec.push(current);

            let next = self.next(previous, current);

            previous = current;
            current = next;
        }
    }

    fn find_fitting_start_tile(&self, loop_vec: &Vec<(usize, usize)>) -> Tile {
        let (sx, sy) = loop_vec[0];
        let (nx, ny) = loop_vec[1];
        let (ex, ey) = loop_vec[loop_vec.len() - 1];

        let dir_start_to_next =
            Direction::from_xy(nx as isize - sx as isize, ny as isize - sy as isize);

        let dir_start_to_end =
            Direction::from_xy(ex as isize - sx as isize, ey as isize - sy as isize);

        match (dir_start_to_next, dir_start_to_end) {
            (Direction::North, Direction::East) => Tile::NorthEast,
            (Direction::North, Direction::South) => Tile::NorthSouth,
            (Direction::North, Direction::West) => Tile::NorthWest,
            (Direction::East, Direction::North) => Tile::NorthEast,
            (Direction::East, Direction::South) => Tile::SouthEast,
            (Direction::East, Direction::West) => Tile::EastWest,
            (Direction::South, Direction::North) => Tile::NorthSouth,
            (Direction::South, Direction::East) => Tile::SouthEast,
            (Direction::South, Direction::West) => Tile::SouthWest,
            (Direction::West, Direction::North) => Tile::NorthWest,
            (Direction::West, Direction::East) => Tile::EastWest,
            (Direction::West, Direction::South) => Tile::SouthWest,
            _ => panic!(
                "Invalid directions: {:?} -> {:?}",
                dir_start_to_next, dir_start_to_end
            ),
        }
    }

    fn simplify(&mut self) {
        let loop_vec = self.find_loop();

        let start_tile = self.find_fitting_start_tile(&loop_vec);

        let mut new_tiles = self
            .tiles
            .iter()
            .map(|row| row.iter().map(|_| Tile::Ground).collect_vec())
            .collect_vec();

        for (loop_i, loop_tile) in loop_vec.iter().enumerate() {
            if loop_i == 0 {
                new_tiles[loop_tile.1][loop_tile.0] = start_tile;
            } else {
                new_tiles[loop_tile.1][loop_tile.0] = self[*loop_tile]
            }
        }

        self.tiles = new_tiles;
    }
}

impl std::ops::Index<(usize, usize)> for TileGrid {
    type Output = Tile;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.tiles[index.1][index.0]
    }
}

fn part1(input: &str) -> isize {
    let grid = TileGrid::from_str(input);

    let loop_vec = grid.find_loop();

    loop_vec.len() as isize / 2
}

fn part2(input: &str) -> isize {
    let mut grid = TileGrid::from_str(input);

    grid.simplify();

    let mut area = 0;

    let mut inside;

    for row in grid.tiles.iter() {
        inside = false;

        for tile in row.iter() {
            match tile {
                Tile::Ground => {
                    if inside {
                        area += 1;
                    }
                }
                Tile::Start => panic!("Start tile should not be in simplified grid"),
                _ => {
                    if tile.south() {
                        inside = !inside;
                    }
                }
            }
        }
    }

    area
}
