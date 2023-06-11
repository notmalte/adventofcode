package main

import (
	"encoding/json"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

func main() {
	b, err := os.ReadFile("day12/input.txt")
	if err != nil {
		panic(err)
	}

	s := string(b)

	fmt.Printf("Part 1: %v\n", part1(s))
	fmt.Printf("Part 2: %v\n", part2(s))
}

func part1(s string) int {
	re := regexp.MustCompile(`-?\d+`)

	matches := re.FindAllString(s, -1)

	sum := 0
	for _, match := range matches {
		n, _ := strconv.Atoi(match)

		sum += n
	}

	return sum
}

func part2(s string) int {
	var j any

	err := json.Unmarshal([]byte(s), &j)
	if err != nil {
		panic(err)
	}

	return getNonRedSum(j)
}

func getNonRedSum(v any) int {
	switch vv := v.(type) {
	case []any:
		sum := 0
		for _, vvv := range vv {
			sum += getNonRedSum(vvv)
		}
		return sum
	case map[string]any:
		for _, vvv := range vv {
			if vvv == "red" {
				return 0
			}
		}

		sum := 0
		for _, vvv := range vv {
			sum += getNonRedSum(vvv)
		}
		return sum
	case string:
		return 0
	case float64:
		return int(vv)
	}

	panic(fmt.Sprintf("unknown type %T", v))
}
