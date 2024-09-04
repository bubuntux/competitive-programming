package main

import (
	"testing"
)

func largestAltitude(gain []int) int {
	highestPoint := 0
	altitude := 0
	for _, v := range gain {
		altitude = altitude + v
		if highestPoint < altitude {
			highestPoint = altitude
		}
	}
	return highestPoint
}

// https://leetcode.com/problems/find-the-highest-altitude/

func Test(t *testing.T) {
	type args struct {
		Input string
	}
	tests := []struct {
		name string
		gain []int
		want int
	}{
		{"example1", []int{-5, 1, 5, 0, -7}, 1},
		{"example2", []int{-4, -3, -2, -1, 4, 3, 2}, 0},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := largestAltitude(tt.gain); got != tt.want {
				t.Errorf("%v \n got = %v \n want = %v", tt.name, got, tt.want)
			}
		})
	}
}
