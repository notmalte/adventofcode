use itertools::Itertools;
use std::{env, fs};

fn main() {
    let current_dir = env::current_dir().unwrap();

    let input_file = current_dir.join("day9/input.txt");

    println!("Reading input from: {}", input_file.display());

    let input = fs::read_to_string(input_file).unwrap();

    println!("Part 1: {}", part1(&input));

    println!("Part 2: {}", part2(&input));
}

fn parse_histories(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|history| {
            history
                .split_whitespace()
                .map(|val| val.parse::<isize>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

fn build_pyramid(history: Vec<isize>) -> Vec<Vec<isize>> {
    let mut pyramid = vec![history];

    while pyramid.last().unwrap().iter().any(|val| *val != 0) {
        let last = pyramid.last().unwrap();

        let mut next = vec![];

        for i in 0..last.len() - 1 {
            next.push(last[i + 1] - last[i])
        }

        pyramid.push(next);
    }

    pyramid
}

fn find_next_value(history: Vec<isize>) -> isize {
    let mut pyramid = build_pyramid(history);

    pyramid.last_mut().unwrap().push(0);

    for layer in (0..pyramid.len() - 1).rev() {
        let a = *pyramid[layer].last().unwrap();
        let b = *pyramid[layer + 1].last().unwrap();

        pyramid[layer].push(a + b);
    }

    *pyramid[0].last().unwrap()
}

fn find_previous_value(history: Vec<isize>) -> isize {
    let mut pyramid = build_pyramid(history);

    pyramid.last_mut().unwrap().insert(0, 0);

    for layer in (0..pyramid.len() - 1).rev() {
        let a = pyramid[layer][0];
        let b = pyramid[layer + 1][0];

        pyramid[layer].insert(0, a - b);
    }

    pyramid[0][0]
}

fn part1(input: &str) -> isize {
    let histories = parse_histories(input);

    histories
        .into_iter()
        .map(|history| find_next_value(history))
        .sum()
}

fn part2(input: &str) -> isize {
    let histories = parse_histories(input);

    histories
        .into_iter()
        .map(|history| find_previous_value(history))
        .sum()
}
