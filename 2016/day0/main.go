package main

import (
	"fmt"
	"os"
)

func main() {
	b, err := os.ReadFile("day0/input.txt") // <-- Adjust this
	if err != nil {
		panic(err)
	}

	s := string(b)

	fmt.Printf("Part 1: %v\n", part1(s))
	fmt.Printf("Part 2: %v\n", part2(s))
}

func part1(s string) int {
	panic("todo")
}

func part2(s string) int {
	panic("todo")
}
