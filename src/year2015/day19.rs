use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    let parsed = parse_input(input);

    let molecule_chars = parsed.molecule.chars().collect_vec();

    let mut replaced = HashSet::new();

    for replacement in parsed.replacements {
        let from_chars = replacement.from.chars().collect_vec();
        let to_chars = replacement.to.chars().collect_vec();

        for i in 0..molecule_chars.len() {
            if molecule_chars[i..].starts_with(&from_chars) {
                let mut new_molecule = molecule_chars.clone();
                new_molecule.splice(i..i + replacement.from.len(), to_chars.clone());
                replaced.insert(new_molecule.into_iter().collect::<String>());
            }
        }
    }

    replaced.len().to_string()
}

fn part2(input: &str) -> String {
    let parsed = parse_input(input);

    let molecule_chars = parsed.molecule.chars().collect_vec();

    let mut replacements_sorted = parsed.replacements.clone();
    replacements_sorted
        .sort_by_key(|replacement| (replacement.from.len() as i64) - (replacement.to.len() as i64));

    let mut visited = HashMap::new();

    let mut queue_buckets = HashMap::new();

    queue_buckets.insert(molecule_chars.len(), vec![(0, molecule_chars)]);

    while !queue_buckets.is_empty() {
        let min_len = *queue_buckets.keys().min().unwrap();

        let (steps, molecule_chars) = queue_buckets.get_mut(&min_len).unwrap().pop().unwrap();

        if queue_buckets.get(&min_len).unwrap().is_empty() {
            queue_buckets.remove(&min_len);
        }

        if let Some(visited_steps) = visited.get(&molecule_chars) {
            if *visited_steps <= steps {
                continue;
            }
        }

        visited.insert(molecule_chars.clone(), steps);

        if molecule_chars == vec!['e'] {
            return steps.to_string();
        }

        let new_steps = steps + 1;

        for replacement in &replacements_sorted {
            let from_chars = replacement.from.chars().collect_vec();
            let to_chars = replacement.to.chars().collect_vec();

            for i in 0..molecule_chars.len() {
                if molecule_chars[i..].starts_with(&to_chars) {
                    let mut new_molecule = molecule_chars.clone();
                    new_molecule.splice(i..i + replacement.to.len(), from_chars.clone());

                    let new_len = new_molecule.len();

                    if !queue_buckets.contains_key(&new_len) {
                        queue_buckets.insert(new_len, vec![]);
                    }

                    queue_buckets
                        .get_mut(&new_len)
                        .unwrap()
                        .push((new_steps, new_molecule));
                }
            }
        }
    }

    malformed("2015", "19")
}

#[derive(Clone, Debug)]
struct Replacement {
    from: String,
    to: String,
}

struct Input {
    replacements: Vec<Replacement>,
    molecule: String,
}

fn parse_input(input: &str) -> Input {
    let (replacements_str, molecule) = input
        .split_once("\n\n")
        .unwrap_or_else(|| malformed("2015", "19"));

    let replacements = replacements_str
        .lines()
        .map(|line| {
            let (from, to) = line
                .split_once(" => ")
                .unwrap_or_else(|| malformed("2015", "19"));
            Replacement {
                from: from.to_string(),
                to: to.to_string(),
            }
        })
        .collect();

    Input {
        replacements,
        molecule: molecule.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("H => HO\nH => OH\nO => HH\n\nHOH"), "4");
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2("e => H\ne => O\nH => HO\nH => OH\nO => HH\n\nHOHOHO"),
            "6"
        );
    }
}
