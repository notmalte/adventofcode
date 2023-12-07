use itertools::Itertools;
use std::{env, fs};

fn main() {
    let current_dir = env::current_dir().unwrap();

    let input_file = current_dir.join("day6/input.txt");

    println!("Reading input from: {}", input_file.display());

    let input = fs::read_to_string(input_file).unwrap();

    println!("Part 1: {}", part1(&input));

    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> isize {
    let (times, distances) = input.split_once("\n").unwrap();

    let times = times
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse::<isize>().unwrap());

    let distances = distances
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse::<isize>().unwrap());

    let races = times.zip(distances).collect_vec();

    let mut product = 1;

    for (time, distance) in races {
        let mut count = 0;

        for t in 1..time {
            let d = (time - t) * (t);

            if d > distance {
                count += 1;
            }
        }

        product *= count;
    }

    product
}

fn part2(input: &str) -> isize {
    let (times, distances) = input.split_once("\n").unwrap();

    let time = times
        .split_once(": ")
        .unwrap()
        .1
        .replace(" ", "")
        .parse::<isize>()
        .unwrap();

    let distance = distances
        .split_once(": ")
        .unwrap()
        .1
        .replace(" ", "")
        .parse::<isize>()
        .unwrap();

    let mut count = 0;

    for t in 1..time {
        let d = (time - t) * (t);

        if d > distance {
            count += 1;
        }
    }

    count
}
