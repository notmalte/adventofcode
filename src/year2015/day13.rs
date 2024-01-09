use std::collections::HashMap;

use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    let parsed = parse_input(input);

    find_max_happiness(&parsed, vec![]).to_string()
}

fn part2(input: &str) -> String {
    let mut parsed = parse_input(input);

    parsed.insert("me".to_string(), HashMap::new());

    let others = parsed
        .keys()
        .filter(|&person| person != "me")
        .cloned()
        .collect::<Vec<_>>();

    for person in others {
        parsed.entry("me".to_string()).and_modify(|gains| {
            gains.insert(person.clone(), 0);
        });

        parsed.entry(person).and_modify(|gains| {
            gains.insert("me".to_string(), 0);
        });
    }

    find_max_happiness(&parsed, vec![]).to_string()
}

type Happiness = HashMap<String, HashMap<String, i64>>;

fn parse_input(input: &str) -> Happiness {
    let re = regex::Regex::new(
        r"^(.+) would (lose|gain) (\d+) happiness units by sitting next to (.+)\.$",
    )
    .unwrap();

    let mut happiness = Happiness::new();

    for line in input.lines() {
        let caps = re.captures(line).unwrap_or_else(|| malformed("2015", "13"));

        let person1 = caps[1].to_string();
        let direction = caps[2].to_string();
        let mut amount = caps[3].parse::<i64>().unwrap();
        let person2 = caps[4].to_string();

        if direction == "lose" {
            amount *= -1;
        }

        happiness
            .entry(person1)
            .or_default()
            .insert(person2, amount);
    }

    happiness
}

fn find_max_happiness(happiness: &Happiness, current_seating: Vec<String>) -> i64 {
    let remaining = happiness
        .iter()
        .map(|(person, _)| person)
        .filter(|person| !current_seating.contains(person))
        .collect::<Vec<_>>();

    if remaining.is_empty() {
        return current_seating
            .iter()
            .enumerate()
            .map(|(i, person)| {
                let len = current_seating.len();

                let gains = &happiness[person];

                gains[&current_seating[(i + 1) % len]]
                    + gains[&current_seating[(i + len - 1) % len]]
            })
            .sum();
    }

    let mut max_happiness = i64::MIN;

    for person in remaining {
        let mut new_seating = current_seating.clone();
        new_seating.push(person.clone());

        let new_happiness = find_max_happiness(happiness, new_seating);

        max_happiness = max_happiness.max(new_happiness);
    }

    max_happiness
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol."
            ),
            "330"
        )
    }
}
