use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    let parsed = parse_input(input);

    parsed
        .iter()
        .filter(|(a, b, c)| a + b > *c && a + c > *b && b + c > *a)
        .count()
        .to_string()
}

fn part2(input: &str) -> String {
    let parsed = parse_input(input);

    parsed
        .chunks_exact(3)
        .flat_map(|chunk| {
            let (a1, b1, c1) = chunk[0];
            let (a2, b2, c2) = chunk[1];
            let (a3, b3, c3) = chunk[2];
            vec![(a1, a2, a3), (b1, b2, b3), (c1, c2, c3)]
        })
        .filter(|(a, b, c)| a + b > *c && a + c > *b && b + c > *a)
        .count()
        .to_string()
}

fn parse_input(input: &str) -> Vec<(u64, u64, u64)> {
    input
        .lines()
        .map(|line| {
            let l = line.split_whitespace().collect::<Vec<_>>();
            let a = l[0].parse().unwrap_or_else(|_| malformed("2016", "03"));
            let b = l[1].parse().unwrap_or_else(|_| malformed("2016", "03"));
            let c = l[2].parse().unwrap_or_else(|_| malformed("2016", "03"));
            (a, b, c)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("5 10 25"), "0");
        assert_eq!(part1("3 4 5"), "1");
        assert_eq!(part1("5 10 25\n3 4 5"), "1");
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2("101 301 501\n102 302 502\n103 303 503\n201 401 601\n202 402 602\n203 403 603"),
            "6"
        );
    }
}
