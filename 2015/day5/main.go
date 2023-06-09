package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	b, err := os.ReadFile("day5/input.txt")
	if err != nil {
		panic(err)
	}

	s := string(b)

	fmt.Printf("Part 1: %v\n", part1(s))
	fmt.Printf("Part 2: %v\n", part2(s))
}

func part1(s string) int {
	lines := strings.Split(strings.TrimSpace(s), "\n")

	c := 0

	for _, l := range lines {
		if atLeast3Vowels(l) && letterTwiceInARow(l) && doesNotContainSpecialStrings(l) {
			c++
		}
	}

	return c
}

func part2(s string) int {
	lines := strings.Split(strings.TrimSpace(s), "\n")

	c := 0

	for _, l := range lines {
		if pairOfTwoLetters(l) && repeatingLetterWithOneBetween(l) {
			c++
		}
	}

	return c
}

func atLeast3Vowels(s string) bool {
	c := 0
	for _, r := range s {
		if r == 'a' || r == 'e' || r == 'i' || r == 'o' || r == 'u' {
			c++
		}
	}
	return c >= 3
}

func letterTwiceInARow(s string) bool {
	runes := []rune(s)

	for i := 1; i < len(runes); i++ {
		if runes[i] == runes[i-1] {
			return true
		}
	}

	return false
}

func doesNotContainSpecialStrings(s string) bool {
	special := []string{
		"ab",
		"cd",
		"pq",
		"xy",
	}

	for _, sp := range special {
		if strings.Contains(s, sp) {
			return false
		}
	}

	return true
}

func pairOfTwoLetters(s string) bool {
	runes := []rune(s)

	for i := 0; i < len(runes)-3; i++ {
		for j := i + 2; j < len(runes)-1; j++ {
			if runes[i] == runes[j] && runes[i+1] == runes[j+1] {
				return true
			}
		}
	}

	return false
}

func repeatingLetterWithOneBetween(s string) bool {
	runes := []rune(s)

	for i := 0; i < len(runes)-2; i++ {
		if runes[i] == runes[i+2] {
			return true
		}
	}

	return false
}
