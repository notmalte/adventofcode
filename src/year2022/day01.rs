use crate::Answer;

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|num| num.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .max()
        .unwrap()
        .to_string()
}

fn part2(input: &str) -> String {
    let mut vec: Vec<u64> = input
        .split("\n\n")
        .map(|group| group.lines().map(|num| num.parse::<u64>().unwrap()).sum())
        .collect();

    vec.sort();

    vec.iter().rev().take(3).sum::<u64>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000"),
            "24000"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000"),
            "45000"
        );
    }
}
