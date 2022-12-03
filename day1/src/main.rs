use std::{env, fs};

fn main() {
    let current_dir = env::current_dir().unwrap();

    let input_file = current_dir.join("day1/input.txt");

    println!("Reading input from: {}", input_file.display());

    let input = fs::read_to_string(input_file).unwrap();

    println!("Part 1: {}", part1(&input));

    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> isize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|num| num.parse::<isize>().unwrap())
                .sum::<isize>()
        })
        .max()
        .unwrap()
}

fn part2(input: &str) -> isize {
    let mut vec: Vec<isize> = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|num| num.parse::<isize>().unwrap())
                .sum::<isize>()
        })
        .collect();

    vec.sort();

    vec.iter().rev().take(3).sum()
}
