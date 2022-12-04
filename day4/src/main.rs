use std::{env, fs};

fn main() {
    let current_dir = env::current_dir().unwrap();

    let input_file = current_dir.join("day4/input.txt");

    println!("Reading input from: {}", input_file.display());

    let input = fs::read_to_string(input_file).unwrap();

    println!("Input: {}", input);

    println!("Part 1: {}", part1(&input));

    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> isize {
    let mut score = 0;

    input.lines().for_each(|line| {
        let vec: Vec<isize> = line
            .split(|c| c == ',' || c == '-')
            .map(|s| s.parse::<isize>().unwrap())
            .collect();

        debug_assert_eq!(vec.len(), 4);

        let start1 = vec[0];
        let end1 = vec[1];

        let start2 = vec[2];
        let end2 = vec[3];

        if start1 >= start2 && end1 <= end2 || start1 <= start2 && end1 >= end2 {
            score += 1;
        }
    });

    score
}

fn part2(input: &str) -> isize {
    let mut score = 0;

    input.lines().for_each(|line| {
        let vec: Vec<isize> = line
            .split(|c| c == ',' || c == '-')
            .map(|s| s.parse::<isize>().unwrap())
            .collect();

        debug_assert_eq!(vec.len(), 4);

        let start1 = vec[0];
        let end1 = vec[1];

        let start2 = vec[2];
        let end2 = vec[3];

        if start1 <= end2 && start2 <= end1 {
            score += 1;
        }
    });

    score
}
