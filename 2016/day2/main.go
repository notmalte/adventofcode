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

func part1(s string) string {
	lines := strings.Split(strings.TrimSpace(s), "\n")

	keypad := [][]int{
		{1, 2, 3},
		{4, 5, 6},
		{7, 8, 9},
	}

	x, y := 1, 1

	code := ""

	for _, line := range lines {
		for _, r := range line {
			switch r {
			case 'U':
				if y > 0 {
					y--
				}
			case 'D':
				if y < 2 {
					y++
				}
			case 'L':
				if x > 0 {
					x--
				}
			case 'R':
				if x < 2 {
					x++
				}
			}
		}

		code += strconv.Itoa(keypad[y][x])
	}

	return code
}

func part2(s string) string {
	lines := strings.Split(strings.TrimSpace(s), "\n")

	keypad := [][]rune{
		{0, 0, '1', 0, 0},
		{0, '2', '3', '4', 0},
		{'5', '6', '7', '8', '9'},
		{0, 'A', 'B', 'C', 0},
		{0, 0, 'D', 0, 0},
	}

	x, y := 0, 2

	code := []rune{}

	for _, line := range lines {
		for _, r := range line {
			switch r {
			case 'U':
				if y <= 0 {
					continue
				}

				if keypad[y-1][x] == 0 {
					continue
				}

				y--
			case 'D':
				if y >= 4 {
					continue
				}

				if keypad[y+1][x] == 0 {
					continue
				}

				y++
			case 'L':
				if x <= 0 {
					continue
				}

				if keypad[y][x-1] == 0 {
					continue
				}

				x--
			case 'R':
				if x >= 4 {
					continue
				}

				if keypad[y][x+1] == 0 {
					continue
				}

				x++
			}
		}

		code = append(code, keypad[y][x])
	}

	return string(code)
}
