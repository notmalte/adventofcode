package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	b, err := os.ReadFile("day16/input.txt")
	if err != nil {
		panic(err)
	}

	s := string(b)

	fmt.Printf("Part 1: %v\n", part1(s))
	fmt.Printf("Part 2: %v\n", part2(s))
}

var search = map[string]int{
	"children":    3,
	"cats":        7,
	"samoyeds":    2,
	"pomeranians": 3,
	"akitas":      0,
	"vizslas":     0,
	"goldfish":    5,
	"trees":       3,
	"cars":        2,
	"perfumes":    1,
}

func part1(s string) int {
	sues := parseSues(s)

	for _, s := range sues {
		fitsSearch := true

		for item, amount := range s.items {
			wantAmount := search[item]

			if wantAmount != amount {
				fitsSearch = false
				break
			}
		}

		if fitsSearch {
			return s.number
		}
	}

	panic("sue not found")
}

func part2(s string) int {
	sues := parseSues(s)

	for _, s := range sues {
		fitsSearch := true

		for item, amount := range s.items {
			wantAmount := search[item]

			if item == "cats" || item == "trees" {
				if wantAmount >= amount {
					fitsSearch = false
					break
				}
			} else if item == "pomeranians" || item == "goldfish" {
				if wantAmount <= amount {
					fitsSearch = false
					break
				}
			} else {
				if wantAmount != amount {
					fitsSearch = false
					break
				}
			}
		}

		if fitsSearch {
			return s.number
		}
	}

	panic("sue not found")
}

type sue struct {
	number int
	items  map[string]int
}

func parseSues(s string) []sue {
	lines := strings.Split(strings.TrimSpace(s), "\n")

	sues := []sue{}

	for _, line := range lines {
		line := line[4:]

		numStr, itemsStr, _ := strings.Cut(line, ": ")

		num, _ := strconv.Atoi(numStr)

		items := map[string]int{}

		for _, item := range strings.Split(itemsStr, ", ") {
			name, amountStr, _ := strings.Cut(item, ": ")

			amount, _ := strconv.Atoi(amountStr)

			items[name] = amount
		}

		sues = append(sues, sue{
			number: num,
			items:  items,
		})
	}

	return sues
}
