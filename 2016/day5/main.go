package main

import (
	"crypto/md5"
	"encoding/hex"
	"fmt"
	"os"
	"strconv"
)

func main() {
	b, err := os.ReadFile("day5/input.txt")
	if err != nil {
		panic(err)
	}

	s := string(b)

	fmt.Printf("Part 1: %v\n", part1(s))
	fmt.Printf("Part 2: %v\n", part2(s))
}

func part1(s string) string {
	result := []rune{}

	for j := 0; j < 100_000_000; j++ {
		hash := doorIntegerHash(s, j)

		if isValid(hash) {
			result = append(result, rune(hash[5]))

			if len(result) == 8 {
				break
			}
		}
	}

	return string(result)
}

func part2(s string) string {
	result := []rune{'_', '_', '_', '_', '_', '_', '_', '_'}

	for j := 0; j < 100_000_000; j++ {
		hash := doorIntegerHash(s, j)

		if isValid(hash) {
			pos, err := strconv.Atoi(string(hash[5]))
			if err == nil && pos < 8 && result[pos] == '_' {
				result[pos] = rune(hash[6])

				if isComplete(result) {
					break
				}
			}
		}
	}

	return string(result)
}

func doorIntegerHash(s string, i int) []rune {
	in := s + strconv.Itoa(i)
	hashBytes := md5.Sum([]byte(in))
	return []rune(hex.EncodeToString(hashBytes[:]))
}

func isComplete(result []rune) bool {
	for _, r := range result {
		if r == '_' {
			return false
		}
	}

	return true
}

func isValid(hash []rune) bool {
	for k := 0; k < 5; k++ {
		if hash[k] != '0' {
			return false
		}
	}

	return true
}
