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

    let sum: u64 = parsed
        .iter()
        .filter(|room| room.is_real())
        .map(|room| room.sector_id)
        .sum();

    sum.to_string()
}

fn part2(input: &str) -> String {
    let parsed = parse_input(input);

    let valid = parsed
        .iter()
        .filter(|room| room.is_real())
        .collect::<Vec<_>>();

    for room in valid {
        let mut decrypted = String::new();

        for c in room.name.chars() {
            if c == '-' {
                decrypted.push(' ');
                continue;
            }

            let c = (c as u8 - b'a' + (room.sector_id % 26) as u8) % 26 + b'a';
            decrypted.push(c as char);
        }

        if decrypted.contains("northpole") {
            return room.sector_id.to_string();
        }
    }

    malformed("2016", "04")
}

struct Room {
    name: String,
    sector_id: u64,
    checksum: String,
}

impl Room {
    fn is_real(&self) -> bool {
        let mut counts_map = HashMap::new();

        for c in self.name.chars() {
            if c == '-' {
                continue;
            }

            counts_map.entry(c).and_modify(|e| *e += 1).or_insert(1);
        }

        let mut counts_vec: Vec<_> = counts_map.into_iter().collect();

        counts_vec.sort_by(|a, b| {
            if a.1 == b.1 {
                a.0.cmp(&b.0)
            } else {
                b.1.cmp(&a.1)
            }
        });

        let mut checksum = String::new();
        for (c, _) in counts_vec.iter().take(5) {
            checksum.push(*c);
        }

        checksum == self.checksum
    }
}

fn parse_input(input: &str) -> Vec<Room> {
    let re = regex::Regex::new(r"([a-z-]+)-(\d+)\[([a-z]+)\]").unwrap();

    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap_or_else(|| malformed("2016", "04"));

            Room {
                name: caps[1].to_string(),
                sector_id: caps[2].parse().unwrap(),
                checksum: caps[3].to_string(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "aaaaa-bbb-z-y-x-123[abxyz]\na-b-c-d-e-f-g-h-987[abcde]\nnot-a-real-room-404[oarel]\ntotally-real-room-200[decoy]";
        assert_eq!(part1(input), "1514");
    }

    #[test]
    fn test_part2() {
        let input = "lmprfnmjc-mzhcar-qrmpyec-548[mcrpa]";
        assert_eq!(part2(input), "548");
    }
}
