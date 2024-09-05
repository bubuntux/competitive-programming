package main

import (
	"testing"
)

func pivotIndex(nums []int) int {
	right := 0
	for _, v := range nums {
		right = right + v
	}

	left := 0
	for i, v := range nums {
		right = right - v
		if left == right {
			return i
		}
		left = left + v
	}

	return -1
}

// https://leetcode.com/problems/find-pivot-index/

func Test(t *testing.T) {
	type args struct {
		Input string
	}
	tests := []struct {
		name string
		nums []int
		want int
	}{
		{"example1", []int{1, 7, 3, 6, 5, 6}, 3},
		{"example2", []int{1, 2, 3}, -1},
		{"example3", []int{2, 1, -1}, 0},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := pivotIndex(tt.nums); got != tt.want {
				t.Errorf("%v \n got = %v \n want = %v", tt.name, got, tt.want)
			}
		})
	}
}
