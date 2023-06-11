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
	b, err := os.ReadFile("day13/input.txt")
	if err != nil {
		panic(err)
	}

	s := string(b)

	fmt.Printf("Part 1: %v\n", part1(s))
	fmt.Printf("Part 2: %v\n", part2(s))
}

func part1(s string) int {
	m := parseInput(s)

	people := []string{}
	for k := range m {
		people = append(people, k)
	}

	return findMaxScore(m, len(people), nil)
}

func part2(s string) int {
	m := parseInput(s)

	m["me"] = map[string]int{}

	for k := range m {
		m["me"][k] = 0
		m[k]["me"] = 0
	}

	people := []string{}
	for k := range m {
		people = append(people, k)
	}

	return findMaxScore(m, len(people), nil)
}

func parseInput(s string) map[string]map[string]int {
	re := regexp.MustCompile(`^(?P<name1>\S+) would (?P<direction>lose|gain) (?P<amount>\d+) happiness units by sitting next to (?P<name2>\S+).$`)

	m := map[string]map[string]int{}

	lines := strings.Split(strings.TrimSpace(s), "\n")

	for _, l := range lines {
		matches := re.FindStringSubmatch(l)

		name1 := matches[re.SubexpIndex("name1")]
		direction := matches[re.SubexpIndex("direction")]
		amount, _ := strconv.Atoi(matches[re.SubexpIndex("amount")])
		name2 := matches[re.SubexpIndex("name2")]

		if _, ok := m[name1]; !ok {
			m[name1] = map[string]int{}
		}

		if direction == "lose" {
			amount = -amount
		}

		m[name1][name2] = amount
	}

	return m
}

func findMaxScore(m map[string]map[string]int, l int, s []string) int {
	if s == nil {
		max := math.MinInt

		for k := range m {
			score := findMaxScore(m, l, []string{k})
			if score > max {
				max = score
			}
		}

		return max
	}

	if len(s) == l {
		score := 0

		for i, p := range s {
			score += m[p][s[(i+1)%l]] + m[p][s[(i+l-1)%l]]
		}

		return score
	}

	candidates := []string{}
OUTER:
	for k := range m {
		for _, p := range s {
			if p == k {
				continue OUTER
			}
		}
		candidates = append(candidates, k)
	}

	max := math.MinInt
	for _, c := range candidates {
		newStack := make([]string, len(s)+1)
		copy(newStack, s)
		newStack[len(s)] = c

		score := findMaxScore(m, l, newStack)
		if score > max {
			max = score
		}

	}
	return max
}
