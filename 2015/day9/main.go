package main

import (
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

func main() {
	b, err := os.ReadFile("day9/input.txt")
	if err != nil {
		panic(err)
	}

	s := string(b)

	fmt.Printf("Part 1: %v\n", part1(s))
	fmt.Printf("Part 2: %v\n", part2(s))
}

func part1(s string) int {
	distances := parseDistances(s)

	locations := map[string]struct{}{}

	for start := range distances {
		locations[start] = struct{}{}

		for end := range distances[start] {
			locations[end] = struct{}{}
		}
	}

	length := findShortestRoute(distances, len(locations), nil)

	return length
}

func part2(s string) int {
	distances := parseDistances(s)

	locations := map[string]struct{}{}

	for start := range distances {
		locations[start] = struct{}{}

		for end := range distances[start] {
			locations[end] = struct{}{}
		}
	}

	length := findLongestRoute(distances, len(locations), nil)

	return length
}

type distances map[string]map[string]int

func parseDistances(s string) distances {
	lines := strings.Split(strings.TrimSpace(s), "\n")

	d := distances{}

	for _, l := range lines {
		route, distanceStr, _ := strings.Cut(l, " = ")

		distance, _ := strconv.Atoi(distanceStr)

		start, end, _ := strings.Cut(route, " to ")

		_, ok := d[start]
		if ok {
			d[start][end] = distance
		} else {
			d[start] = map[string]int{
				end: distance,
			}
		}

		_, ok = d[end]
		if ok {
			d[end][start] = distance
		} else {
			d[end] = map[string]int{
				start: distance,
			}
		}
	}

	return d
}

func findShortestRoute(d distances, n int, currentRoute []string) int {
	if currentRoute == nil {
		minLength := math.MaxInt

		for start := range d {
			route := []string{start}

			routeLength := findShortestRoute(d, n, route)

			if routeLength < minLength {
				minLength = routeLength
			}
		}

		return minLength
	}

	if len(currentRoute) == n {
		currentLength := 0
		for i := 0; i < len(currentRoute)-1; i++ {
			currentLength += d[currentRoute[i]][currentRoute[i+1]]
		}

		return currentLength
	}

	candidates := getNextCandidates(d, currentRoute)

	minLength := math.MaxInt

	for _, next := range candidates {
		newRoute := make([]string, len(currentRoute)+1)
		copy(newRoute, currentRoute)
		newRoute[len(currentRoute)] = next

		routeLength := findShortestRoute(d, n, newRoute)

		if routeLength < minLength {
			minLength = routeLength
		}
	}

	return minLength
}

func findLongestRoute(d distances, n int, currentRoute []string) int {
	if currentRoute == nil {
		maxLength := math.MinInt

		for start := range d {
			route := []string{start}

			routeLength := findLongestRoute(d, n, route)

			if routeLength > maxLength {
				maxLength = routeLength
			}
		}

		return maxLength
	}

	if len(currentRoute) == n {
		currentLength := 0
		for i := 0; i < len(currentRoute)-1; i++ {
			currentLength += d[currentRoute[i]][currentRoute[i+1]]
		}

		return currentLength
	}

	candidates := getNextCandidates(d, currentRoute)

	maxLength := math.MinInt

	for _, next := range candidates {
		newRoute := make([]string, len(currentRoute)+1)
		copy(newRoute, currentRoute)
		newRoute[len(currentRoute)] = next

		routeLength := findLongestRoute(d, n, newRoute)

		if routeLength > maxLength {
			maxLength = routeLength
		}
	}

	return maxLength
}

func getNextCandidates(d distances, currentRoute []string) []string {
	candidates := []string{}
	for next := range d[currentRoute[len(currentRoute)-1]] {
		alreadyInRoute := false
		for _, prev := range currentRoute {
			if next == prev {
				alreadyInRoute = true
				break
			}
		}
		if !alreadyInRoute {
			candidates = append(candidates, next)
		}
	}

	return candidates
}
