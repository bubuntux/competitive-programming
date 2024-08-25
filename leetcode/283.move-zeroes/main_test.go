package main

import (
	"reflect"
	"testing"
)

func moveZeroes(nums []int) {
	zeroCount := 0
	n := len(nums)
	for i := 0; i < n; i++ {
		if nums[i] == 0 {
			zeroCount++
			continue
		}
		nums[i-zeroCount] = nums[i]
	}
	if zeroCount > 0 {
		for i := n - zeroCount; i < n; i++ {
			nums[i] = 0
		}
	}
}

// https://leetcode.com/problems/move-zeroes

func Test(t *testing.T) {
	tests := []struct {
		name  string
		input []int
		want  []int
	}{
		{
			"example1",
			[]int{0, 1, 0, 3, 12},
			[]int{1, 3, 12, 0, 0},
		},
		{
			"example2",
			[]int{0},
			[]int{0},
		},
		{
			"example32",
			[]int{0, 1, 0},
			[]int{1, 0, 0},
		},
		{
			"example41",
			[]int{1},
			[]int{1},
		},
		{
			"example44",
			[]int{-13009, 0, -81471, 93346, 0, -71602, -18829, -45703, 0, 0, 0, 15246, 0, 51324, 89825, 0, 70362, 0, 50913, 0, 47988, -87456, 94441, 0, 0, 77733, 9338},
			[]int{-13009, -81471, 93346, -71602, -18829, -45703, 15246, 51324, 89825, 70362, 50913, 47988, -87456, 94441, 77733, 9338, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			moveZeroes(tt.input)
			if !reflect.DeepEqual(tt.input, tt.want) {
				t.Errorf("reverseVowelsStrings (%v) got = %v want = %v", tt.name, tt.input, tt.want)
			}
		})
	}
}
