use itertools::{Itertools, MinMaxResult};

use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    let map = Map::from(input);

    map.regions()
        .iter()
        .map(|r| r.area() * r.perimeter())
        .sum::<usize>()
        .to_string()
}

fn part2(input: &str) -> String {
    let map = Map::from(input);

    map.regions()
        .iter()
        .map(|r| r.area() * r.corners())
        .sum::<usize>()
        .to_string()
}

#[derive(Debug, Clone)]
struct Map {
    plants: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl Map {
    fn from(input: &str) -> Self {
        let plants: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();

        let height = plants.len();
        let width = plants[0].len();

        Self {
            plants,
            height,
            width,
        }
    }

    fn regions(&self) -> Vec<Region> {
        let mut rs: Vec<Region> = Vec::new();

        for (y, row) in self.plants.iter().enumerate() {
            for (x, &c) in row.iter().enumerate() {
                if rs.iter().any(|r| r.plots.contains(&(x, y))) {
                    continue;
                }

                let mut plots = Vec::new();
                let mut poi = vec![(x, y)];

                while let Some((px, py)) = poi.pop() {
                    let pc = self.plants[py][px];

                    if pc != c {
                        continue;
                    }

                    if plots.contains(&(px, py)) {
                        continue;
                    }

                    if rs.iter().any(|r| r.plots.contains(&(px, py))) {
                        continue;
                    }

                    plots.push((px, py));

                    let candidates = [
                        (px > 0).then(|| (px - 1, py)),
                        (px < self.width - 1).then(|| (px + 1, py)),
                        (py > 0).then(|| (px, py - 1)),
                        (py < self.height - 1).then(|| (px, py + 1)),
                    ];

                    poi.extend(candidates.iter().flatten());
                }

                rs.push(Region::new(plots));
            }
        }

        rs
    }
}

#[derive(Debug, Clone)]
struct Region {
    plots: Vec<(usize, usize)>,
    bounding: ((usize, usize), (usize, usize)),
}

impl Region {
    fn new(plots: Vec<(usize, usize)>) -> Self {
        let (&xmin, &xmax) = match plots.iter().map(|(x, _)| x).minmax() {
            MinMaxResult::NoElements => malformed("2024", "12"),
            MinMaxResult::OneElement(x) => (x, x),
            MinMaxResult::MinMax(xmin, xmax) => (xmin, xmax),
        };

        let (&ymin, &ymax) = match plots.iter().map(|(_, y)| y).minmax() {
            MinMaxResult::NoElements => malformed("2024", "12"),
            MinMaxResult::OneElement(y) => (y, y),
            MinMaxResult::MinMax(ymin, ymax) => (ymin, ymax),
        };

        Self {
            plots,
            bounding: ((xmin, ymin), (xmax, ymax)),
        }
    }

    fn area(&self) -> usize {
        self.plots.len()
    }

    fn perimeter(&self) -> usize {
        let mut p = 0;

        for (x, y) in &self.plots {
            if *x == 0 || !self.plots.contains(&(x - 1, *y)) {
                p += 1;
            }

            if !self.plots.contains(&(x + 1, *y)) {
                p += 1;
            }

            if *y == 0 || !self.plots.contains(&(*x, y - 1)) {
                p += 1;
            }

            if !self.plots.contains(&(*x, y + 1)) {
                p += 1;
            }
        }

        p
    }

    fn corners(&self) -> usize {
        let mut cs = 0;

        let ((xmin, ymin), (xmax, ymax)) = self.bounding;

        for (xl, xr) in ((xmin as isize - 1)..=(xmax as isize + 1)).tuple_windows() {
            for (yt, yb) in ((ymin as isize - 1)..=(ymax as isize + 1)).tuple_windows() {
                let tl = xl != -1 && yt != -1 && self.plots.contains(&(xl as usize, yt as usize));
                let tr = yt != -1 && self.plots.contains(&(xr as usize, yt as usize));
                let bl = xl != -1 && self.plots.contains(&(xl as usize, yb as usize));
                let br = self.plots.contains(&(xr as usize, yb as usize));

                let t = tl != tr;
                let r = tr != br;
                let b = bl != br;
                let l = tl != bl;

                cs += if t && r && b && l {
                    2
                } else if (t != b) && (l != r) {
                    1
                } else {
                    0
                }
            }
        }

        cs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
            ),
            "1930"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
            ),
            "1206"
        );
    }
}
