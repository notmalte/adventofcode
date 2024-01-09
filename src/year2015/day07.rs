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

    parsed.eval("a").to_string()
}

fn part2(input: &str) -> String {
    let mut parsed = parse_input(input);

    let a = parsed.eval("a");

    parsed
        .wires
        .insert("b".to_string(), WireInput::Pure(WireOrValue::Value(a)));

    parsed.eval("a").to_string()
}

#[derive(Debug, Clone)]
enum WireOrValue {
    Wire(String),
    Value(u16),
}

#[derive(Debug, Clone)]
enum WireInput {
    Pure(WireOrValue),
    And(WireOrValue, WireOrValue),
    Or(WireOrValue, WireOrValue),
    LShift(WireOrValue, u16),
    RShift(WireOrValue, u16),
    Not(WireOrValue),
}

#[derive(Debug)]
struct Circuit {
    wires: HashMap<String, WireInput>,
}

impl Circuit {
    fn eval(&self, wire: &str) -> u16 {
        self.eval_cached(wire, &mut HashMap::new())
    }

    fn eval_cached(&self, wire: &str, cache: &mut HashMap<String, u16>) -> u16 {
        let input = &self.wires[wire];

        match input {
            WireInput::Pure(wire_or_value) => self.eval_wire_or_value_cached(wire_or_value, cache),
            WireInput::And(a, b) => {
                self.eval_wire_or_value_cached(a, cache) & self.eval_wire_or_value_cached(b, cache)
            }
            WireInput::Or(a, b) => {
                self.eval_wire_or_value_cached(a, cache) | self.eval_wire_or_value_cached(b, cache)
            }
            WireInput::LShift(a, b) => self.eval_wire_or_value_cached(a, cache) << b,
            WireInput::RShift(a, b) => self.eval_wire_or_value_cached(a, cache) >> b,
            WireInput::Not(a) => !self.eval_wire_or_value_cached(a, cache),
        }
    }

    fn eval_wire_or_value_cached(
        &self,
        wire_or_value: &WireOrValue,
        cache: &mut HashMap<String, u16>,
    ) -> u16 {
        match wire_or_value {
            WireOrValue::Wire(wire) => {
                if !cache.contains_key(wire) {
                    let value = self.eval_cached(wire, cache);

                    cache.insert(wire.clone(), value);
                }

                cache[wire]
            }
            WireOrValue::Value(value) => *value,
        }
    }
}

fn parse_wire_or_value(input: &str) -> WireOrValue {
    if let Ok(value) = input.parse() {
        WireOrValue::Value(value)
    } else {
        WireOrValue::Wire(input.to_string())
    }
}

fn parse_input(input: &str) -> Circuit {
    let mut map = HashMap::new();

    for line in input.lines() {
        let (src, dst) = line.split_once(" -> ").unwrap();

        let input = match src.split(' ').collect::<Vec<_>>()[..] {
            [a] => WireInput::Pure(parse_wire_or_value(a)),
            [a, "AND", b] => WireInput::And(parse_wire_or_value(a), parse_wire_or_value(b)),
            [a, "OR", b] => WireInput::Or(parse_wire_or_value(a), parse_wire_or_value(b)),
            [a, "LSHIFT", b] => WireInput::LShift(
                parse_wire_or_value(a),
                b.parse().unwrap_or_else(|_| malformed("2015", "07")),
            ),
            [a, "RSHIFT", b] => WireInput::RShift(
                parse_wire_or_value(a),
                b.parse().unwrap_or_else(|_| malformed("2015", "07")),
            ),
            ["NOT", a] => WireInput::Not(parse_wire_or_value(a)),
            _ => malformed("2015", "07"),
        };

        map.insert(dst.to_string(), input);
    }

    Circuit { wires: map }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("123 -> x\n456 -> y\nx AND y -> a\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i"), "72");
    }
}
