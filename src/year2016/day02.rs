use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    let parsed = parse_input(input);

    let keypad = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];

    let mut x: usize = 1;
    let mut y: usize = 1;

    let mut code = String::new();

    for line in parsed {
        for direction in line {
            match direction {
                Direction::Up => {
                    y = y.saturating_sub(1);
                }
                Direction::Down => {
                    if y < 2 {
                        y += 1;
                    }
                }
                Direction::Left => {
                    x = x.saturating_sub(1);
                }
                Direction::Right => {
                    if x < 2 {
                        x += 1;
                    }
                }
            }
        }

        code.push(keypad[y][x]);
    }

    code
}

fn part2(input: &str) -> String {
    let parsed = parse_input(input);

    let keypad = [
        [' ', ' ', '1', ' ', ' '],
        [' ', '2', '3', '4', ' '],
        ['5', '6', '7', '8', '9'],
        [' ', 'A', 'B', 'C', ' '],
        [' ', ' ', 'D', ' ', ' '],
    ];

    let mut x = 0;
    let mut y = 2;

    let mut code = String::new();

    for line in parsed {
        for direction in line {
            match direction {
                Direction::Up => {
                    if y > 0 && keypad[y - 1][x] != ' ' {
                        y -= 1;
                    }
                }
                Direction::Down => {
                    if y < 4 && keypad[y + 1][x] != ' ' {
                        y += 1;
                    }
                }
                Direction::Left => {
                    if x > 0 && keypad[y][x - 1] != ' ' {
                        x -= 1;
                    }
                }
                Direction::Right => {
                    if x < 4 && keypad[y][x + 1] != ' ' {
                        x += 1;
                    }
                }
            }
        }

        code.push(keypad[y][x]);
    }

    code
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_input(input: &str) -> Vec<Vec<Direction>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'U' => Direction::Up,
                    'D' => Direction::Down,
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => malformed("2016", "2"),
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("ULL\nRRDDD\nLURDL\nUUUUD"), "1985");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("ULL\nRRDDD\nLURDL\nUUUUD"), "5DB3");
    }
}
