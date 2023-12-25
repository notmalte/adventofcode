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

    fn find_start(&self) -> Option<(usize, usize)> {
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if *tile == Tile::Start {
                    return Some((x, y));
                }
            }
        }

        None
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

    let start = grid.find_start().unwrap();

    let first;

    if grid.connects(start, (start.0, start.1 - 1)) {
        first = (start.0, start.1 - 1);
    } else if grid.connects(start, (start.0, start.1 + 1)) {
        first = (start.0, start.1 + 1);
    } else if grid.connects(start, (start.0 - 1, start.1)) {
        first = (start.0 - 1, start.1);
    } else if grid.connects(start, (start.0 + 1, start.1)) {
        first = (start.0 + 1, start.1);
    } else {
        panic!("Start does not connect to any other tile");
    }

    let mut steps = 1;

    let mut previous = start;
    let mut current = first;

    loop {
        if current == start {
            break;
        }

        let next = grid.next(previous, current);

        previous = current;
        current = next;

        steps += 1;
    }

    steps / 2
}

fn part2(input: &str) -> isize {
    todo!();
}
