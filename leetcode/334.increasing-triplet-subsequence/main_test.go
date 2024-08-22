package main

import (
	"math"
	"reflect"
	"testing"
)

func increasingTriplet(nums []int) bool {
	if len(nums) < 3 {
		return false
	}

	p1 := math.MaxInt
	p2 := math.MaxInt

	for _, v := range nums {
		if v <= p1 {
			p1 = v
			continue
		}
		if v <= p2 {
			p2 = v
			continue
		}
		return true
	}

	return false
}

// https://leetcode.com/problems/increasing-triplet-subsequence

func Test(t *testing.T) {
	tests := []struct {
		name  string
		input []int
		want  bool
	}{
		{
			"example1",
			[]int{1, 2, 3, 4, 5},
			true,
		},
		{
			"example2",
			[]int{5, 4, 3, 2, 1},
			false,
		},
		{
			"example3",
			[]int{2, 1, 5, 0, 4, 6},
			true,
		},
		{
			"example28",
			[]int{20, 100, 10, 12, 5, 13},
			true,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := increasingTriplet(tt.input); !reflect.DeepEqual(got, tt.want) {
				t.Errorf("reverseVowelsStrings = %v, want %v", got, tt.want)
			}
		})
	}
}
