package main

import (
	"fmt"
	"os"
	"regexp"
	"sort"
	"strconv"
	"strings"
)

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
	nums := parseNums(s)

	possible := 0

	for _, n := range nums {
		sort.Ints(n)

		if n[0]+n[1] > n[2] {
			possible++
		}
	}

	return possible
}

func part2(s string) int {
	nums := parseNums(s)

	possible := 0

	for y := 0; y < len(nums); y += 3 {
		for x := 0; x < 3; x++ {
			n := []int{nums[y][x], nums[y+1][x], nums[y+2][x]}
			sort.Ints(n)

			if n[0]+n[1] > n[2] {
				possible++
			}
		}
	}

	return possible
}

func parseNums(s string) [][]int {
	lines := strings.Split(strings.TrimSpace(s), "\n")

	re := regexp.MustCompile(`^\s*(\d+)\s+(\d+)\s+(\d+)\s*$`)

	nums := [][]int{}

	for _, line := range lines {
		match := re.FindStringSubmatch(line)

		a, _ := strconv.Atoi(match[1])
		b, _ := strconv.Atoi(match[2])
		c, _ := strconv.Atoi(match[3])

		nums = append(nums, []int{a, b, c})
	}

	return nums
}
