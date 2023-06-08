package main

import (
	"fmt"
	"os"
)

func main() {
	b, err := os.ReadFile("day1/input.txt")
	if err != nil {
		panic(err)
	}

	s := string(b)

	fmt.Printf("Part 1: %v\n", part1(s))
	fmt.Printf("Part 2: %v\n", part2(s))
}

func part1(s string) int {
	floor := 0

	for _, r := range s {
		switch r {
		case '(':
			floor++
		case ')':
			floor--
		}
	}

	return floor
}

func part2(s string) int {
	pos := 1
	floor := 0

	for _, r := range s {
		switch r {
		case '(':
			floor++
		case ')':
			floor--
			if floor < 0 {
				return pos
			}
		}

		pos++
	}

	return pos
}
