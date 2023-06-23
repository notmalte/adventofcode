package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
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
	instructions := parseInstructions(s)

	screen := make([][]bool, HEIGHT)
	for i := range screen {
		screen[i] = make([]bool, WIDTH)
	}

	for _, instruction := range instructions {
		instruction.apply(screen)
	}

	count := 0

	for _, row := range screen {
		for _, cell := range row {
			if cell {
				count++
			}
		}
	}

	return count
}

func part2(s string) string {
	instructions := parseInstructions(s)

	screen := make([][]bool, HEIGHT)
	for i := range screen {
		screen[i] = make([]bool, WIDTH)
	}

	for _, instruction := range instructions {
		instruction.apply(screen)
	}

	lines := []string{}

	for _, row := range screen {
		str := ""

		for _, cell := range row {
			if cell {
				str += "#"
			} else {
				str += " "
			}
		}

		lines = append(lines, str)
	}

	return "\n" + strings.Join(lines, "\n")
}

const (
	WIDTH  = 50
	HEIGHT = 6
)

type rectInstruction struct {
	width  int
	height int
}

func (i rectInstruction) apply(screen [][]bool) {
	for x := 0; x < i.width; x++ {
		for y := 0; y < i.height; y++ {
			screen[y][x] = true
		}
	}
}

type rotateRowInstruction struct {
	row    int
	amount int
}

func (i rotateRowInstruction) apply(screen [][]bool) {
	row := screen[i.row]

	length := len(row)
	amount := i.amount % length

	newRow := make([]bool, length)
	for j := range row {
		newRow[(j+amount)%length] = row[j]
	}

	screen[i.row] = newRow
}

type rotateColumnInstruction struct {
	column int
	amount int
}

func (i rotateColumnInstruction) apply(screen [][]bool) {
	column := make([]bool, len(screen))
	for j := range column {
		column[j] = screen[j][i.column]
	}

	length := len(column)
	amount := i.amount % length

	newColumn := make([]bool, length)
	for j := range column {
		newColumn[(j+amount)%length] = column[j]
	}

	for j := range column {
		screen[j][i.column] = newColumn[j]
	}
}

type instruction interface {
	apply(screen [][]bool)
}

func parseInstructions(s string) []instruction {
	lines := strings.Split(strings.TrimSpace(s), "\n")

	instructions := []instruction{}

	rectRegex := regexp.MustCompile(`rect (\d+)x(\d+)`)
	rotateRowRegex := regexp.MustCompile(`rotate row y=(\d+) by (\d+)`)
	rotateColumnRegex := regexp.MustCompile(`rotate column x=(\d+) by (\d+)`)

	for _, line := range lines {
		if matches := rectRegex.FindStringSubmatch(line); matches != nil {
			width, _ := strconv.Atoi(matches[1])
			height, _ := strconv.Atoi(matches[2])

			instructions = append(instructions, rectInstruction{width, height})
		} else if matches := rotateRowRegex.FindStringSubmatch(line); matches != nil {
			row, _ := strconv.Atoi(matches[1])
			amount, _ := strconv.Atoi(matches[2])

			instructions = append(instructions, rotateRowInstruction{row, amount})
		} else if matches := rotateColumnRegex.FindStringSubmatch(line); matches != nil {
			column, _ := strconv.Atoi(matches[1])
			amount, _ := strconv.Atoi(matches[2])

			instructions = append(instructions, rotateColumnInstruction{column, amount})
		} else {
			panic("invalid instruction")
		}
	}

	return instructions
}
