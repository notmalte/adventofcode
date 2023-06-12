package main

import (
	"fmt"
	"math"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func main() {
	b, err := os.ReadFile("day15/input.txt")
	if err != nil {
		panic(err)
	}

	s := string(b)

	fmt.Printf("Part 1: %v\n", part1(s))
	fmt.Printf("Part 2: %v\n", part2(s))
}

func part1(s string) int {
	ingredients := parseIngredients(s)

	return findBestRecipe(ingredients, -1, nil)
}

func part2(s string) int {
	ingredients := parseIngredients(s)

	return findBestRecipe(ingredients, 500, nil)
}

type ingredient struct {
	name       string
	capacity   int
	durability int
	flavor     int
	texture    int
	calories   int
}

type portion struct {
	ingredient ingredient
	amount     int
}

type recipe struct {
	portions []portion
}

func (r *recipe) score() int {
	capacity := 0
	durability := 0
	flavor := 0
	texture := 0

	for _, p := range r.portions {
		capacity += p.ingredient.capacity * p.amount
		durability += p.ingredient.durability * p.amount
		flavor += p.ingredient.flavor * p.amount
		texture += p.ingredient.texture * p.amount
	}

	if capacity < 0 || durability < 0 || flavor < 0 || texture < 0 {
		return 0
	}

	return capacity * durability * flavor * texture
}

func parseIngredients(s string) []ingredient {
	lines := strings.Split(strings.TrimSpace(s), "\n")

	ingredients := []ingredient{}

	re := regexp.MustCompile(`(?P<name>\w+): capacity (?P<capacity>-?\d+), durability (?P<durability>-?\d+), flavor (?P<flavor>-?\d+), texture (?P<texture>-?\d+), calories (?P<calories>-?\d+)`)

	for _, line := range lines {
		matches := re.FindStringSubmatch(line)

		name := matches[re.SubexpIndex("name")]
		capacity, _ := strconv.Atoi(matches[re.SubexpIndex("capacity")])
		durability, _ := strconv.Atoi(matches[re.SubexpIndex("durability")])
		flavor, _ := strconv.Atoi(matches[re.SubexpIndex("flavor")])
		texture, _ := strconv.Atoi(matches[re.SubexpIndex("texture")])
		calories, _ := strconv.Atoi(matches[re.SubexpIndex("calories")])

		ingredients = append(ingredients, ingredient{
			name:       name,
			capacity:   capacity,
			durability: durability,
			flavor:     flavor,
			texture:    texture,
			calories:   calories,
		})
	}

	return ingredients
}

func findBestRecipe(ingredients []ingredient, calories int, amounts []int) int {
	if amounts == nil {
		max := math.MinInt

		for i := 0; i <= 100; i++ {
			score := findBestRecipe(ingredients, calories, []int{i})

			if score > max {
				max = score
			}
		}

		return max
	}

	if len(amounts) == len(ingredients) {
		r := recipe{}

		c := 0

		for i, amount := range amounts {
			r.portions = append(r.portions, portion{
				ingredient: ingredients[i],
				amount:     amount,
			})

			c += ingredients[i].calories * amount
		}

		if calories != -1 && c != calories {
			return math.MinInt
		}

		return r.score()
	}

	capacity := 100

	for _, amount := range amounts {
		capacity -= amount
	}

	if capacity < 0 {
		panic("invalid capacity")
	}

	max := math.MinInt

	for i := 0; i <= capacity; i++ {
		newAmounts := append(amounts, i)

		score := findBestRecipe(ingredients, calories, newAmounts)

		if score > max {
			max = score
		}
	}

	return max
}
