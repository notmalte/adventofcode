use std::{env, fs};

fn main() {
    let current_dir = env::current_dir().unwrap();

    let input_file = current_dir.join("day6/input.txt");

    println!("Reading input from: {}", input_file.display());

    let input = fs::read_to_string(input_file).unwrap();

    println!("Input: {}", input);

    println!("Part 1: {}", part1(&input));

    println!("Part 2: {}", part2(&input));
}

fn find_unique_pattern(input: &str, length: usize) -> usize {
    let chars: Vec<char> = input.chars().collect();

    let len = chars.len();

    for i in (length - 1)..len {
        let mut copy = vec![' '; length];
        copy.copy_from_slice(&chars[i - (length - 1)..i + 1]);

        copy.sort();
        copy.dedup();

        if copy.len() == length {
            return i + 1;
        }
    }

    panic!("No unique pattern found");
}

fn part1(input: &str) -> usize {
    find_unique_pattern(input, 4)
}

fn part2(input: &str) -> usize {
    find_unique_pattern(input, 14)
}
