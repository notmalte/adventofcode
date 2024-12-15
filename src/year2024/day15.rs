use std::collections::HashSet;

use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    let input = Input::from(input);

    let mut map = input.map.clone();
    let mut position = input.position;

    for &d in &input.movements {
        map.try_move(&mut position, d);
    }

    map.gps_sum().to_string()
}

fn part2(input: &str) -> String {
    let input = Input::from(input);

    let mut map = Map {
        height: input.map.height,
        width: input.map.width * 2,
        elements: Vec::new(),
    };

    for e in &input.map.elements {
        let mut cells = Vec::new();

        for c in &e.cells {
            cells.push(Cell { x: 2 * c.x, y: c.y });
            cells.push(Cell {
                x: 2 * c.x + 1,
                y: c.y,
            });
        }

        map.elements.push(Element {
            kind: e.kind,
            cells,
        });
    }

    let mut position = Cell {
        x: 2 * input.position.x,
        y: input.position.y,
    };

    for &d in &input.movements {
        map.try_move(&mut position, d);
    }

    map.gps_sum().to_string()
}

#[derive(Debug, Clone)]
struct Map {
    height: usize,
    width: usize,
    elements: Vec<Element>,
}

impl Map {
    fn get(&self, cell: Cell) -> Option<&Element> {
        self.elements
            .iter()
            .find(|e| e.cells.iter().any(|&c| c == cell))
    }

    fn neighbor(&self, cell: Cell, direction: Direction) -> Option<Cell> {
        let (dx, dy) = match direction {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        };

        let (nx, ny) = (cell.x as isize + dx, cell.y as isize + dy);

        if (0..self.width as isize).contains(&nx) && (0..self.height as isize).contains(&ny) {
            Some(Cell {
                x: nx as usize,
                y: ny as usize,
            })
        } else {
            None
        }
    }

    fn try_move(&mut self, position: &mut Cell, direction: Direction) -> bool {
        let Some(neighbor) = self.neighbor(*position, direction) else {
            return false;
        };

        let mut cells_to_visit = vec![neighbor];
        let mut cells_visited = HashSet::new();
        let mut elements_will_get_pushed = HashSet::new();

        while let Some(cell) = cells_to_visit.pop() {
            if cells_visited.contains(&cell) {
                continue;
            }

            match self.get(cell) {
                None => {
                    cells_visited.insert(cell);
                }
                Some(element) => {
                    if element.kind == ElementKind::Wall {
                        return false;
                    }

                    for &element_cell in &element.cells {
                        let Some(element_cell_neighbor) = self.neighbor(element_cell, direction)
                        else {
                            return false;
                        };

                        cells_visited.insert(cell);
                        cells_to_visit.push(element_cell_neighbor);
                    }

                    elements_will_get_pushed.insert(element.clone());
                }
            }
        }

        self.elements = self
            .elements
            .iter()
            .map(|e| {
                if elements_will_get_pushed.contains(e) {
                    Element {
                        kind: e.kind,
                        cells: e
                            .cells
                            .iter()
                            .map(|c| self.neighbor(*c, direction).unwrap())
                            .collect(),
                    }
                } else {
                    e.clone()
                }
            })
            .collect();

        *position = neighbor;

        true
    }

    fn gps_sum(&self) -> usize {
        self.elements
            .iter()
            .filter(|e| e.kind == ElementKind::Box)
            .map(|e| {
                let c = e.cells.iter().min_by(|a, b| a.x.cmp(&b.x)).unwrap();

                100 * c.y + c.x
            })
            .sum()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Element {
    kind: ElementKind,
    cells: Vec<Cell>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ElementKind {
    Box,
    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cell {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
struct Input {
    map: Map,
    position: Cell,
    movements: Vec<Direction>,
}

impl Input {
    fn from(input: &str) -> Self {
        let (map_s, movements_s) = input
            .split_once("\n\n")
            .unwrap_or_else(|| malformed("2024", "15"));

        let height = map_s.lines().count();
        let width = map_s
            .lines()
            .next()
            .unwrap_or_else(|| malformed("2024", "15"))
            .chars()
            .count();

        let mut elements = Vec::new();
        let mut position = None;

        for (y, row_s) in map_s.lines().enumerate() {
            for (x, c) in row_s.chars().enumerate() {
                match c {
                    '.' => {}
                    '@' => {
                        if position.is_some() {
                            malformed("2024", "15");
                        }

                        position = Some(Cell { x, y });
                    }
                    'O' => {
                        elements.push(Element {
                            kind: ElementKind::Box,
                            cells: vec![Cell { x, y }],
                        });
                    }
                    '#' => {
                        elements.push(Element {
                            kind: ElementKind::Wall,
                            cells: vec![Cell { x, y }],
                        });
                    }
                    _ => malformed("2024", "15"),
                }
            }
        }

        let map = Map {
            height,
            width,
            elements,
        };

        let Some(position) = position else {
            malformed("2024", "15");
        };

        let movements = movements_s
            .replace("\n", "")
            .chars()
            .map(|c| match c {
                '^' => Direction::North,
                '>' => Direction::East,
                'v' => Direction::South,
                '<' => Direction::West,
                _ => malformed("2024", "15"),
            })
            .collect();

        Self {
            map,
            position,
            movements,
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
                "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
            ),
            "10092"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
            ),
            "9021"
        );
    }
}
