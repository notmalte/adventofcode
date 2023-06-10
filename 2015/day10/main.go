package main

import (
	"fmt"
	"os"
)

func main() {
	b, err := os.ReadFile("day10/input.txt")
	if err != nil {
		panic(err)
	}

	s := string(b)

	fmt.Printf("Part 1: %v\n", part1(s))
	fmt.Printf("Part 2: %v\n", part2(s))
}

func part1(s string) int {
	for i := 0; i < 40; i++ {
		s = lookAndSay(s)
	}

	return len(s)
}

func part2(s string) int {
	for i := 0; i < 50; i++ {
		s = lookAndSay(s)
	}

	return len(s)
}

func lookAndSay(s string) string {
	runes := []rune(s)

	var lastRune rune = 0
	streak := 0

	result := []rune{}

	for i := 0; i < len(runes); i++ {
		if runes[i] == lastRune {
			streak++
		} else {
			if streak != 0 {
				result = append(result, '0'+rune(streak), lastRune)
			}

			streak = 1
			lastRune = runes[i]
		}
	}

	result = append(result, '0'+rune(streak), lastRune)

	return string(result)
}
