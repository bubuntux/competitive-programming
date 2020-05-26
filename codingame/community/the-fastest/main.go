package main

import (
	"fmt"
	time2 "time"
)

func main() {
	var N int
	fmt.Scan(&N)

	var time time2.Time
	for i := 0; i < N; i++ {
		var t string
		fmt.Scan(&t)
		newTime, _ := time2.Parse("15:04:05", t)
		if time.After(newTime) {
			time = newTime
		}
	}

	fmt.Printf("%02d:%02d:%02d", time.Hour(), time.Minute(), time.Second())
}
