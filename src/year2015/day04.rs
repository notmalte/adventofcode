use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    for i in 0.. {
        let digest = md5::compute(format!("{}{}", input, i));

        if digest[0] == 0 && digest[1] == 0 && digest[2] <= 0x0f {
            return i.to_string();
        }
    }

    malformed("2015", "04")
}

fn part2(input: &str) -> String {
    for i in 0.. {
        let digest = md5::compute(format!("{}{}", input, i));

        if digest[0] == 0 && digest[1] == 0 && digest[2] == 0 {
            return i.to_string();
        }
    }

    malformed("2015", "04")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("abcdef"), "609043");
        assert_eq!(part1("pqrstuv"), "1048970");
    }
}
