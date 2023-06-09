package main

import (
	"fmt"
	"os"
	"strconv"
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
	inputs := buildInputs(s)

	a := inputs["a"]

	c := cache{}

	v := a.eval(inputs, c)

	return v
}

func part2(s string) int {
	inputs := buildInputs(s)

	c := cache{}
	a := inputs["a"]
	v := a.eval(inputs, c)

	inputs["b"] = &pureInput{
		value: v,
	}

	c = cache{}
	v = a.eval(inputs, c)

	return v
}

type cache map[string]int

func (c *cache) getOrSet(key string, closure func() int) int {
	v, ok := (*c)[key]
	if ok {
		return v
	}

	v = closure()

	(*c)[key] = v

	return v
}

type input interface {
	eval(wireInputMap, cache) int
}

type wireInputMap map[string]input

type pureInput struct {
	value int
}

func (i *pureInput) eval(w wireInputMap, c cache) int {
	return i.value
}

type wireInput struct {
	source string
}

func (i *wireInput) eval(w wireInputMap, c cache) int {
	s := w[i.source]

	return c.getOrSet(i.source, func() int {
		return s.eval(w, c)
	})
}

type andInput struct {
	left  input
	right input
}

func (i *andInput) eval(w wireInputMap, c cache) int {
	return i.left.eval(w, c) & i.right.eval(w, c)
}

type orInput struct {
	left  input
	right input
}

func (i *orInput) eval(w wireInputMap, c cache) int {
	return i.left.eval(w, c) | i.right.eval(w, c)
}

type notInput struct {
	source input
}

func (i *notInput) eval(w wireInputMap, c cache) int {
	return 0xffff ^ i.source.eval(w, c)
}

type lshiftInput struct {
	source input
	by     int
}

func (i *lshiftInput) eval(w wireInputMap, c cache) int {
	return i.source.eval(w, c) << i.by
}

type rshiftInput struct {
	source input
	by     int
}

func (i *rshiftInput) eval(w wireInputMap, c cache) int {
	return i.source.eval(w, c) >> i.by
}

func getPureOrWireInput(v string) input {
	vInt, err := strconv.Atoi(v)

	if err != nil {
		return &wireInput{
			source: v,
		}
	} else {
		return &pureInput{
			value: vInt,
		}
	}
}

func buildInputs(s string) wireInputMap {
	lines := strings.Split(strings.TrimSpace(s), "\n")

	inputs := wireInputMap{}

	for _, l := range lines {
		src, dst, _ := strings.Cut(l, " -> ")

		if strings.Contains(src, " AND ") {
			left, right, _ := strings.Cut(src, " AND ")

			inputs[dst] = &andInput{
				left:  getPureOrWireInput(left),
				right: getPureOrWireInput(right),
			}
		} else if strings.Contains(src, " OR ") {
			left, right, _ := strings.Cut(src, " OR ")

			inputs[dst] = &orInput{
				left:  getPureOrWireInput(left),
				right: getPureOrWireInput(right),
			}
		} else if strings.HasPrefix(src, "NOT ") {
			identifier := strings.TrimPrefix(src, "NOT ")

			inputs[dst] = &notInput{
				source: getPureOrWireInput(identifier),
			}
		} else if strings.Contains(src, " LSHIFT ") {
			left, right, _ := strings.Cut(src, " LSHIFT ")

			by, _ := strconv.Atoi(right)

			inputs[dst] = &lshiftInput{
				source: getPureOrWireInput(left),
				by:     by,
			}
		} else if strings.Contains(src, " RSHIFT ") {
			left, right, _ := strings.Cut(src, " RSHIFT ")

			by, _ := strconv.Atoi(right)

			inputs[dst] = &rshiftInput{
				source: getPureOrWireInput(left),
				by:     by,
			}
		} else {
			inputs[dst] = getPureOrWireInput(src)
		}
	}

	return inputs
}
