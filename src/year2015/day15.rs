use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    let parsed = parse_input(input);

    find_best_score(&parsed, None, vec![]).to_string()
}

fn part2(input: &str) -> String {
    let parsed = parse_input(input);

    find_best_score(&parsed, Some(500), vec![]).to_string()
}

#[derive(Debug, Clone)]
struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

#[derive(Debug, Clone)]
struct Portion {
    ingredient: Ingredient,
    amount: i64,
}

#[derive(Debug, Clone)]
struct Recipe {
    portions: Vec<Portion>,
}

impl Recipe {
    fn score(&self) -> i64 {
        let mut capacity = 0;
        let mut durability = 0;
        let mut flavor = 0;
        let mut texture = 0;

        for portion in &self.portions {
            capacity += portion.ingredient.capacity * portion.amount;
            durability += portion.ingredient.durability * portion.amount;
            flavor += portion.ingredient.flavor * portion.amount;
            texture += portion.ingredient.texture * portion.amount;
        }

        if capacity < 0 || durability < 0 || flavor < 0 || texture < 0 {
            return 0;
        }

        capacity * durability * flavor * texture
    }
}

fn parse_input(input: &str) -> Vec<Ingredient> {
    let re = regex::Regex::new(
        r"^(.+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)$",
    )
    .unwrap();

    let mut ingredients = vec![];

    for line in input.lines() {
        let caps = re.captures(line).unwrap_or_else(|| malformed("2015", "15"));

        let capacity = caps[2].parse::<i64>().unwrap();
        let durability = caps[3].parse::<i64>().unwrap();
        let flavor = caps[4].parse::<i64>().unwrap();
        let texture = caps[5].parse::<i64>().unwrap();
        let calories = caps[6].parse::<i64>().unwrap();

        ingredients.push(Ingredient {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        });
    }

    ingredients
}

fn find_best_score(
    ingredients: &Vec<Ingredient>,
    calories_limit: Option<i64>,
    amounts: Vec<i64>,
) -> i64 {
    if amounts.len() == ingredients.len() {
        let mut portions = vec![];

        let mut calories = 0;

        for (ingredient, amount) in ingredients.iter().zip(&amounts) {
            portions.push(Portion {
                ingredient: ingredient.clone(),
                amount: *amount,
            });

            calories += ingredient.calories * amount;
        }

        if let Some(calories_limit) = calories_limit {
            if calories != calories_limit {
                return i64::MIN;
            }
        }

        let recipe = Recipe { portions };

        return recipe.score();
    }

    let capacity = 100 - amounts.iter().sum::<i64>();

    let mut max = i64::MIN;

    for i in 0..=capacity {
        let mut new_amounts = amounts.clone();
        new_amounts.push(i);

        let score = find_best_score(ingredients, calories_limit, new_amounts);

        if score > max {
            max = score;
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"
            ),
            "62842880"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"
            ),
            "57600000"
        );
    }
}
