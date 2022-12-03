use std::{env, fs};

fn main() {
    let current_dir = env::current_dir().unwrap();

    let input_file = current_dir.join("day0/input.txt"); // <-- Adjust this

    println!("Reading input from: {}", input_file.display());

    let input = fs::read_to_string(input_file).unwrap();

    println!("Part 1: {}", part1(&input));

    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> isize {
    println!("Input: {}", input);

    todo!();
}

fn part2(input: &str) -> isize {
    println!("Input: {}", input);

    todo!();
}
