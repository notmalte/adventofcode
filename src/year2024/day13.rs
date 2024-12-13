use regex::Regex;

use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    parse_input(input)
        .iter()
        .filter_map(tokens_for_prize)
        .sum::<u64>()
        .to_string()
}

fn part2(input: &str) -> String {
    parse_input(input)
        .iter()
        .filter_map(|m| {
            let m2 = Machine {
                a: m.a,
                b: m.b,
                prize: (m.prize.0 + 10000000000000, m.prize.1 + 10000000000000),
            };

            tokens_for_prize(&m2)
        })
        .sum::<u64>()
        .to_string()
}

fn tokens_for_prize(machine: &Machine) -> Option<u64> {
    // A X = B
    //
    // [x_a x_b] [a] = [x_p]
    // [y_a y_b] [b]   [y_p]
    //
    // -> if det(A) != 0
    //
    // [a] = [x_a x_b]^-1 [x_p]
    // [b]   [y_a y_b]    [y_p]
    //
    // -> Cramer's rule
    //
    // a = det([x_p x_b]) / det(A)
    //         [y_p y_b]
    //
    // b = det([x_a x_p]) / det(A)
    //         [y_a y_p]
    //
    // -> if a, b \in N we have a solution

    let (x_a, y_a) = machine.a;
    let (x_b, y_b) = machine.b;
    let (x_p, y_p) = machine.prize;

    let det = determinant(x_a, x_b, y_a, y_b);

    if det == 0 {
        panic!("unexpected parallel vectors")
    }

    let a_numerator = determinant(x_p, x_b, y_p, y_b);
    let b_numerator = determinant(x_a, x_p, y_a, y_p);

    if a_numerator % det != 0 || b_numerator % det != 0 {
        return None;
    }

    let a = a_numerator / det;
    let b = b_numerator / det;

    if a < 0 || b < 0 {
        return None;
    }

    Some(3 * a as u64 + b as u64)
}

fn determinant(a: i64, b: i64, c: i64, d: i64) -> i64 {
    a * d - b * c
}

#[derive(Debug, Clone)]
struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

fn parse_input(input: &str) -> Vec<Machine> {
    let mut v = Vec::new();

    let re = Regex::new(
        r"^Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)$",
    )
    .unwrap();

    for machine in input.split("\n\n") {
        let caps = re
            .captures(machine.trim())
            .unwrap_or_else(|| malformed("2024", "13"));

        v.push(Machine {
            a: (caps[1].parse().unwrap(), caps[2].parse().unwrap()),
            b: (caps[3].parse().unwrap(), caps[4].parse().unwrap()),
            prize: (caps[5].parse().unwrap(), caps[6].parse().unwrap()),
        })
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"
            ),
            "480"
        );
    }
}
