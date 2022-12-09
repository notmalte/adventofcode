use std::{collections::HashSet, env, fs};

fn main() {
    let current_dir = env::current_dir().unwrap();

    let input_file = current_dir.join("day9/input.txt");

    println!("Reading input from: {}", input_file.display());

    let input = fs::read_to_string(input_file).unwrap();

    println!("Input: {}", input);

    println!("Part 1: {}", part1(&input));

    println!("Part 2: {}", part2(&input));
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

fn parse_moves(input: &str) -> Vec<(Direction, isize)> {
    let mut moves = vec![];

    for line in input.lines() {
        let (direction, distance) = line.split_once(' ').unwrap();

        let direction = match direction {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Unknown direction: {}", direction),
        };

        let distance: isize = distance.parse().unwrap();

        moves.push((direction, distance));
    }

    moves
}

fn get_next_tail_position(head: &Position, tail: &Position) -> Position {
    let mut tail = tail.clone();

    if (head.x - tail.x).abs() > 1 || (head.y - tail.y).abs() > 1 {
        tail.x += (head.x - tail.x).signum();
        tail.y += (head.y - tail.y).signum();
    }

    tail
}

fn part1(input: &str) -> usize {
    let moves = parse_moves(input);

    let mut visited = HashSet::new();

    let mut head = Position { x: 0, y: 0 };
    let mut tail = Position { x: 0, y: 0 };

    visited.insert(Position { x: 0, y: 0 });

    for (direction, distance) in moves {
        for _ in 0..distance {
            match direction {
                Direction::Up => head.y += 1,
                Direction::Down => head.y -= 1,
                Direction::Left => head.x -= 1,
                Direction::Right => head.x += 1,
            }

            tail = get_next_tail_position(&head, &tail);

            visited.insert(tail.clone());
        }
    }

    visited.len()
}

fn part2(input: &str) -> usize {
    let moves = parse_moves(input);

    let mut visited = HashSet::new();

    let mut rope = vec![];
    for _ in 0..10 {
        rope.push(Position { x: 0, y: 0 });
    }

    visited.insert(Position { x: 0, y: 0 });

    for (direction, distance) in moves {
        for _ in 0..distance {
            let mut head = &mut rope[0];

            match direction {
                Direction::Up => head.y += 1,
                Direction::Down => head.y -= 1,
                Direction::Left => head.x -= 1,
                Direction::Right => head.x += 1,
            }

            for i in 1..rope.len() {
                rope[i] = get_next_tail_position(&rope[i - 1], &rope[i]);
            }

            visited.insert(rope[rope.len() - 1].clone());
        }
    }

    visited.len()
}
