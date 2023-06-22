package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	b, err := os.ReadFile("day7/input.txt")
	if err != nil {
		panic(err)
	}

	s := string(b)

	fmt.Printf("Part 1: %v\n", part1(s))
	fmt.Printf("Part 2: %v\n", part2(s))
}

func part1(s string) int {
	lines := strings.Split(strings.TrimSpace(s), "\n")

	count := 0

	for _, line := range lines {
		outsideSequences, insideSequences := parseLine(line)

		if supportsTLS(outsideSequences, insideSequences) {
			count++
		}
	}

	return count
}

func part2(s string) int {
	lines := strings.Split(strings.TrimSpace(s), "\n")

	count := 0

	for _, line := range lines {
		outsideSequences, insideSequences := parseLine(line)

		if supportsSSL(outsideSequences, insideSequences) {
			count++
		}
	}

	return count
}

func parseLine(s string) ([]string, []string) {
	outsideSequences := []string{}
	insideSequences := []string{}

	current := ""
	isInside := false

	for _, c := range s {
		if c == '[' {
			isInside = true
			outsideSequences = append(outsideSequences, current)
			current = ""
		} else if c == ']' {
			isInside = false
			insideSequences = append(insideSequences, current)
			current = ""
		} else {
			current += string(c)
		}
	}

	if isInside {
		insideSequences = append(insideSequences, current)
	} else {
		outsideSequences = append(outsideSequences, current)
	}

	return outsideSequences, insideSequences
}

func supportsTLS(outsideSequences []string, insideSequences []string) bool {
	for _, inside := range insideSequences {
		if containsABBA(inside) {
			return false
		}
	}

	for _, outside := range outsideSequences {
		if containsABBA(outside) {
			return true
		}
	}

	return false
}

func containsABBA(s string) bool {
	for i := 0; i < len(s)-3; i++ {
		if s[i] == s[i+3] && s[i+1] == s[i+2] && s[i] != s[i+1] {
			return true
		}
	}

	return false
}

func supportsSSL(outsideSequences []string, insideSequences []string) bool {
	for _, outside := range outsideSequences {
		abas := findABAs(outside)

		for _, inside := range insideSequences {
			for _, aba := range abas {
				if containsBAB(inside, aba) {
					return true
				}
			}
		}
	}

	return false
}

func findABAs(s string) []string {
	abas := []string{}

	for i := 0; i < len(s)-2; i++ {
		if s[i] == s[i+2] && s[i] != s[i+1] {
			abas = append(abas, s[i:i+3])
		}
	}

	return abas
}

func containsBAB(s string, aba string) bool {
	for i := 0; i < len(s)-2; i++ {
		if s[i] == aba[1] && s[i+1] == aba[0] && s[i+2] == aba[1] {
			return true
		}
	}

	return false
}
