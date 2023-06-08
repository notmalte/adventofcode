package main

import (
	"fmt"
	"os"
)

type house struct {
	x int
	y int
}

func main() {
	b, err := os.ReadFile("day3/input.txt")
	if err != nil {
		panic(err)
	}

	s := string(b)

	fmt.Printf("Part 1: %v\n", part1(s))
	fmt.Printf("Part 2: %v\n", part2(s))
}

func part1(s string) int {
	houses := map[house]int{
		{0, 0}: 1,
	}

	x := 0
	y := 0

	for _, r := range s {
		switch r {
		case '<':
			x--
		case '>':
			x++
		case '^':
			y++
		case 'v':
			y--
		}

		h := house{x, y}
		houses[h]++
	}

	count := 0
	for _, h := range houses {
		if h >= 1 {
			count++
		}
	}

	return count
}

func part2(s string) int {
	houses := map[house]int{
		{0, 0}: 2,
	}

	sx := 0
	sy := 0

	rx := 0
	ry := 0

	i := 0

	for _, r := range s {
		switch r {
		case '<':
			if i%2 == 0 {
				sx--
			} else {
				rx--
			}
		case '>':
			if i%2 == 0 {
				sx++
			} else {
				rx++
			}
		case '^':
			if i%2 == 0 {
				sy++
			} else {
				ry++
			}
		case 'v':
			if i%2 == 0 {
				sy--
			} else {
				ry--
			}
		}

		var h house
		if i%2 == 0 {
			h = house{sx, sy}
		} else {
			h = house{rx, ry}
		}
		houses[h]++

		i++
	}

	count := 0
	for _, h := range houses {
		if h >= 1 {
			count++
		}
	}

	return count
}
