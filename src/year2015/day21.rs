use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    let boss = Stats::from_input(input);

    let shop = Shop::new();

    let mut min_cost = i64::MAX;

    let variations = player_variations(&shop);

    for (player, cost) in variations {
        if fight(&player, &boss) == Outcome::Win {
            min_cost = min_cost.min(cost);
        }
    }

    min_cost.to_string()
}

fn part2(input: &str) -> String {
    let boss = Stats::from_input(input);

    let shop = Shop::new();

    let mut max_cost = i64::MIN;

    let variations = player_variations(&shop);

    for (player, cost) in variations {
        if fight(&player, &boss) == Outcome::Lose {
            max_cost = max_cost.max(cost);
        }
    }

    max_cost.to_string()
}

#[derive(Clone, Debug)]
struct Stats {
    hp: i64,
    damage: i64,
    armor: i64,
}

impl Stats {
    fn from_input(input: &str) -> Self {
        let lines = input
            .lines()
            .map(|l| {
                l.split_once(": ")
                    .unwrap_or_else(|| malformed("2015", "21"))
                    .1
                    .parse()
                    .unwrap_or_else(|_| malformed("2015", "21"))
            })
            .collect::<Vec<_>>();

        if lines.len() != 3 {
            malformed("2015", "21");
        }

        Self {
            hp: lines[0],
            damage: lines[1],
            armor: lines[2],
        }
    }

    fn apply_items(items: &[&Item]) -> (Stats, i64) {
        let mut stats = Stats {
            hp: 100,
            damage: 0,
            armor: 0,
        };
        let mut cost = 0;

        for item in items {
            stats.damage += item.damage;
            stats.armor += item.armor;
            cost += item.cost;
        }

        (stats, cost)
    }
}

#[derive(PartialEq, Debug)]
enum Outcome {
    Win,
    Lose,
}

#[derive(Clone, Debug)]
struct Item {
    cost: i64,
    damage: i64,
    armor: i64,
}

impl Item {
    fn from_tuple(tuple: (i64, i64, i64)) -> Self {
        Self {
            cost: tuple.0,
            damage: tuple.1,
            armor: tuple.2,
        }
    }
}

struct Shop {
    weapons: Vec<Item>,
    armor: Vec<Item>,
    rings: Vec<Item>,
}

impl Shop {
    fn new() -> Self {
        let weapons = vec![(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)];

        let armor = vec![(13, 0, 1), (31, 0, 2), (53, 0, 3), (75, 0, 4), (102, 0, 5)];

        let rings = vec![
            (25, 1, 0),
            (50, 2, 0),
            (100, 3, 0),
            (20, 0, 1),
            (40, 0, 2),
            (80, 0, 3),
        ];

        Self {
            weapons: weapons.into_iter().map(Item::from_tuple).collect(),
            armor: armor.into_iter().map(Item::from_tuple).collect(),
            rings: rings.into_iter().map(Item::from_tuple).collect(),
        }
    }

    fn weapon_variations(&self) -> Vec<Item> {
        self.weapons.clone()
    }

    fn armor_variations(&self) -> Vec<Option<Item>> {
        let mut variations = Vec::new();

        variations.push(None);

        for armor in self.armor.iter() {
            variations.push(Some(armor.clone()));
        }

        variations
    }

    fn ring_variations(&self) -> Vec<(Option<Item>, Option<Item>)> {
        let mut variations = Vec::new();

        variations.push((None, None));

        for ring in self.rings.iter() {
            variations.push((Some(ring.clone()), None));
        }

        for i in 0..self.rings.len() {
            for j in i + 1..self.rings.len() {
                variations.push((Some(self.rings[i].clone()), Some(self.rings[j].clone())));
            }
        }

        variations
    }
}

fn player_variations(shop: &Shop) -> Vec<(Stats, i64)> {
    let mut variations = Vec::new();

    let weapon_variations = shop.weapon_variations();
    let armor_variations = shop.armor_variations();
    let ring_variations = shop.ring_variations();

    for weapon in &weapon_variations {
        for armor in &armor_variations {
            for rings in &ring_variations {
                let mut items = Vec::new();

                items.push(weapon);

                if let Some(armor) = armor {
                    items.push(armor);
                }

                if let Some(ring) = &rings.0 {
                    items.push(ring);
                }

                if let Some(ring) = &rings.1 {
                    items.push(ring);
                }

                let (player, cost) = Stats::apply_items(&items);

                variations.push((player, cost));
            }
        }
    }

    variations
}

fn fight(player: &Stats, boss: &Stats) -> Outcome {
    let mut player = player.clone();
    let mut boss = boss.clone();

    loop {
        boss.hp -= 1.max(player.damage - boss.armor);
        if boss.hp <= 0 {
            return Outcome::Win;
        }

        player.hp -= 1.max(boss.damage - player.armor);
        if player.hp <= 0 {
            return Outcome::Lose;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fight() {
        let player = Stats {
            hp: 8,
            damage: 5,
            armor: 5,
        };

        let boss = Stats {
            hp: 12,
            damage: 7,
            armor: 2,
        };

        assert_eq!(fight(&player, &boss), Outcome::Win);

        let player = Stats {
            hp: 8,
            damage: 5,
            armor: 5,
        };

        let boss = Stats {
            hp: 12,
            damage: 7,
            armor: 3,
        };

        assert_eq!(fight(&player, &boss), Outcome::Lose);
    }

    #[test]
    fn test_player_variations() {
        let shop = Shop::new();

        let variations = player_variations(&shop);

        assert_eq!(variations.len(), 5 * 6 * (1 + 6 + 15)); // 5 weapons * 6 armors * (no rings + 6 rings + 15 ring pairs)
    }
}
