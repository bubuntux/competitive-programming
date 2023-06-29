package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	file, err := os.Open("input")
	if err != nil {
		fmt.Println(err)
		return
	}
	defer file.Close()

	var max int64 = 0
	var cur int64 = 0

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		value := scanner.Text()
		if value == "" {
			if cur > max {
				max = cur
			}
			cur = 0
		} else {
			vi, _ := strconv.ParseInt(value, 10, 64)
			cur += vi
		}
	}

	fmt.Println("Result = ", max)
}
