package main

import (
	"fmt"
	"os"
)

func main() {
	b, err := os.ReadFile("day11/input.txt")
	if err != nil {
		panic(err)
	}

	s := string(b)

	fmt.Printf("Part 1: %v\n", part1(s))
	fmt.Printf("Part 2: %v\n", part2(s))
}

func part1(s string) string {
	limit := 1_000_000

	for i := 0; i < limit; i++ {
		if validPassword(s) {
			return s
		}

		s = incrementPassword(s)
	}

	panic("did not find password")
}

func part2(s string) string {
	return part1(incrementPassword(part1(s)))
}

func incrementPassword(s string) string {
	runes := []rune(s)

	for i := len(runes) - 1; i >= 0; i-- {
		if runes[i] < 'z' {
			runes[i]++

			return string(runes)
		}

		runes[i] = 'a'
	}

	panic("cannot increment")
}

func includesThreeIncreasingLetters(s string) bool {
	runes := []rune(s)

	for i := 0; i < len(runes)-2; i++ {
		if runes[i] <= 'x' && runes[i]+1 == runes[i+1] && runes[i+1]+1 == runes[i+2] {
			return true
		}
	}

	return false
}

func doesNotIncludeConfusingLetters(s string) bool {
	for _, r := range s {
		if r == 'i' || r == 'o' || r == 'l' {
			return false
		}
	}
	return true
}

func includesTwoPairs(s string) bool {
	runes := []rune(s)
	count := 0

	for i := 0; i < len(runes)-1; i++ {
		if runes[i] == runes[i+1] {
			count++
			i++
		}
	}

	return count >= 2
}

func validPassword(s string) bool {
	return includesThreeIncreasingLetters(s) && doesNotIncludeConfusingLetters(s) && includesTwoPairs(s)
}
