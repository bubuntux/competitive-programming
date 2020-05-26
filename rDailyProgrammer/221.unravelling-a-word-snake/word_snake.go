package main

import (
	"fmt"
	"strconv"
	"strings"
)

func printSnake(words string) {
	tab := 1
	for wordIndex, word := range strings.Split(words, " ") {
		wordLength := len(word)
		if wordIndex % 2 == 0 {
			fmt.Println(string(word[0]))
			for charIndex := 1; charIndex < wordLength - 1; charIndex++ {
				fmt.Printf("%" + strconv.Itoa(tab) + "s\n", string(word[charIndex]))
			}
		} else {
			tab += wordLength
			fmt.Printf("%" + strconv.Itoa(tab - 1) + "s", word)
		}
	}
}

func main() {
	printSnake("SHENANIGANS SALTY YOUNGSTER ROUND DOUBLET TERABYTE ESSENCE")
}
