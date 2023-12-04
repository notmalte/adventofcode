use std::{env, fs};

fn main() {
    let current_dir = env::current_dir().unwrap();

    let input_file = current_dir.join("day4/input.txt");

    println!("Reading input from: {}", input_file.display());

    let input = fs::read_to_string(input_file).unwrap();

    println!("Part 1: {}", part1(&input));

    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> isize {
    let mut sum = 0;

    for line in input.lines() {
        let numbers = line.split_once(": ").unwrap().1;

        let (winning, actual) = numbers.split_once(" | ").unwrap();

        let winning = winning
            .split_whitespace()
            .map(|n| n.parse::<isize>().unwrap())
            .collect::<Vec<_>>();

        let actual = actual
            .split_whitespace()
            .map(|n| n.parse::<isize>().unwrap())
            .collect::<Vec<_>>();

        let mut score = 0;

        for n in actual {
            if winning.contains(&n) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }

        sum += score;
    }

    sum
}

fn part2(input: &str) -> isize {
    let lines = input.lines().collect::<Vec<_>>();

    let scores = lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let numbers = line.split_once(": ").unwrap().1;

            let (winning, actual) = numbers.split_once(" | ").unwrap();

            let winning = winning
                .split_whitespace()
                .map(|n| n.parse::<isize>().unwrap())
                .collect::<Vec<_>>();

            let actual = actual
                .split_whitespace()
                .map(|n| n.parse::<isize>().unwrap())
                .collect::<Vec<_>>();

            let max_score = lines.len() - i - 1;

            let score = actual.iter().filter(|n| winning.contains(n)).count();

            score.min(max_score)
        })
        .collect::<Vec<_>>();

    let mut stack = vec![];

    let mut count = scores.len();

    for (i, score) in scores.iter().enumerate() {
        for n in 1..=*score {
            stack.push(i + n);
        }
    }

    while let Some(i) = stack.pop() {
        count += 1;

        for n in 1..=scores[i] {
            stack.push(i + n);
        }
    }

    count as isize
}
