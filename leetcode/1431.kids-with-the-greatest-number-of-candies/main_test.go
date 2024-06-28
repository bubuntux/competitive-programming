package main

import (
	"reflect"
	"testing"
)

func kidsWithCandies(candies []int, extraCandies int) []bool {
	n := len(candies)
	greatest := candies[0]
	for i := 1; i < n; i++ {
		if candies[i] > greatest {
			greatest = candies[i]
		}
	}

	result := make([]bool, n)
	for i, e := range candies {
		result[i] = e+extraCandies >= greatest
	}

	return result
}

// https://leetcode.com/problems/kids-with-the-greatest-number-of-candies

func Test(t *testing.T) {
	tests := []struct {
		name         string
		candies      []int
		extraCandies int
		want         []bool
	}{
		{
			"example1",
			[]int{2, 3, 5, 1, 3},
			3,
			[]bool{true, true, true, false, true},
		},
		{
			"example2",
			[]int{4, 2, 1, 1, 2},
			1,
			[]bool{true, false, false, false, false},
		},
		{
			"example3",
			[]int{12, 1, 12},
			10,
			[]bool{true, false, true},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := kidsWithCandies(tt.candies, tt.extraCandies); !reflect.DeepEqual(got, tt.want) {
				t.Errorf("gcdOfStrings = %v, want %v", got, tt.want)
			}
		})
	}
}
