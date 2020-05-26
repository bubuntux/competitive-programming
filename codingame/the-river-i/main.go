package main

import (
	"fmt"
	"strconv"
)

func main() {
	var r1 int
	fmt.Scan(&r1)

	var r2 int
	fmt.Scan(&r2)

	for r1 != r2 {
		if r1 < r2 {
			r1 += nextValue(r1)
		} else {
			r2 += nextValue(r2)
		}
	}

	fmt.Println(r1)
}

func nextValue(currentValue int) int {
	nextValue := 0
	for _, c := range strconv.Itoa(currentValue) {
		nextValue += int(c - '0')
	}
	return nextValue
}
