use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    let target = input
        .parse::<u64>()
        .unwrap_or_else(|_| malformed("2015", "20"));

    let target_div_10 = target / 10;

    let mut i = 1;
    loop {
        let sum = sum_divisors(i);

        if sum >= target_div_10 {
            return i.to_string();
        }

        i += 1;
    }
}

fn part2(input: &str) -> String {
    let target = input
        .parse::<u64>()
        .unwrap_or_else(|_| malformed("2015", "20"));

    let target_div_11 = (target as f64 / 11.0).ceil() as u64;

    let mut i = 1;
    loop {
        let sum = sum_divisors_with_max_candidates(i, 50);

        if sum >= target_div_11 {
            return i.to_string();
        }

        i += 1;
    }
}

fn sum_divisors(n: u64) -> u64 {
    let sqrt = (n as f64).sqrt() as u64;
    let mut sum = 0;

    for i in 1..=sqrt {
        if n % i == 0 {
            sum += i;
            if n / i != i {
                sum += n / i;
            }
        }
    }

    sum
}

fn sum_divisors_with_max_candidates(n: u64, max_candidates: u64) -> u64 {
    let mut sum = 0;

    for i in 1..=max_candidates {
        if n % i == 0 {
            sum += n / i;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_divisors() {
        assert_eq!(sum_divisors(1), 1);
        assert_eq!(sum_divisors(2), 3);
        assert_eq!(sum_divisors(3), 4);
        assert_eq!(sum_divisors(4), 7);
        assert_eq!(sum_divisors(5), 6);
        assert_eq!(sum_divisors(6), 12);
        assert_eq!(sum_divisors(7), 8);
        assert_eq!(sum_divisors(8), 15);
        assert_eq!(sum_divisors(9), 13);
    }

    #[test]
    fn test_sum_divisors_with_max_candidates() {
        assert_eq!(sum_divisors_with_max_candidates(5, 1), 5);
        assert_eq!(sum_divisors_with_max_candidates(5, 5), 6);
        assert_eq!(sum_divisors_with_max_candidates(5, 10), 6);
    }
}
