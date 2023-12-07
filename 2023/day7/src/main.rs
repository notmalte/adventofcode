use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap, env, fs};

fn main() {
    let current_dir = env::current_dir().unwrap();

    let input_file = current_dir.join("day7/input.txt");

    println!("Reading input from: {}", input_file.display());

    let input = fs::read_to_string(input_file).unwrap();

    println!("Part 1: {}", part1(&input));

    println!("Part 2: {}", part2(&input));
}

fn card_value_part1(card: char) -> isize {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => panic!("invalid card"),
    }
}

fn card_value_part2(card: char) -> isize {
    match card {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        'J' => 1,
        _ => panic!("invalid card"),
    }
}

#[derive(Debug, Clone, Copy)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

fn hand_type(hand: &str) -> HandType {
    let chars = hand.chars().collect_vec();

    if chars.len() != 5 {
        panic!("invalid hand")
    }

    let mut map = HashMap::new();

    for c in chars {
        if let Some(n) = map.get(&c) {
            map.insert(c, *n + 1);
        } else {
            map.insert(c, 1);
        }
    }

    let counts = map.iter().sorted_by_key(|(_, n)| -*n).collect_vec();

    match counts.len() {
        1 => HandType::FiveOfAKind,
        2 => match *counts[0].1 {
            4 => HandType::FourOfAKind,
            _ => HandType::FullHouse,
        },
        3 => match *counts[0].1 {
            3 => HandType::ThreeOfAKind,
            _ => HandType::TwoPair,
        },
        4 => HandType::OnePair,
        5 => HandType::HighCard,
        _ => unreachable!(),
    }
}

fn compare_hand_types(a: HandType, b: HandType) -> Ordering {
    let score1 = a as isize;
    let score2 = b as isize;

    score1.cmp(&score2)
}

fn best_hand_type_variation(hand: &str) -> HandType {
    let chars = hand.chars().collect_vec();

    if chars.len() != 5 {
        panic!("invalid hand")
    }

    if !chars.iter().any(|c| *c == 'J') {
        return hand_type(hand);
    }

    let cards = [
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ];

    let mut variations: Vec<Vec<char>> = vec![];

    for i in 0..5 {
        let c = chars[i];

        if c != 'J' {
            continue;
        }

        if variations.is_empty() {
            for cv in cards {
                let mut cloned = chars.clone();
                cloned[i] = cv;

                variations.push(cloned);
            }
        } else {
            let existing = variations.clone();

            for v in existing {
                for cv in cards {
                    let mut cloned = v.clone();
                    cloned[i] = cv;

                    variations.push(cloned);
                }
            }
        }
    }

    let mut best = HandType::HighCard;

    for v in variations {
        let s = v.iter().collect::<String>();

        let ht = hand_type(&s);

        if compare_hand_types(ht, best).is_gt() {
            best = ht
        }
    }

    best
}

fn compare_hand_cards(hand1: &str, hand2: &str, value_fn: fn(char) -> isize) -> Ordering {
    let chars1 = hand1.chars().collect_vec();
    let chars2 = hand2.chars().collect_vec();

    if chars1.len() != 5 || chars2.len() != 5 {
        panic!("invalid hand")
    }

    for i in 0..5 {
        let score1 = value_fn(chars1[i]);
        let score2 = value_fn(chars2[i]);

        let cmp = score1.cmp(&score2);

        if cmp.is_ne() {
            return cmp;
        }
    }

    Ordering::Equal
}

fn part1(input: &str) -> isize {
    input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(hand, bid)| (hand, bid.parse::<isize>().unwrap()))
        .sorted_by(|(hand1, _), (hand2, _)| {
            let type_cmp = compare_hand_types(hand_type(hand1), hand_type(hand2));

            if type_cmp.is_ne() {
                type_cmp
            } else {
                compare_hand_cards(hand1, hand2, card_value_part1)
            }
        })
        .enumerate()
        .map(|(i, (_, bid))| (i as isize + 1) * bid)
        .sum()
}

fn part2(input: &str) -> isize {
    input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(hand, bid)| {
            (
                hand,
                best_hand_type_variation(hand),
                bid.parse::<isize>().unwrap(),
            )
        })
        .sorted_by(|(hand1, type1, _), (hand2, type2, _)| {
            let type_cmp = compare_hand_types(*type1, *type2);

            if type_cmp.is_ne() {
                type_cmp
            } else {
                compare_hand_cards(hand1, hand2, card_value_part2)
            }
        })
        .enumerate()
        .map(|(i, (_, _, bid))| (i as isize + 1) * bid)
        .sum()
}
