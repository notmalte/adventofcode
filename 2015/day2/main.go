package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	b, err := os.ReadFile("day2/input.txt")
	if err != nil {
		panic(err)
	}

	s := string(b)

	fmt.Printf("Part 1: %v\n", part1(s))
	fmt.Printf("Part 2: %v\n", part2(s))
}

func part1(s string) int {
	lines := strings.Split(strings.TrimSpace(s), "\n")

	total := 0
	for _, l := range lines {
		dimensions := strings.Split(l, "x")
		l, _ := strconv.Atoi(dimensions[0])
		w, _ := strconv.Atoi(dimensions[1])
		h, _ := strconv.Atoi(dimensions[2])

		m := min(l*w, w*h, h*l)

		total += (2*l*w + 2*w*h + 2*h*l) + m
	}

	return total
}

func part2(s string) int {
	lines := strings.Split(strings.TrimSpace(s), "\n")

	total := 0
	for _, l := range lines {
		dimensions := strings.Split(l, "x")
		l, _ := strconv.Atoi(dimensions[0])
		w, _ := strconv.Atoi(dimensions[1])
		h, _ := strconv.Atoi(dimensions[2])

		m := min(2*(l+w), 2*(w+h), 2*(h+l))

		total += (l * w * h) + m
	}

	return total
}

func min(ints ...int) int {
	if len(ints) == 0 {
		panic("min() called without ints")
	}

	min := ints[0]
	for i := 1; i < len(ints); i++ {
		if ints[i] < min {
			min = ints[i]
		}
	}

	return min
}
