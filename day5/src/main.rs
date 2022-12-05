use std::{env, fs};

fn build_initial_stacks_from_header(header: &str) -> Vec<Vec<char>> {
    let mut iter = header.lines().rev();

    let stack_count = iter.next().unwrap().split_ascii_whitespace().count();

    let mut stacks: Vec<Vec<char>> = vec![vec![]; stack_count];

    iter.for_each(|line| {
        stacks.iter_mut().enumerate().for_each(|(i, stack)| {
            let c_index = i * 4 + 1;
            let c = line.chars().nth(c_index).unwrap();
            if c.is_ascii_alphabetic() {
                stack.push(c);
            }
        });
    });

    stacks
}

fn main() {
    let current_dir = env::current_dir().unwrap();

    let input_file = current_dir.join("day5/input.txt");

    println!("Reading input from: {}", input_file.display());

    let input = fs::read_to_string(input_file).unwrap();

    println!("Input: {}", input);

    println!("Part 1: {}", part1(&input));

    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let (header, body) = input.split_once("\n\n").unwrap();

    let mut stacks = build_initial_stacks_from_header(header);

    body.lines().for_each(|line| {
        let split: Vec<&str> = line.split_ascii_whitespace().collect();

        let (amount, from, to) = (
            split[1].parse::<usize>().unwrap(),
            split[3].parse::<usize>().unwrap(),
            split[5].parse::<usize>().unwrap(),
        );

        for _ in 0..amount {
            let c = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(c);
        }
    });

    stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
}

fn part2(input: &str) -> String {
    let (header, body) = input.split_once("\n\n").unwrap();

    let mut stacks = build_initial_stacks_from_header(header);

    body.lines().for_each(|line| {
        let split: Vec<&str> = line.split_ascii_whitespace().collect();

        let (amount, from, to) = (
            split[1].parse::<usize>().unwrap(),
            split[3].parse::<usize>().unwrap(),
            split[5].parse::<usize>().unwrap(),
        );

        let tail_index = stacks[from - 1].len() - amount;

        let tail = stacks[from - 1].drain(tail_index..).collect::<Vec<_>>();

        stacks[to - 1].extend_from_slice(&tail);
    });

    stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
}
