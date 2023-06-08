use std::{collections::HashSet, env, fs};

use itertools::Itertools;

fn match_char_to_priority(c: char) -> isize {
    match c {
        'a'..='z' => c as isize - 'a' as isize + 1,
        'A'..='Z' => c as isize - 'A' as isize + 27,
        _ => panic!("Invalid priority"),
    }
}

fn main() {
    let current_dir = env::current_dir().unwrap();

    let input_file = current_dir.join("day3/input.txt");

    println!("Reading input from: {}", input_file.display());

    let input = fs::read_to_string(input_file).unwrap();

    println!("Input: {}", input);

    println!("Part 1: {}", part1(&input));

    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> isize {
    input
        .lines()
        .map(|line| {
            let len = line.len();

            let mut set1 = HashSet::new();
            let mut set2 = HashSet::new();

            let s1 = &line[0..len / 2];
            let s2 = &line[len / 2..];

            for c in s1.chars() {
                set1.insert(c);
            }

            for c in s2.chars() {
                set2.insert(c);
            }

            match_char_to_priority(*set1.intersection(&set2).next().unwrap())
        })
        .sum()
}

fn part2(input: &str) -> isize {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let inner_lines = chunk.collect_vec();

            let mut set1 = HashSet::new();
            let mut set2 = HashSet::new();
            let mut set3 = HashSet::new();

            let s1 = &inner_lines[0];
            let s2 = &inner_lines[1];
            let s3 = &inner_lines[2];

            for c in s1.chars() {
                set1.insert(c);
            }

            for c in s2.chars() {
                set2.insert(c);
            }

            for c in s3.chars() {
                set3.insert(c);
            }

            let mut intersection = set1;
            intersection.retain(|c| set2.contains(c));
            intersection.retain(|c| set3.contains(c));

            match_char_to_priority(*intersection.iter().next().unwrap())
        })
        .sum()
}
