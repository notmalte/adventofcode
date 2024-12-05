use itertools::Itertools;

use std::cmp::Ordering;

use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    let parsed = Input::from(input);

    parsed
        .updates
        .iter()
        .filter_map(|u| {
            if !update_is_ordered(u, &parsed.rules) {
                return None;
            }

            Some(u[u.len() / 2])
        })
        .sum::<u64>()
        .to_string()
}

fn part2(input: &str) -> String {
    let parsed = Input::from(input);

    parsed
        .updates
        .iter()
        .filter_map(|u| {
            if update_is_ordered(u, &parsed.rules) {
                return None;
            }

            let ordered = u
                .iter()
                .sorted_by(|&&a, &&b| {
                    if let Some(rule) = parsed
                        .rules
                        .iter()
                        .find(|r| (r.0 == a && r.1 == b) || (r.0 == b && r.1 == a))
                    {
                        if rule.0 == a {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    } else {
                        Ordering::Equal
                    }
                })
                .collect_vec();

            Some(ordered[ordered.len() / 2])
        })
        .sum::<u64>()
        .to_string()
}

fn update_is_ordered(update: &[u64], rules: &[(u64, u64)]) -> bool {
    !rules.iter().any(|r| {
        update.contains(&r.0)
            && update.contains(&r.1)
            && (update.iter().position(|&p| p == r.0).unwrap()
                > update.iter().position(|&p| p == r.1).unwrap())
    })
}

#[derive(Debug)]
struct Input {
    rules: Vec<(u64, u64)>,
    updates: Vec<Vec<u64>>,
}

impl Input {
    fn from(input: &str) -> Self {
        let (rules_str, updates_str) = input
            .split_once("\n\n")
            .unwrap_or_else(|| malformed("2024", "05"));

        let rules = rules_str
            .lines()
            .map(|s| {
                let (left, right) = s.split_once("|").unwrap_or_else(|| malformed("2024", "05"));

                (
                    left.parse().unwrap_or_else(|_| malformed("2024", "05")),
                    right.parse().unwrap_or_else(|_| malformed("2024", "05")),
                )
            })
            .collect();

        let updates = updates_str
            .lines()
            .map(|s| {
                s.split(",")
                    .map(|n| n.parse().unwrap_or_else(|_| malformed("2024", "05")))
                    .collect()
            })
            .collect();

        Self { rules, updates }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
            ),
            "143"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
            ),
            "123"
        );
    }
}
