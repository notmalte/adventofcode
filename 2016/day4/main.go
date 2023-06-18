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
	b, err := os.ReadFile("day4/input.txt")
	if err != nil {
		panic(err)
	}

	s := string(b)

	fmt.Printf("Part 1: %v\n", part1(s))
	fmt.Printf("Part 2: %v\n", part2(s))
}

func part1(s string) int {
	rooms := parseRooms(s)

	sum := 0

	for _, room := range rooms {
		if validateChecksum(room.name, room.checksum) {
			sum += room.sectorID
		}
	}

	return sum
}

func part2(s string) int {
	rooms := parseRooms(s)

	for _, room := range rooms {
		if !validateChecksum(room.name, room.checksum) {
			continue
		}

		decrypted := []rune{}

		for _, r := range room.name {
			decrypted = append(decrypted, (r-'a'+rune(room.sectorID))%26+'a')
		}

		if strings.Contains(string(decrypted), "northpole") {
			return room.sectorID
		}
	}

	return -1
}

type room struct {
	name     string
	sectorID int
	checksum string
}

func parseRooms(s string) []room {
	re := regexp.MustCompile(`^([a-z-]+)-(\d+)\[([a-z]+)\]$`)

	rooms := []room{}

	lines := strings.Split(strings.TrimSpace(s), "\n")

	for _, line := range lines {
		matches := re.FindStringSubmatch(line)

		name := matches[1]
		sectorIDStr := matches[2]
		checksum := matches[3]

		name = strings.ReplaceAll(name, "-", "")
		sectorID, _ := strconv.Atoi(sectorIDStr)

		rooms = append(rooms, room{name, sectorID, checksum})
	}

	return rooms
}

func validateChecksum(name string, checksum string) bool {
	amounts := map[rune]int{}

	for _, r := range name {
		amounts[r]++
	}

	type pair struct {
		letter rune
		amount int
	}

	pairs := []pair{}

	for letter, amount := range amounts {
		pairs = append(pairs, pair{letter, amount})
	}

	sort.Slice(pairs, func(i, j int) bool {
		if pairs[i].amount == pairs[j].amount {
			return pairs[i].letter < pairs[j].letter
		}

		return pairs[i].amount > pairs[j].amount
	})

	for i, c := range checksum {
		if c != pairs[i].letter {
			return false
		}
	}

	return true
}
