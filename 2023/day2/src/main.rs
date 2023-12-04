use std::{env, fs};

fn main() {
    let current_dir = env::current_dir().unwrap();

    let input_file = current_dir.join("day2/input.txt");

    println!("Reading input from: {}", input_file.display());

    let input = fs::read_to_string(input_file).unwrap();

    println!("Part 1: {}", part1(&input));

    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> isize {
    let red_max = 12;
    let green_max = 13;
    let blue_max = 14;

    let mut possible_games = vec![];

    'line: for line in input.lines() {
        let (game, contents) = line.split_once(": ").unwrap();

        let game_id: isize = game.split_once(" ").unwrap().1.parse().unwrap();

        let contents_groups: Vec<_> = contents.split("; ").collect();

        for group in contents_groups {
            let elements: Vec<_> = group.split(", ").collect();

            for element in elements {
                let (count, color) = element.split_once(" ").unwrap();

                let count: isize = count.parse().unwrap();

                let max = match color {
                    "red" => red_max,
                    "green" => green_max,
                    "blue" => blue_max,
                    _ => unreachable!(),
                };

                if count > max {
                    continue 'line;
                }
            }
        }

        possible_games.push(game_id);
    }

    possible_games.iter().sum()
}

fn part2(input: &str) -> isize {
    let mut power = 0;

    for line in input.lines() {
        let contents = line.split_once(": ").unwrap().1;

        let mut red_min = 0;
        let mut green_min = 0;
        let mut blue_min = 0;

        let contents_groups: Vec<_> = contents.split("; ").collect();

        for group in contents_groups {
            let elements: Vec<_> = group.split(", ").collect();

            for element in elements {
                let (count, color) = element.split_once(" ").unwrap();

                let count: isize = count.parse().unwrap();

                match color {
                    "red" => red_min = red_min.max(count),
                    "green" => green_min = green_min.max(count),
                    "blue" => blue_min = blue_min.max(count),

                    _ => unreachable!(),
                }
            }
        }

        power += red_min * green_min * blue_min;
    }

    power
}
