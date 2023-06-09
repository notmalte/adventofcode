package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	b, err := os.ReadFile("day8/input.txt")
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
		c += codeToMemoryDiff(l)
	}

	return c
}

func part2(s string) int {
	lines := strings.Split(strings.TrimSpace(s), "\n")

	c := 0

	for _, l := range lines {
		c += memoryToCodeDiff(l)
	}

	return c
}

func codeToMemoryDiff(s string) int {
	runes := []rune(s)

	lengthInCode := len(runes)

	runes = runes[1 : len(runes)-1]

	lengthInMemory := 0

	for i := 0; i < len(runes); i++ {
		lengthInMemory++

		if runes[i] != '\\' {
			continue
		}

		if runes[i+1] == '"' || runes[i+1] == '\\' {
			i++
			continue
		}

		if runes[i+1] == 'x' {
			i += 3
			continue
		}
	}

	return lengthInCode - lengthInMemory
}

func memoryToCodeDiff(s string) int {
	runes := []rune(s)

	lengthInMemory := len(runes)

	lengthInCode := 2

	for i := 0; i < len(runes); i++ {
		lengthInCode++

		if runes[i] == '"' || runes[i] == '\\' {
			lengthInCode++
		}
	}

	return lengthInCode - lengthInMemory
}
