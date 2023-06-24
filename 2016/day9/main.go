package main

import (
	"fmt"
	"os"
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
	return decompressedLength(s, false)
}

func part2(s string) int {
	return decompressedLength(s, true)
}

func decompressedLength(s string, recursive bool) int {
	result := 0

	for i := 0; i < len(s); i++ {
		if s[i] == '(' {
			var length, repeat int
			fmt.Sscanf(s[i+1:], "%dx%d)", &length, &repeat)

			i += len(fmt.Sprintf("%dx%d)", length, repeat))

			if recursive {
				result += decompressedLength(s[i+1:i+1+length], true) * repeat
			} else {
				result += length * repeat
			}

			i += length
		} else {
			result++
		}
	}

	return result
}
