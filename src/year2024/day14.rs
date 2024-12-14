use regex::Regex;

use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

const WIDTH: usize = 101;
const HEIGHT: usize = 103;

const MAX_TRIES: usize = 100_000;
const AREA_THRESHOLD: usize = 100;

fn part1(input: &str) -> String {
    let (a, b, c, d) = parse_input(input)
        .into_iter()
        .map(|r| advance(&r, 100))
        .fold((0, 0, 0, 0), |(a, b, c, d), r| {
            if r.p.0 == WIDTH / 2 || r.p.1 == HEIGHT / 2 {
                (a, b, c, d)
            } else {
                match (r.p.0 < WIDTH / 2, r.p.1 < HEIGHT / 2) {
                    (true, true) => (a + 1, b, c, d),
                    (false, true) => (a, b + 1, c, d),
                    (true, false) => (a, b, c + 1, d),
                    (false, false) => (a, b, c, d + 1),
                }
            }
        });

    (a * b * c * d).to_string()
}

fn part2(input: &str) -> String {
    let robots = parse_input(input);

    for s in 0..MAX_TRIES {
        let mut grid = [[false; WIDTH]; HEIGHT];

        for r in robots.iter().map(|r| advance(r, s)) {
            grid[r.p.1][r.p.0] = true;
        }

        let mut regions: Vec<Vec<(usize, usize)>> = Vec::new();

        for (y, row) in grid.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                if !v || regions.iter().any(|r| r.contains(&(x, y))) {
                    continue;
                }

                let mut cells = Vec::new();
                let mut poi = vec![(x, y)];

                while let Some((px, py)) = poi.pop() {
                    if !grid[py][px] {
                        continue;
                    }

                    if cells.contains(&(px, py)) {
                        continue;
                    }

                    if regions.iter().any(|r| r.contains(&(px, py))) {
                        continue;
                    }

                    cells.push((px, py));

                    let candidates = [
                        (px > 0).then(|| (px - 1, py)),
                        (px < WIDTH - 1).then(|| (px + 1, py)),
                        (py > 0).then(|| (px, py - 1)),
                        (py < HEIGHT - 1).then(|| (px, py + 1)),
                    ];

                    poi.extend(candidates.iter().flatten());
                }

                regions.push(cells);
            }
        }

        let biggest_region = regions.iter().map(|r| r.len()).max().unwrap_or(0);

        if biggest_region > AREA_THRESHOLD {
            return s.to_string();
        }
    }

    malformed("2024", "14")
}

fn advance(r: &Robot, s: usize) -> Robot {
    let x = (r.p.0 as isize + s as isize * r.v.0).rem_euclid(WIDTH as isize) as usize;
    let y = (r.p.1 as isize + s as isize * r.v.1).rem_euclid(HEIGHT as isize) as usize;

    Robot { p: (x, y), v: r.v }
}

#[derive(Debug, Clone)]
struct Robot {
    p: (usize, usize),
    v: (isize, isize),
}

fn parse_input(input: &str) -> Vec<Robot> {
    let re = Regex::new(r"^p=(\d+),(\d+) v=(-?\d+),(-?\d+)$").unwrap();

    input
        .lines()
        .map(|l| {
            let caps = re.captures(l).unwrap_or_else(|| malformed("2024", "14"));

            Robot {
                p: (caps[1].parse().unwrap(), caps[2].parse().unwrap()),
                v: (caps[3].parse().unwrap(), caps[4].parse().unwrap()),
            }
        })
        .collect()
}
