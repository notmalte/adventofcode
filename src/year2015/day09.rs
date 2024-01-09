use itertools::Itertools;
use std::collections::HashMap;

use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    let distances = parse_distances(input);

    find_best_route(&distances, true, vec![]).to_string()
}

fn part2(input: &str) -> String {
    let distances = parse_distances(input);

    find_best_route(&distances, false, vec![]).to_string()
}

type Distances = HashMap<String, HashMap<String, u64>>;

fn parse_distances(input: &str) -> Distances {
    let mut distances = Distances::new();

    let re = regex::Regex::new(r"^(.+) to (.+) = (\d+)$").unwrap();

    for line in input.lines() {
        let caps = re.captures(line).unwrap_or_else(|| malformed("2015", "09"));

        let start = caps[1].to_string();
        let end = caps[2].to_string();
        let distance = caps[3].parse::<u64>().unwrap();

        distances
            .entry(start.clone())
            .or_default()
            .insert(end.clone(), distance);

        distances.entry(end).or_default().insert(start, distance);
    }

    distances
}

fn find_best_route(distances: &Distances, find_min: bool, current_route: Vec<String>) -> u64 {
    let remaining = distances
        .iter()
        .map(|(location, _)| location)
        .filter(|location| !current_route.contains(location))
        .collect::<Vec<_>>();

    if remaining.is_empty() {
        return current_route
            .iter()
            .tuple_windows()
            .map(|(start, end)| distances[start][end])
            .sum();
    }

    let mut best_route = if find_min { u64::MAX } else { u64::MIN };

    for location in remaining {
        let mut new_route = current_route.clone();
        new_route.push(location.clone());

        let new_distance = find_best_route(distances, find_min, new_route);

        if (find_min && (new_distance < best_route)) || (!find_min && (new_distance > best_route)) {
            best_route = new_distance;
        }
    }

    best_route
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1("London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141"),
            "605"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2("London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141"),
            "982"
        );
    }
}
