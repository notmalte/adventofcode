use std::{env, fs};

fn main() {
    let current_dir = env::current_dir().unwrap();

    let input_file = current_dir.join("day10/input.txt");

    println!("Reading input from: {}", input_file.display());

    let input = fs::read_to_string(input_file).unwrap();

    println!("Part 1: {}", part1(&input));

    println!("Part 2:");
    part2(&input);
}

#[derive(Debug, Clone)]
enum Instruction {
    Noop,
    Addx(isize),
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut instructions = vec![];

    for line in input.lines() {
        let instruction = if line == "noop" {
            Instruction::Noop
        } else if line.starts_with("addx") {
            let value: isize = line.strip_prefix("addx ").unwrap().parse().unwrap();
            Instruction::Addx(value)
        } else {
            panic!("Unknown instruction: {}", line);
        };

        instructions.push(instruction);
    }

    instructions
}

fn get_x_before_cycle_vec(instructions: &[Instruction]) -> Vec<isize> {
    let mut x_before_cycle = vec![1];

    for instruction in instructions {
        match instruction {
            Instruction::Noop => {
                x_before_cycle.push(x_before_cycle[x_before_cycle.len() - 1]);
            }
            Instruction::Addx(value) => {
                let mut previous_x = x_before_cycle[x_before_cycle.len() - 1];

                x_before_cycle.push(previous_x);

                previous_x += value;
                x_before_cycle.push(previous_x);
            }
        }
    }

    x_before_cycle
}

fn part1(input: &str) -> isize {
    let instructions = parse_instructions(input);

    let x_before_cycle = get_x_before_cycle_vec(&instructions);

    let mut signal_strengths = vec![];

    for i in (20..=x_before_cycle.len()).step_by(40) {
        signal_strengths.push((i as isize) * x_before_cycle[i - 1]);
    }

    signal_strengths.iter().sum()
}

fn part2(input: &str) {
    let instructions = parse_instructions(input);

    let x_before_cycle = get_x_before_cycle_vec(&instructions);

    let screen_width = 40;
    let screen_height = 6;

    let mut screen = vec![vec![false; screen_width]; screen_height];

    for i in 0..screen_width * screen_height {
        let x = i % screen_width;
        let y = i / screen_width;

        let x_before_cycle = x_before_cycle[i];

        if (x as isize) >= x_before_cycle - 1 && (x as isize) <= x_before_cycle + 1 {
            screen[y][x] = true;
        }
    }

    for row in screen {
        for cell in row {
            if cell {
                print!("█");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
