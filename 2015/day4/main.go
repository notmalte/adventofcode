package main

import (
	"crypto/md5"
	"encoding/hex"
	"fmt"
	"os"
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
	return findLeadingZerosHash(s, 5)
}

func part2(s string) int {
	return findLeadingZerosHash(s, 6)
}

func findLeadingZerosHash(s string, leading int) int {
	safeUpperLimit := 100_000_000

OUTER:
	for i := 1; i < safeUpperLimit; i++ {
		f := fmt.Sprintf("%s%d", s, i)

		hashBytes := md5.Sum([]byte(f))

		hash := hex.EncodeToString(hashBytes[:])

		for j := 0; j < leading; j++ {
			if hash[j] != '0' {
				continue OUTER
			}
		}

		return i
	}

	panic("did not find solution")
}
