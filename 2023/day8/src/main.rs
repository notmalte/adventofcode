use itertools::Itertools;
use std::{collections::HashMap, env, fs};

fn main() {
    let current_dir = env::current_dir().unwrap();

    let input_file = current_dir.join("day8/input.txt");

    println!("Reading input from: {}", input_file.display());

    let input = fs::read_to_string(input_file).unwrap();

    println!("Part 1: {}", part1(&input));

    println!("Part 2: {}", part2(&input));
}

#[derive(Debug)]
struct NodeLinks {
    left: String,
    right: String,
}

fn parse_input(input: &str) -> (Vec<char>, HashMap<String, NodeLinks>) {
    let (directions, nodes) = input.split_once("\n\n").unwrap();

    let directions = directions.chars().collect_vec();

    let mut node_map = HashMap::new();

    for node in nodes.lines() {
        let (name, links) = node.split_once(" = ").unwrap();

        let links = links.replace("(", "").replace(")", "");

        let (left, right) = links.split_once(", ").unwrap();

        node_map.insert(
            name.to_string(),
            NodeLinks {
                left: left.to_string(),
                right: right.to_string(),
            },
        );
    }

    (directions, node_map)
}

fn part1(input: &str) -> isize {
    let (directions, node_map) = parse_input(input);

    let mut current_node = "AAA".to_string();
    let mut steps = 0;

    while current_node != "ZZZ" {
        let node = node_map.get(&current_node).unwrap();

        if directions[steps % directions.len()] == 'R' {
            current_node = node.right.clone();
        } else {
            current_node = node.left.clone();
        }

        steps += 1;
    }

    steps as isize
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn part2(input: &str) -> isize {
    let (directions, node_map) = parse_input(input);

    let mut relevant_nodes = vec![];

    for (name, _) in node_map.iter() {
        if name.ends_with("A") {
            relevant_nodes.push(name.clone());
        }
    }

    let mut cycle_map = HashMap::new();

    for node in relevant_nodes {
        let mut current_node = node.clone();
        let mut steps = 0;

        while !current_node.ends_with("Z") {
            let node = node_map.get(&current_node).unwrap();

            if directions[steps % directions.len()] == 'R' {
                current_node = node.right.clone();
            } else {
                current_node = node.left.clone();
            }

            steps += 1;
        }

        cycle_map.insert(node, steps);
    }

    cycle_map
        .iter()
        .map(|(_, v)| *v)
        .reduce(|a, b| lcm(a, b))
        .unwrap() as isize
}
