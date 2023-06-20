package main

import (
	"fmt"
	"math"
	"os"
	"strings"
)

func main() {
	b, err := os.ReadFile("day6/input.txt")
	if err != nil {
		panic(err)
	}

	s := string(b)

	fmt.Printf("Part 1: %v\n", part1(s))
	fmt.Printf("Part 2: %v\n", part2(s))
}

func part1(s string) string {
	counts := getCounts(s)

	result := []rune{}

	for _, count := range counts {
		max := math.MinInt
		var maxChar rune

		for c, n := range count {
			if n > max {
				max = n
				maxChar = c
			}
		}

		result = append(result, maxChar)
	}

	return string(result)
}

func part2(s string) string {
	counts := getCounts(s)

	result := []rune{}

	for _, count := range counts {
		min := math.MaxInt
		var minChar rune

		for c, n := range count {
			if n < min {
				min = n
				minChar = c
			}
		}

		result = append(result, minChar)
	}

	return string(result)
}

func getCounts(s string) []map[rune]int {
	lines := strings.Split(strings.TrimSpace(s), "\n")

	length := len(lines[0])

	counts := []map[rune]int{}

	for i := 0; i < length; i++ {
		counts = append(counts, map[rune]int{})
	}

	for _, line := range lines {
		runes := []rune(line)

		for i, c := range runes {
			counts[i][c]++
		}
	}

	return counts
}
