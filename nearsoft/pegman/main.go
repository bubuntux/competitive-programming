package main

import (
	"fmt"
	"strconv"
)

const (
	EMPTY = '.'
	UP    = '^'
	RIGHT = '>'
	DOWN  = 'v'
	LEFT  = '<'
)

func processTestCase(lines []string) string {
	counter := 0
	R := len(lines)
	for r := 0; r < R; r++ {
		for c, d := range lines[r] {
			if d == EMPTY {
				continue
			}
			if valid(lines, r, c, d) {
				continue
			}
			directions := make(map[rune]bool)
			directions[UP] = true
			directions[RIGHT] = true
			directions[DOWN] = true
			directions[LEFT] = true
			delete(directions, d)
			validFound := false
			for nd := range directions {
				if valid(lines, r, c, nd) {
					validFound = true
					break
				}
			}
			if validFound {
				counter++
			} else {
				return "IMPOSSIBLE"
			}
		}
	}

	return strconv.Itoa(counter)
}

func valid(lines []string, r int, c int, d rune) bool {
	switch d {
	case UP:
		r--
		if r < 0 {
			return false
		}
		break
	case RIGHT:
		c++
		if c >= len(lines[r]) {
			return false
		}
		break
	case DOWN:
		r++
		if r >= len(lines) {
			return false
		}
		break
	case LEFT:
		c--
		if c < 0 {
			return false
		}
		break
	}

	if lines[r][c] == EMPTY {
		return valid(lines, r, c, d)
	}

	return true
}

func main() {
	var T int
	fmt.Scan(&T)

	for t := 1; t <= T; t++ {
		var R, C int
		fmt.Scan(&R, &C)

		var lines = make([]string, R)
		for r := 0; r < R; r++ {
			fmt.Scan(&lines[r])
		}

		var result = processTestCase(lines)
		fmt.Println(fmt.Sprintf("Case #%d: %s", t, result))
	}
}
