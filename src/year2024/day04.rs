use crate::Answer;

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    let grid = parse_input(input);

    grid.iter()
        .enumerate()
        .fold(0, |acc, (y, row)| {
            acc + row.iter().enumerate().fold(0, |acc, (x, el)| {
                acc + [
                    (0, -1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                    (0, 1),
                    (-1, 1),
                    (-1, 0),
                    (-1, -1),
                ]
                .iter()
                .fold(0, |acc, (dx, dy)| {
                    if *el == 'X'
                        && ['M', 'A', 'S'].iter().enumerate().all(|(i, c)| {
                            let nx = (x as isize) + (i as isize + 1) * dx;
                            let ny = (y as isize) + (i as isize + 1) * dy;

                            (0..(row.len() as isize)).contains(&nx)
                                && (0..(grid.len() as isize)).contains(&ny)
                                && grid[ny as usize][nx as usize] == *c
                        })
                    {
                        acc + 1
                    } else {
                        acc
                    }
                })
            })
        })
        .to_string()
}

fn part2(input: &str) -> String {
    let grid = parse_input(input);

    grid.iter()
        .enumerate()
        .fold(0, |acc, (y, row)| {
            acc + row.iter().enumerate().fold(0, |acc, (x, el)| {
                let tl = ((x as isize) - 1, (y as isize) - 1);
                let tr = ((x as isize) + 1, (y as isize) - 1);
                let bl = ((x as isize) - 1, (y as isize) + 1);
                let br = ((x as isize) + 1, (y as isize) + 1);

                if *el == 'A'
                    && [tl, tr, bl, br].iter().all(|(x, y)| {
                        (0..(row.len() as isize)).contains(x)
                            && (0..(grid.len() as isize)).contains(y)
                            && (grid[*y as usize][*x as usize] == 'M'
                                || grid[*y as usize][*x as usize] == 'S')
                    })
                    && grid[tl.1 as usize][tl.0 as usize] != grid[br.1 as usize][br.0 as usize]
                    && grid[tr.1 as usize][tr.0 as usize] != grid[bl.1 as usize][bl.0 as usize]
                {
                    acc + 1
                } else {
                    acc
                }
            })
        })
        .to_string()
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|s| s.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            ),
            "18"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            ),
            "9"
        );
    }
}
