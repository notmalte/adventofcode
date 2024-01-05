use itertools::iproduct;

use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    let mut grid = Grid::from_str(input);

    for _ in 0..100 {
        grid.tick();
    }

    grid.count().to_string()
}

fn part2(input: &str) -> String {
    let mut grid = Grid::from_str(input);

    grid.turn_on_corners();

    for _ in 0..100 {
        grid.tick();
        grid.turn_on_corners();
    }

    grid.count().to_string()
}

#[derive(Debug, Clone)]
struct Grid {
    lights: Vec<Vec<bool>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn from_str(input: &str) -> Self {
        let mut lights = vec![];

        let mut cols = 0;

        for line in input.lines() {
            let mut row = vec![];

            for c in line.chars() {
                row.push(c == '#');
            }

            if cols == 0 {
                cols = row.len();
            } else if cols != row.len() {
                malformed("2015", "18");
            }

            lights.push(row);
        }

        let rows = lights.len();

        Self { lights, rows, cols }
    }

    fn neighbors(&self, x: usize, y: usize) -> usize {
        let y_min = y.saturating_sub(1);
        let y_max = (y + 1).min(self.rows - 1);

        let x_min = x.saturating_sub(1);
        let x_max = (x + 1).min(self.cols - 1);

        iproduct!(x_min..=x_max, y_min..=y_max)
            .filter(|&(xi, yi)| !(xi == x && yi == y))
            .filter(|&(xi, yi)| self.lights[yi][xi])
            .count()
    }

    fn tick(&mut self) {
        let new_lights = self
            .lights
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, &light)| {
                        let neighbors = self.neighbors(x, y);

                        if light {
                            neighbors == 2 || neighbors == 3
                        } else {
                            neighbors == 3
                        }
                    })
                    .collect()
            })
            .collect();

        self.lights = new_lights;
    }

    fn count(&self) -> usize {
        self.lights.iter().flatten().filter(|&&light| light).count()
    }

    fn turn_on_corners(&mut self) {
        self.lights[0][0] = true;
        self.lights[0][self.cols - 1] = true;
        self.lights[self.rows - 1][0] = true;
        self.lights[self.rows - 1][self.cols - 1] = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tick() {
        let initial = ".#.#.#\n...##.\n#....#\n..#...\n#.#..#\n####..";

        let mut grid = Grid::from_str(initial);

        for _ in 0..4 {
            grid.tick();
        }

        assert_eq!(grid.count(), 4);
    }

    #[test]
    fn test_tick_corners() {
        let initial = "##.#.#\n...##.\n#....#\n..#...\n#.#..#\n####.#";

        let mut grid = Grid::from_str(initial);

        grid.turn_on_corners();

        for _ in 0..5 {
            grid.tick();
            grid.turn_on_corners();
        }

        assert_eq!(grid.count(), 17);
    }
}
