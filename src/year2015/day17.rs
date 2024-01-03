use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    let parsed = parse_input(input);

    let containers = find_container_options_indexes(&parsed, vec![]);

    containers.len().to_string()
}

fn part2(input: &str) -> String {
    let parsed = parse_input(input);

    let containers = find_container_options_indexes(&parsed, vec![]);

    let min_amount = containers.iter().map(|c| c.len()).min().unwrap();

    containers
        .iter()
        .filter(|c| c.len() == min_amount)
        .count()
        .to_string()
}

const LITERS: u64 = 150;

fn parse_input(input: &str) -> Vec<u64> {
    let mut v = input
        .lines()
        .map(|line| line.parse().unwrap_or_else(|_| malformed("2015", "17")))
        .collect::<Vec<_>>();

    v.sort_unstable();

    v
}

fn find_container_options_indexes(
    containers: &Vec<u64>,
    current_indexes: Vec<usize>,
) -> Vec<Vec<usize>> {
    let current_capacity = current_indexes.iter().map(|&i| containers[i]).sum::<u64>();

    if current_capacity == LITERS {
        return vec![current_indexes.clone()];
    }

    let remaining_capacity = LITERS - current_capacity;

    let search_from_index = if current_indexes.is_empty() {
        0
    } else {
        *current_indexes.last().unwrap() + 1
    };

    let candidate_indexes = (search_from_index..containers.len())
        .filter(|&i| containers[i] <= remaining_capacity)
        .collect::<Vec<_>>();

    if candidate_indexes.is_empty() {
        return vec![];
    }

    let mut options = vec![];

    for &i in &candidate_indexes {
        let mut new_current = current_indexes.clone();
        new_current.push(i);

        let result = find_container_options_indexes(containers, new_current);
        options.extend(result);
    }

    options
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("120\n90\n60\n30\n30"), "4");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("120\n90\n60\n30\n30"), "3");
    }
}
