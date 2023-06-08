use std::{env, fs};

#[derive(PartialEq, Copy, Clone, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

fn str_to_shape(s: &str) -> Shape {
    match s {
        "A" | "X" => Shape::Rock,
        "B" | "Y" => Shape::Paper,
        "C" | "Z" => Shape::Scissors,
        _ => panic!("Invalid shape"),
    }
}

fn str_to_outcome(s: &str) -> Outcome {
    match s {
        "X" => Outcome::Loss,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => panic!("Invalid outcome"),
    }
}

fn score_per_shape(m: Shape) -> isize {
    match m {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn score_per_round(mine: Shape, theirs: Shape) -> isize {
    let shape_score = score_per_shape(mine);

    let outcome_score = if mine == theirs {
        3
    } else if (mine == Shape::Rock && theirs == Shape::Scissors)
        || (mine == Shape::Paper && theirs == Shape::Rock)
        || (mine == Shape::Scissors && theirs == Shape::Paper)
    {
        6
    } else {
        0
    };

    shape_score + outcome_score
}

fn main() {
    let current_dir = env::current_dir().unwrap();

    let input_file = current_dir.join("day2/input.txt");

    println!("Reading input from: {}", input_file.display());

    let input = fs::read_to_string(input_file).unwrap();

    println!("Part 1: {}", part1(&input));

    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> isize {
    let mut score = 0;

    for line in input.lines() {
        let shapes: Vec<Shape> = line.split(' ').map(str_to_shape).collect();

        let round_score = score_per_round(shapes[1], shapes[0]);

        score += round_score;
    }

    score
}

fn part2(input: &str) -> isize {
    let mut score = 0;

    for line in input.lines() {
        let chars: Vec<&str> = line.split(' ').collect();

        let their_shape = str_to_shape(chars[0]);

        let wanted_outcome = str_to_outcome(chars[1]);

        let my_shape = match wanted_outcome {
            Outcome::Loss => match their_shape {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            },
            Outcome::Draw => their_shape,
            Outcome::Win => match their_shape {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            },
        };

        let round_score = score_per_round(my_shape, their_shape);

        score += round_score;
    }

    score
}
