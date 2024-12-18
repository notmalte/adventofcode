use std::collections::HashSet;

use itertools::Itertools;
use pathfinding::prelude::dijkstra;

use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

const WIDTH: usize = 71;
const HEIGHT: usize = 71;
const COUNT: usize = 1024;

fn part1(input: &str) -> String {
    let parsed = parse_input(input);

    let mut corrupted = vec![vec![false; WIDTH]; HEIGHT];

    for &(x, y) in &parsed[..COUNT] {
        corrupted[y][x] = true;
    }

    let (_, cost) = find_path(&corrupted, WIDTH, HEIGHT).unwrap_or_else(|| malformed("2024", "16"));

    cost.to_string()
}

fn part2(input: &str) -> String {
    let parsed = parse_input(input);

    let mut corrupted = vec![vec![false; WIDTH]; HEIGHT];

    for &(x, y) in &parsed[..COUNT] {
        corrupted[y][x] = true;
    }

    let (path, _) = find_path(&corrupted, WIDTH, HEIGHT).unwrap_or_else(|| malformed("2024", "16"));
    let mut path_set: HashSet<_> = path.into_iter().collect();

    for &(x, y) in &parsed[COUNT..] {
        corrupted[y][x] = true;

        if !path_set.contains(&(x, y)) {
            continue;
        }

        let Some((path, _)) = find_path(&corrupted, WIDTH, HEIGHT) else {
            return format!("{},{}", x, y);
        };

        path_set = path.into_iter().collect();
    }

    malformed("2024", "16")
}

fn find_path(
    corrupted: &Vec<Vec<bool>>,
    width: usize,
    height: usize,
) -> Option<(Vec<(usize, usize)>, usize)> {
    let start = (0, 0);
    let end = (width - 1, height - 1);

    dijkstra(
        &start,
        |&(x, y)| {
            let candidates = [
                (x > 0).then(|| (x - 1, y)),
                (x < width - 1).then(|| (x + 1, y)),
                (y > 0).then(|| (x, y - 1)),
                (y < height - 1).then(|| (x, y + 1)),
            ];

            candidates
                .into_iter()
                .flatten()
                .filter(|&(x, y)| !corrupted[y][x])
                .map(|p| (p, 1usize))
                .collect_vec()
        },
        |&p| p == end,
    )
}

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap_or_else(|| malformed("2024", "18"));
            (
                x.parse().unwrap_or_else(|_| malformed("2024", "18")),
                y.parse().unwrap_or_else(|_| malformed("2024", "18")),
            )
        })
        .collect()
}
