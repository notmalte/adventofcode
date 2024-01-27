use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    let mut result = String::new();

    for i in 0.. {
        let digest = md5::compute(format!("{}{}", input, i));

        if digest[0] == 0 && digest[1] == 0 && digest[2] <= 0x0f {
            result.push_str(&format!("{:x}", digest[2]));
        }

        if result.len() == 8 {
            return result;
        }
    }

    malformed("2016", "05")
}

fn part2(input: &str) -> String {
    let mut result = vec!['_'; 8];

    for i in 0.. {
        let digest = md5::compute(format!("{}{}", input, i));

        if digest[0] == 0 && digest[1] == 0 && digest[2] <= 0x07 {
            let index = digest[2] as usize;

            if result[index] == '_' {
                result[index] = format!("{:x}", digest[3] >> 4).chars().next().unwrap();
            }
        }

        if !result.contains(&'_') {
            return result.into_iter().collect();
        }
    }

    malformed("2016", "05")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("abc"), "18f47a30");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("abc"), "05ace8e3");
    }
}
