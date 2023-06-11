package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	b, err := os.ReadFile("day14/input.txt")
	if err != nil {
		panic(err)
	}

	s := string(b)

	fmt.Printf("Part 1: %v\n", part1(s))
	fmt.Printf("Part 2: %v\n", part2(s))
}

func part1(s string) int {
	reindeers := parseReindeers(s)

	for i := 0; i < 2503; i++ {
		for j := range reindeers {
			reindeers[j].tick()
		}
	}

	maxDistance := 0
	for _, r := range reindeers {
		if r.distance > maxDistance {
			maxDistance = r.distance
		}
	}

	return maxDistance
}

func part2(s string) int {
	reindeers := parseReindeers(s)

	for i := 0; i < 2503; i++ {
		for j := range reindeers {
			reindeers[j].tick()
		}

		maxDistance := 0
		for _, r := range reindeers {
			if r.distance > maxDistance {
				maxDistance = r.distance
			}
		}

		for j := range reindeers {
			if reindeers[j].distance == maxDistance {
				reindeers[j].score++
			}
		}
	}

	maxScore := 0
	for _, r := range reindeers {
		if r.score > maxScore {
			maxScore = r.score
		}
	}

	return maxScore
}

type reindeer struct {
	name         string
	speed        int
	flyTime      int
	restTime     int
	distance     int
	isResting    bool
	restTimeDone int
	flyTimeDone  int
	score        int
}

func (r *reindeer) tick() {
	if r.isResting {
		r.restTimeDone++
		if r.restTimeDone == r.restTime {
			r.isResting = false
			r.restTimeDone = 0
		}
	} else {
		r.distance += r.speed
		r.flyTimeDone++
		if r.flyTimeDone == r.flyTime {
			r.isResting = true
			r.flyTimeDone = 0
		}
	}
}

func parseReindeers(s string) []reindeer {
	lines := strings.Split(strings.TrimSpace(s), "\n")

	reindeers := []reindeer{}

	for _, line := range lines {
		var name string
		var speed, flyTime, restTime int

		fmt.Sscanf(line, "%s can fly %d km/s for %d seconds, but then must rest for %d seconds.", &name, &speed, &flyTime, &restTime)

		reindeers = append(reindeers, reindeer{
			name:     name,
			speed:    speed,
			flyTime:  flyTime,
			restTime: restTime,
		})
	}

	return reindeers
}
