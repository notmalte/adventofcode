package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	b, err := os.ReadFile("day19/input.txt")
	if err != nil {
		panic(err)
	}

	s := string(b)

	fmt.Printf("Part 1: %v\n", part1(s))
	fmt.Printf("Part 2: %v\n", part2(s))
}

func part1(s string) int {
	in := parseInput(s)

	m := generateMolecules(in)

	d := distinctMolecules(m)

	return len(d)
}

func part2(s string) int {
	panic("todo")
}

type replacement struct {
	from string
	to   string
}

type input struct {
	replacements []replacement
	molecule     string
}

func parseInput(s string) input {
	lines := strings.Split(strings.TrimSpace(s), "\n")

	molecule := lines[len(lines)-1]

	replacementLines := lines[:len(lines)-2]

	replacements := []replacement{}

	for _, l := range replacementLines {
		from, to, _ := strings.Cut(l, " => ")

		replacements = append(replacements, replacement{
			from: from,
			to:   to,
		})
	}

	return input{
		replacements: replacements,
		molecule:     molecule,
	}
}

func generateMolecules(in input) []string {
	molecules := []string{}

	originalRunes := []rune(in.molecule)

	for _, replace := range in.replacements {
		fromRunes := []rune(replace.from)
		toRunes := []rune(replace.to)

		for i := 0; i <= len(originalRunes)-len(fromRunes); i++ {
			isMatch := true
			for j := 0; j < len(fromRunes); j++ {
				if fromRunes[j] != originalRunes[i+j] {
					isMatch = false
					break
				}
			}

			if !isMatch {
				continue
			}

			replaced := []rune{}
			replaced = append(replaced, originalRunes[:i]...)
			replaced = append(replaced, toRunes...)
			replaced = append(replaced, originalRunes[i+len(fromRunes):]...)

			molecules = append(molecules, string(replaced))
		}
	}

	return molecules
}

func distinctMolecules(m []string) []string {
	dm := map[string]struct{}{}

	for _, v := range m {
		dm[v] = struct{}{}
	}

	d := []string{}

	for k := range dm {
		d = append(d, k)
	}

	return d
}
