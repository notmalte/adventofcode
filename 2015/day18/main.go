package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	b, err := os.ReadFile("day18/input.txt")
	if err != nil {
		panic(err)
	}

	s := string(b)

	fmt.Printf("Part 1: %v\n", part1(s))
	fmt.Printf("Part 2: %v\n", part2(s))
}

func part1(s string) int {
	g := parseGrid(s)

	for i := 0; i < 100; i++ {
		g = g.nextGrid()
	}

	return g.countLights()
}

func part2(s string) int {
	g := parseGrid(s)

	g.turnOnCorners()

	for i := 0; i < 100; i++ {
		g = g.nextGrid()

		g.turnOnCorners()
	}

	return g.countLights()
}

type grid struct {
	values [][]bool
}

func (g grid) String() string {
	runes := make([]rune, 0, len(g.values)*len(g.values[0]))

	for _, yv := range g.values {
		for _, xv := range yv {
			if xv {
				runes = append(runes, '#')
			} else {
				runes = append(runes, '.')
			}
		}
		runes = append(runes, '\n')
	}

	return string(runes)
}

func (g *grid) neighbors(x, y int) int {
	ySize := len(g.values)
	xSize := len(g.values[0])

	xMin := x - 1
	if xMin < 0 {
		xMin = 0
	}

	yMin := y - 1
	if yMin < 0 {
		yMin = 0
	}

	xMax := x + 1
	if xMax >= xSize {
		xMax = xSize - 1
	}

	yMax := y + 1
	if yMax >= ySize {
		yMax = ySize - 1
	}

	neighbors := 0

	for xCur := xMin; xCur <= xMax; xCur++ {
		for yCur := yMin; yCur <= yMax; yCur++ {
			if x == xCur && y == yCur {
				continue
			}

			if g.values[yCur][xCur] {
				neighbors++
			}
		}
	}

	return neighbors
}

func (g *grid) nextState(x, y int) bool {
	n := g.neighbors(x, y)

	v := g.values[y][x]

	if v {
		return n == 2 || n == 3
	} else {
		return n == 3
	}
}

func (g *grid) nextGrid() grid {
	v := [][]bool{}

	for y, yv := range g.values {
		vl := []bool{}

		for x := range yv {
			vl = append(vl, g.nextState(x, y))
		}

		v = append(v, vl)
	}

	return grid{
		values: v,
	}
}

func (g *grid) countLights() int {
	count := 0

	for _, yv := range g.values {
		for _, xv := range yv {
			if xv {
				count++
			}
		}
	}

	return count
}

func (g *grid) turnOnCorners() {
	g.values[0][0] = true
	g.values[0][len(g.values[0])-1] = true
	g.values[len(g.values)-1][0] = true
	g.values[len(g.values)-1][len(g.values[len(g.values)-1])-1] = true
}

func parseGrid(s string) grid {
	lines := strings.Split(strings.TrimSpace(s), "\n")

	v := [][]bool{}

	for _, l := range lines {
		vl := []bool{}

		for _, r := range l {
			vl = append(vl, r == '#')
		}

		v = append(v, vl)
	}

	return grid{
		values: v,
	}
}
