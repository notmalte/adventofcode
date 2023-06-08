use std::{collections::VecDeque, env, fs};

fn main() {
    let current_dir = env::current_dir().unwrap();

    let input_file = current_dir.join("day11/input.txt");

    println!("Reading input from: {}", input_file.display());

    let input = fs::read_to_string(input_file).unwrap();

    println!("Part 1: {}", part1(&input));

    println!("Part 2: {}", part2(&input));
}

#[derive(Debug)]
enum Operand {
    Old,
    Constant(isize),
}

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug)]
struct Operation {
    operand1: Operand,
    operator: Operator,
    operand2: Operand,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<isize>,
    inspections: usize,
    operation: Operation,
    test_divisible_by: isize,
    target_index_if_true: usize,
    target_index_if_false: usize,
}

impl Monkey {
    fn get_result(&self, item: isize) -> isize {
        let operand1 = match self.operation.operand1 {
            Operand::Old => item,
            Operand::Constant(c) => c,
        };

        let operand2 = match self.operation.operand2 {
            Operand::Old => item,
            Operand::Constant(c) => c,
        };

        match self.operation.operator {
            Operator::Add => operand1 + operand2,
            Operator::Multiply => operand1 * operand2,
        }
    }

    fn get_target_index_from_result(&self, result: isize) -> usize {
        if result % self.test_divisible_by == 0 {
            self.target_index_if_true
        } else {
            self.target_index_if_false
        }
    }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|monkey| {
            let lines: Vec<&str> = monkey.lines().skip(1).map(|s| s.trim()).collect();

            let items: VecDeque<isize> = lines[0]
                .strip_prefix("Starting items: ")
                .unwrap()
                .split(", ")
                .map(|s| s.parse().unwrap())
                .collect();

            let operation = parse_operation(lines[1].strip_prefix("Operation: new = ").unwrap());

            let test_divisible_by = lines[2]
                .strip_prefix("Test: divisible by ")
                .unwrap()
                .parse()
                .unwrap();

            let target_index_if_true = lines[3]
                .strip_prefix("If true: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap();

            let target_index_if_false = lines[4]
                .strip_prefix("If false: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap();

            Monkey {
                items,
                inspections: 0,
                operation,
                test_divisible_by,
                target_index_if_true,
                target_index_if_false,
            }
        })
        .collect()
}

fn parse_operation(input: &str) -> Operation {
    let parts: Vec<&str> = input.split(' ').collect();

    let operand1 = match parts[0] {
        "old" => Operand::Old,
        _ => Operand::Constant(parts[0].parse().unwrap()),
    };

    let operator = match parts[1] {
        "+" => Operator::Add,
        "*" => Operator::Multiply,
        _ => panic!("Unknown operator: {}", parts[1]),
    };

    let operand2 = match parts[2] {
        "old" => Operand::Old,
        _ => Operand::Constant(parts[2].parse().unwrap()),
    };

    Operation {
        operand1,
        operator,
        operand2,
    }
}

fn calculate_monkey_business(monkeys: &[Monkey]) -> usize {
    let mut monkey_inspections: Vec<usize> = monkeys.iter().map(|m| m.inspections).collect();

    monkey_inspections.sort_by(|a, b| b.cmp(a));

    monkey_inspections[0] * monkey_inspections[1]
}

fn part1(input: &str) -> usize {
    let mut monkeys = parse_monkeys(input);

    let monkey_count = monkeys.len();

    let rounds = 20;

    for _ in 1..=rounds {
        for i in 0..monkey_count {
            let mut thrown_items = vec![];

            let monkey = &mut monkeys[i];

            while let Some(item) = monkey.items.pop_front() {
                let result = monkey.get_result(item) / 3;

                let target_index = monkey.get_target_index_from_result(result);

                thrown_items.push((target_index, result));

                monkey.inspections += 1;
            }

            for (target_index, item) in thrown_items {
                monkeys[target_index].items.push_back(item);
            }
        }
    }

    calculate_monkey_business(&monkeys)
}

fn part2(input: &str) -> usize {
    let mut monkeys = parse_monkeys(input);

    let monkey_count = monkeys.len();

    let monkeys_divisible_product: isize = monkeys.iter().map(|m| m.test_divisible_by).product();

    let rounds = 10000;

    for _ in 1..=rounds {
        for i in 0..monkey_count {
            let mut thrown_items = vec![];

            let monkey = &mut monkeys[i];

            while let Some(item) = monkey.items.pop_front() {
                let result = monkey.get_result(item) % monkeys_divisible_product;

                let target_index = monkey.get_target_index_from_result(result);

                thrown_items.push((target_index, result));

                monkey.inspections += 1;
            }

            for (target_index, item) in thrown_items {
                monkeys[target_index].items.push_back(item);
            }
        }
    }

    calculate_monkey_business(&monkeys)
}
