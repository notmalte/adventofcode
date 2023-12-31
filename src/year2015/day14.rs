use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

const TOTAL_TIME: i64 = 2503;

fn part1(input: &str) -> String {
    let mut parsed = parse_input(input);

    for _ in 0..TOTAL_TIME {
        for reindeer in parsed.iter_mut() {
            reindeer.tick();
        }
    }

    parsed.iter().map(|r| r.distance).max().unwrap().to_string()
}

fn part2(input: &str) -> String {
    let mut parsed = parse_input(input);

    for _ in 0..TOTAL_TIME {
        for reindeer in parsed.iter_mut() {
            reindeer.tick();
        }

        let max_distance = parsed.iter().map(|r| r.distance).max().unwrap();

        for reindeer in parsed.iter_mut() {
            if reindeer.distance == max_distance {
                reindeer.score += 1;
            }
        }
    }

    parsed.iter().map(|r| r.score).max().unwrap().to_string()
}

#[derive(Debug)]
struct Reindeer {
    speed: i64,
    fly_time: i64,
    rest_time: i64,
    distance: i64,
    is_resting: bool,
    rest_time_done: i64,
    fly_time_done: i64,
    score: i64,
}

impl Reindeer {
    fn tick(&mut self) {
        if self.is_resting {
            self.rest_time_done += 1;

            if self.rest_time_done == self.rest_time {
                self.is_resting = false;
                self.rest_time_done = 0;
            }
        } else {
            self.distance += self.speed;
            self.fly_time_done += 1;

            if self.fly_time_done == self.fly_time {
                self.is_resting = true;
                self.fly_time_done = 0;
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<Reindeer> {
    let re = regex::Regex::new(
        r"^(.+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds\.$",
    )
    .unwrap();

    let mut reindeer = vec![];

    for line in input.lines() {
        let caps = re.captures(line).unwrap_or_else(|| malformed("2015", "14"));

        let speed = caps[2].parse::<i64>().unwrap();
        let fly_time = caps[3].parse::<i64>().unwrap();
        let rest_time = caps[4].parse::<i64>().unwrap();

        reindeer.push(Reindeer {
            speed,
            fly_time,
            rest_time,
            distance: 0,
            is_resting: false,
            rest_time_done: 0,
            fly_time_done: 0,
            score: 0,
        });
    }

    reindeer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(r"Foo can fly 1 km/s for 1 seconds, but then must rest for 1 seconds."),
            ((TOTAL_TIME as f64 / 2.0).ceil()).to_string()
        );

        assert_eq!(
            part1(r"Foo can fly 0 km/s for 1 seconds, but then must rest for 1 seconds."),
            "0"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(r"Foo can fly 1 km/s for 1 seconds, but then must rest for 1 seconds."),
            (TOTAL_TIME).to_string()
        );
    }
}
