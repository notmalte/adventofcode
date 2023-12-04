use std::{env, fs};

fn main() {
    let current_dir = env::current_dir().unwrap();

    let input_file = current_dir.join("day3/input.txt");

    println!("Reading input from: {}", input_file.display());

    let input = fs::read_to_string(input_file).unwrap();

    println!("Part 1: {}", part1(&input));

    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> isize {
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut sum = 0;

    let mut y = 0;
    while y < grid.len() {
        let mut x = 0;
        while x < grid[y].len() {
            let c = grid[y][x];

            if c.is_ascii_digit() {
                let x_start = x;

                let mut digits = vec![c];

                x += 1;

                while x < grid[y].len() {
                    let c = grid[y][x];

                    if c.is_ascii_digit() {
                        digits.push(c);
                    } else {
                        break;
                    }

                    x += 1;
                }

                let x_end = x - 1;

                let num = digits.iter().collect::<String>().parse::<isize>().unwrap();

                let bounding_box_x_start = 0.max((x_start as isize) - 1);
                let bounding_box_x_end = ((grid[y].len() as isize) - 1).min((x_end as isize) + 1);

                let bounding_box_y_start = 0.max((y as isize) - 1);
                let bounding_box_y_end = ((grid[y].len() as isize) - 1).min((y as isize) + 1);

                'bounding: for yb in bounding_box_y_start..=bounding_box_y_end {
                    for xb in bounding_box_x_start..=bounding_box_x_end {
                        if yb == (y as isize) && xb >= (x_start as isize) && xb <= (x_end as isize)
                        {
                            continue;
                        }

                        let cb = grid[yb as usize][xb as usize];

                        if !cb.is_ascii_digit() && cb != '.' {
                            sum += num;
                            break 'bounding;
                        }
                    }
                }
            }

            x += 1;
        }

        y += 1;
    }

    sum
}

fn part2(input: &str) -> isize {
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    fn find_full_number(grid: &Vec<Vec<char>>, x: usize, y: usize) -> (isize, usize, usize, usize) {
        let mut x_start = x;
        let mut x_end = x;

        while x_start > 0 && grid[y][x_start - 1].is_ascii_digit() {
            x_start -= 1;
        }

        while x_end < grid[y].len() - 1 && grid[y][x_end + 1].is_ascii_digit() {
            x_end += 1;
        }

        let mut digits = vec![];

        for x in x_start..=x_end {
            digits.push(grid[y][x]);
        }

        let num = digits.iter().collect::<String>().parse::<isize>().unwrap();

        (num, x_start, x_end, y)
    }

    let mut sum = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let c = grid[y][x];

            if c == '*' {
                let mut adjacent_numbers: Vec<(isize, usize, usize, usize)> = vec![];

                let bounding_box_x_start = 0.max((x as isize) - 1);
                let bounding_box_x_end = ((grid[y].len() as isize) - 1).min((x as isize) + 1);

                let bounding_box_y_start = 0.max((y as isize) - 1);
                let bounding_box_y_end = ((grid[y].len() as isize) - 1).min((y as isize) + 1);

                for yb in bounding_box_y_start..=bounding_box_y_end {
                    for xb in bounding_box_x_start..=bounding_box_x_end {
                        if yb == (y as isize) && xb == (x as isize) {
                            continue;
                        }

                        let cb = grid[yb as usize][xb as usize];

                        if cb.is_ascii_digit() {
                            let found = find_full_number(&grid, xb as usize, yb as usize);

                            if adjacent_numbers.iter().any(|existing| {
                                existing.1 == found.1
                                    && existing.2 == found.2
                                    && existing.3 == found.3
                            }) {
                                continue;
                            }

                            adjacent_numbers.push(found);
                        }
                    }
                }

                if adjacent_numbers.len() == 2 {
                    sum += adjacent_numbers[0].0 * adjacent_numbers[1].0;
                }
            }
        }
    }

    sum
}
