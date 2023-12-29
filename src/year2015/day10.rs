use crate::Answer;

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    let mut result = input.to_string();

    for _ in 0..40 {
        result = look_and_say(&result);
    }

    result.len().to_string()
}

fn part2(input: &str) -> String {
    let mut result = input.to_string();

    for _ in 0..50 {
        result = look_and_say(&result);
    }

    result.len().to_string()
}

fn look_and_say(input: &str) -> String {
    let mut result = String::new();

    let mut iter = input.chars().peekable();

    while let Some(c) = iter.next() {
        let mut count = 1;

        while let Some(&next) = iter.peek() {
            if next == c {
                count += 1;
                iter.next();
            } else {
                break;
            }
        }

        result.push_str(&count.to_string());
        result.push(c);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_look_and_say() {
        assert_eq!(look_and_say("1"), "11");
        assert_eq!(look_and_say("11"), "21");
        assert_eq!(look_and_say("21"), "1211");
        assert_eq!(look_and_say("1211"), "111221");
        assert_eq!(look_and_say("111221"), "312211");
    }
}
