use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

const GRID_SIZE: usize = 1000;

fn part1(input: &str) -> String {
    let mut grid = vec![vec![false; GRID_SIZE]; GRID_SIZE];

    let parsed = parse_input(input);

    for instruction in parsed {
        for x in instruction.start_x..=instruction.end_x {
            for y in instruction.start_y..=instruction.end_y {
                match instruction.command {
                    Command::TurnOn => grid[x][y] = true,
                    Command::TurnOff => grid[x][y] = false,
                    Command::Toggle => grid[x][y] = !grid[x][y],
                }
            }
        }
    }

    grid.iter().flatten().filter(|x| **x).count().to_string()
}

fn part2(input: &str) -> String {
    let mut grid = vec![vec![0u64; GRID_SIZE]; GRID_SIZE];

    let parsed = parse_input(input);

    for instruction in parsed {
        for x in instruction.start_x..=instruction.end_x {
            for y in instruction.start_y..=instruction.end_y {
                match instruction.command {
                    Command::TurnOn => grid[x][y] += 1,
                    Command::TurnOff => grid[x][y] = grid[x][y].saturating_sub(1),
                    Command::Toggle => grid[x][y] += 2,
                }
            }
        }
    }

    grid.iter().flatten().sum::<u64>().to_string()
}

enum Command {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Instruction {
    command: Command,
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let re =
        regex::Regex::new(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();

    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap_or_else(|| malformed("2015", "06"));

            let command = match &caps[1] {
                "turn on" => Command::TurnOn,
                "turn off" => Command::TurnOff,
                "toggle" => Command::Toggle,
                _ => unreachable!(),
            };

            Instruction {
                command,
                start_x: caps[2].parse().unwrap(),
                start_y: caps[3].parse().unwrap(),
                end_x: caps[4].parse().unwrap(),
                end_y: caps[5].parse().unwrap(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("turn on 0,0 through 2,2"), "9");
        assert_eq!(part1("toggle 0,0 through 999,999"), "1000000");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("turn on 0,0 through 0,0"), "1");
        assert_eq!(part2("toggle 0,0 through 999,999"), "2000000");
    }
}
