package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func main() {
	b, err := os.ReadFile("day6/input.txt")
	if err != nil {
		panic(err)
	}

	s := string(b)

	fmt.Printf("Part 1: %v\n", part1(s))
	fmt.Printf("Part 2: %v\n", part2(s))
}

var size = 1_000

func part1(s string) int {
	lights := make([][]bool, size)
	for i := range lights {
		lights[i] = make([]bool, size)
	}

	lines := strings.Split(strings.TrimSpace(s), "\n")

	for _, l := range lines {
		startX, startY, endX, endY := parseCoordinates(l)

		for x := startX; x <= endX; x++ {
			for y := startY; y <= endY; y++ {
				if strings.HasPrefix(l, "turn on") {
					lights[y][x] = true
				} else if strings.HasPrefix(l, "turn off") {
					lights[y][x] = false
				} else if strings.HasPrefix(l, "toggle") {
					lights[y][x] = !lights[y][x]
				}
			}
		}
	}

	c := 0
	for _, ls := range lights {
		for _, l := range ls {
			if l {
				c++
			}
		}
	}

	return c
}

func part2(s string) int {
	lights := make([][]int, size)
	for i := range lights {
		lights[i] = make([]int, size)
	}

	lines := strings.Split(strings.TrimSpace(s), "\n")

	for _, l := range lines {
		startX, startY, endX, endY := parseCoordinates(l)

		for x := startX; x <= endX; x++ {
			for y := startY; y <= endY; y++ {
				if strings.HasPrefix(l, "turn on") {
					lights[y][x]++
				} else if strings.HasPrefix(l, "turn off") {
					if lights[y][x] > 0 {
						lights[y][x]--
					}
				} else if strings.HasPrefix(l, "toggle") {
					lights[y][x] += 2
				}
			}
		}
	}

	c := 0
	for _, ls := range lights {
		for _, l := range ls {
			c += l
		}
	}

	return c
}

var re = regexp.MustCompile(`.+ (?P<StartX>\d+),(?P<StartY>\d+) through (?P<EndX>\d+),(?P<EndY>\d+)`)

func parseCoordinates(line string) (int, int, int, int) {
	matches := re.FindStringSubmatch(line)

	startX, _ := strconv.Atoi(matches[re.SubexpIndex("StartX")])
	startY, _ := strconv.Atoi(matches[re.SubexpIndex("StartY")])
	endX, _ := strconv.Atoi(matches[re.SubexpIndex("EndX")])
	endY, _ := strconv.Atoi(matches[re.SubexpIndex("EndY")])

	return startX, startY, endX, endY
}
