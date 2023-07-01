package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	file, err := os.Open("input")
	if err != nil {
		fmt.Println(err)
		return
	}
	defer file.Close()

	m := make(map[string]int)
	// A X Rock 1
	// B Y Paper 2
	// C Z Scissors 3
	m["A Y"] = 8
	m["A X"] = 4
	m["A Z"] = 3

	m["B Y"] = 5
	m["B X"] = 1
	m["B Z"] = 9

	m["C Y"] = 2
	m["C X"] = 7
	m["C Z"] = 6

	res := 0

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		value := scanner.Text()
		res += m[value]
	}

	fmt.Println("Result = ", res)
}
