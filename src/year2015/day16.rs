use std::collections::HashMap;

use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    let wanted = get_wanted();

    let parsed = parse_input(input);

    for sue in parsed {
        let fits = sue.items.iter().all(|(name, &value)| value == wanted[name]);

        if fits {
            return sue.number.to_string();
        }
    }

    malformed("2015", "16")
}

fn part2(input: &str) -> String {
    let wanted = get_wanted();

    let parsed = parse_input(input);

    for sue in parsed {
        let fits = sue.items.iter().all(|(name, &value)| match name.as_str() {
            "cats" | "trees" => value > wanted[name],
            "pomeranians" | "goldfish" => value < wanted[name],
            _ => value == wanted[name],
        });

        if fits {
            return sue.number.to_string();
        }
    }

    malformed("2015", "16")
}

fn get_wanted() -> HashMap<String, u64> {
    HashMap::from([
        ("children".to_string(), 3),
        ("cats".to_string(), 7),
        ("samoyeds".to_string(), 2),
        ("pomeranians".to_string(), 3),
        ("akitas".to_string(), 0),
        ("vizslas".to_string(), 0),
        ("goldfish".to_string(), 5),
        ("trees".to_string(), 3),
        ("cars".to_string(), 2),
        ("perfumes".to_string(), 1),
    ])
}

#[derive(Debug)]
struct Sue {
    number: u64,
    items: HashMap<String, u64>,
}

fn parse_input(input: &str) -> Vec<Sue> {
    let re = regex::Regex::new(r"^Sue (\d+): (.*)$").unwrap();

    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap_or_else(|| malformed("2015", "16"));

            let number = caps[1].parse().unwrap();

            let items = caps[2]
                .split(", ")
                .map(|item| {
                    let parts = item.split(": ").collect::<Vec<_>>();
                    let name = parts[0].to_string();
                    let value = parts[1].parse().unwrap();

                    (name, value)
                })
                .collect();

            Sue { number, items }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Sue 1: trees: 1, cars: 2, perfumes: 3
Sue 2: children: 1, cats: 2, samoyeds: 3
Sue 3: children: 3, cats: 7, cars: 2";

        assert_eq!(part1(input), "3");
    }

    #[test]
    fn test_part2() {
        let input = "Sue 1: trees: 1, cars: 2, perfumes: 3
Sue 2: children: 1, cats: 2, samoyeds: 3
Sue 3: children: 3, cats: 8, pomeranians: 2";

        assert_eq!(part2(input), "3");
    }
}
