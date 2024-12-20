use std::collections::{HashMap, VecDeque};

use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    let (map, start, end) = Map::from(input);

    map.count_cheats(&start, &end, 2, 100).to_string()
}

fn part2(input: &str) -> String {
    let (map, start, end) = Map::from(input);

    map.count_cheats(&start, &end, 20, 100).to_string()
}

#[derive(Debug, Clone)]
struct Map {
    walls: Vec<Vec<bool>>,
    height: usize,
    width: usize,
}

impl Map {
    fn from(input: &str) -> (Self, (usize, usize), (usize, usize)) {
        let mut walls = Vec::new();
        let mut start = None;
        let mut end = None;

        for (y, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                row.push(c == '#');

                if c == 'S' {
                    if start.is_some() {
                        malformed("2024", "20");
                    }
                    start = Some((x, y));
                } else if c == 'E' {
                    if end.is_some() {
                        malformed("2024", "20");
                    }
                    end = Some((x, y));
                }
            }
            walls.push(row);
        }

        let Some(start) = start else {
            malformed("2024", "20");
        };

        let Some(end) = end else {
            malformed("2024", "20");
        };

        let height = walls.len();
        let width = walls[0].len();

        (
            Self {
                walls,
                height,
                width,
            },
            start,
            end,
        )
    }

    fn neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        let candidates = [
            (x > 0).then(|| (x - 1, y)),
            (x < self.width - 1).then(|| (x + 1, y)),
            (y > 0).then(|| (x, y - 1)),
            (y < self.height - 1).then(|| (x, y + 1)),
        ];

        candidates.into_iter().flatten()
    }

    fn neighbors_range(
        &self,
        x: usize,
        y: usize,
        min_distance: usize,
        max_distance: usize,
    ) -> impl Iterator<Item = ((usize, usize), usize)> + '_ {
        let y_start = y.saturating_sub(max_distance);
        let y_end = if y + max_distance < self.height {
            y + max_distance
        } else {
            self.height - 1
        };

        (y_start..=y_end).flat_map(move |yc| {
            let remaining = max_distance - y.abs_diff(yc);

            let x_start = x.saturating_sub(remaining);
            let x_end = if x + remaining < self.width {
                x + remaining
            } else {
                self.width - 1
            };

            (x_start..=x_end).filter_map(move |xc| {
                let distance = x.abs_diff(xc) + y.abs_diff(yc);
                if distance < min_distance {
                    None
                } else {
                    Some(((xc, yc), distance))
                }
            })
        })
    }

    fn distances(&self, from: &(usize, usize)) -> HashMap<(usize, usize), usize> {
        let mut distances = HashMap::new();

        let mut queue = VecDeque::new();
        queue.push_back((*from, 0));

        while let Some(((x, y), dist)) = queue.pop_front() {
            distances.insert((x, y), dist);

            for (xs, ys) in self.neighbors(x, y) {
                if !self.walls[ys][xs] && !distances.contains_key(&(xs, ys)) {
                    queue.push_back(((xs, ys), dist + 1));
                }
            }
        }

        distances
    }

    fn count_cheats(
        &self,
        start: &(usize, usize),
        end: &(usize, usize),
        max_distance: usize,
        min_savings: usize,
    ) -> usize {
        let dist_from_start = self.distances(start);
        let dist_from_end = self.distances(end);

        let opt = dist_from_start[end];

        let mut count = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                if self.walls[y][x] {
                    continue;
                }

                let ds = dist_from_start[&(x, y)];

                for ((xn, yn), dn) in self
                    .neighbors_range(x, y, 2, max_distance)
                    .filter(|&((xn, yn), _)| !self.walls[yn][xn])
                {
                    let de = dist_from_end[&(xn, yn)];

                    let d = ds + dn + de;

                    if d < opt && opt - d >= min_savings {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}
