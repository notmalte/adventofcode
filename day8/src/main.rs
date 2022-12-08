use std::{env, fs};

use itertools::Itertools;

fn main() {
    let current_dir = env::current_dir().unwrap();

    let input_file = current_dir.join("day8/input.txt");

    println!("Reading input from: {}", input_file.display());

    let input = fs::read_to_string(input_file).unwrap();

    println!("Input: {}", input);

    println!("Part 1: {}", part1(&input));

    println!("Part 2: {}", part2(&input));
}

fn build_grid(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

struct DirectionTrees {
    north: Vec<usize>,
    east: Vec<usize>,
    south: Vec<usize>,
    west: Vec<usize>,
}

fn get_trees_in_directions(grid: &Vec<Vec<usize>>, x: usize, y: usize) -> DirectionTrees {
    let width = grid[0].len();
    let height = grid.len();

    let trees_north: Vec<usize> = grid[0..y].iter().map(|row| row[x]).rev().collect();
    let trees_south: Vec<usize> = grid[y + 1..height].iter().map(|row| row[x]).collect();

    let trees_east: Vec<usize> = grid[y][x + 1..width].to_vec();
    let trees_west: Vec<usize> = grid[y][0..x].iter().copied().rev().collect();

    DirectionTrees {
        north: trees_north,
        east: trees_east,
        south: trees_south,
        west: trees_west,
    }
}

fn view_distance(trees_in_direction: &Vec<usize>, tree_height: usize) -> usize {
    match trees_in_direction.iter().position(|&t| t >= tree_height) {
        Some(i) => i + 1,
        None => trees_in_direction.len(),
    }
}

fn part1(input: &str) -> isize {
    let grid = build_grid(input);

    let width = grid[0].len();
    let height = grid.len();

    let mut visible_trees = 0;

    for (y, x) in (0..height).cartesian_product(0..width) {
        let tree = grid[y][x];

        let trees = get_trees_in_directions(&grid, x, y);

        if trees.north.iter().all(|&t| t < tree)
            || trees.east.iter().all(|&t| t < tree)
            || trees.south.iter().all(|&t| t < tree)
            || trees.west.iter().all(|&t| t < tree)
        {
            visible_trees += 1;
        }
    }

    visible_trees
}

fn part2(input: &str) -> usize {
    let grid = build_grid(input);

    let width = grid[0].len();
    let height = grid.len();

    let mut highest_scenic_score = 0;

    for (y, x) in (0..height).cartesian_product(0..width) {
        let tree = grid[y][x];

        let trees = get_trees_in_directions(&grid, x, y);

        let view_north = view_distance(&trees.north, tree);
        let view_east = view_distance(&trees.east, tree);
        let view_south = view_distance(&trees.south, tree);
        let view_west = view_distance(&trees.west, tree);

        let scenic_score = view_north * view_east * view_south * view_west;

        if scenic_score > highest_scenic_score {
            highest_scenic_score = scenic_score;
        }
    }

    highest_scenic_score
}
