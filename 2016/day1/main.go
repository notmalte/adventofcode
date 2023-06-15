package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
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
	moves := strings.Split(s, ", ")

	degrees := 0
	x, y := 0, 0

	for _, move := range moves {
		runes := []rune(move)

		if runes[0] == 'R' {
			degrees = (degrees + 90) % 360
		} else if runes[0] == 'L' {
			degrees = (degrees + 270) % 360
		} else {
			panic("unknown turn")
		}

		distance, err := strconv.Atoi(string(runes[1:]))
		if err != nil {
			panic(err)
		}

		switch degrees {
		case 0:
			y += distance
		case 90:
			x += distance
		case 180:
			y -= distance
		case 270:
			x -= distance
		}
	}

	if x < 0 {
		x = -x
	}
	if y < 0 {
		y = -y
	}

	return x + y
}

type point struct {
	x, y int
}

func part2(s string) int {
	moves := strings.Split(s, ", ")

	degrees := 0
	x, y := 0, 0

	visited := map[point]struct{}{}

	for _, move := range moves {
		runes := []rune(move)

		if runes[0] == 'R' {
			degrees = (degrees + 90) % 360
		} else if runes[0] == 'L' {
			degrees = (degrees + 270) % 360
		} else {
			panic("unknown turn")
		}

		distance, err := strconv.Atoi(string(runes[1:]))
		if err != nil {
			panic(err)
		}

		xTarget, yTarget, xDelta, yDelta := x, y, 0, 0

		switch degrees {
		case 0:
			yTarget += distance
			yDelta = 1
		case 90:
			xTarget += distance
			xDelta = 1
		case 180:
			yTarget -= distance
			yDelta = -1
		case 270:
			xTarget -= distance
			xDelta = -1
		}

		for x != xTarget || y != yTarget {
			x += xDelta
			y += yDelta

			p := point{
				x: x,
				y: y,
			}

			if _, ok := visited[p]; ok {
				if x < 0 {
					x = -x
				}
				if y < 0 {
					y = -y
				}

				return x + y
			}

			visited[p] = struct{}{}
		}
	}

	panic("no point visited twice")
}
