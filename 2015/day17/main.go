package main

import (
	"fmt"
	"math"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	b, err := os.ReadFile("day17/input.txt")
	if err != nil {
		panic(err)
	}

	s := string(b)

	fmt.Printf("Part 1: %v\n", part1(s))
	fmt.Printf("Part 2: %v\n", part2(s))
}

func part1(s string) int {
	containers := parseContainers(s)

	options := findContainerOptionsIndexes(containers, nil)

	return len(options)
}

func part2(s string) int {
	containers := parseContainers(s)

	options := findContainerOptionsIndexes(containers, nil)

	minAmount := math.MaxInt
	for _, option := range options {
		if len(option) < minAmount {
			minAmount = len(option)
		}
	}

	minOptions := 0
	for _, option := range options {
		if len(option) == minAmount {
			minOptions++
		}
	}

	return minOptions
}

var eggnog = 150

func parseContainers(s string) []int {
	lines := strings.Split(strings.TrimSpace(s), "\n")

	containers := []int{}

	for _, line := range lines {
		n, _ := strconv.Atoi(line)
		containers = append(containers, n)
	}

	sort.Ints(containers)

	return containers
}

func findContainerOptionsIndexes(containers []int, currentIndexes []int) [][]int {
	if currentIndexes == nil {
		options := [][]int{}

		for i, n := range containers {
			if n <= eggnog {
				result := findContainerOptionsIndexes(containers, []int{i})
				options = append(options, result...)
			}
		}

		return options
	}

	currentCapacity := 0
	for _, i := range currentIndexes {
		currentCapacity += containers[i]
	}

	if currentCapacity == eggnog {
		return [][]int{
			currentIndexes,
		}
	}

	remainingCapacity := eggnog - currentCapacity

	lastIndexUsed := currentIndexes[len(currentIndexes)-1]

	candidateIndexes := []int{}
	for i := lastIndexUsed + 1; i < len(containers); i++ {
		n := containers[i]

		if n <= remainingCapacity {
			candidateIndexes = append(candidateIndexes, i)
		}
	}

	if len(candidateIndexes) == 0 {
		return [][]int{}
	}

	options := [][]int{}

	for _, i := range candidateIndexes {
		newCurrent := make([]int, len(currentIndexes)+1)
		copy(newCurrent, currentIndexes)
		newCurrent[len(currentIndexes)] = i

		result := findContainerOptionsIndexes(containers, newCurrent)
		options = append(options, result...)
	}

	return options
}
