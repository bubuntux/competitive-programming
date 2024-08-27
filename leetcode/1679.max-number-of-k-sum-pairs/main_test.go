package main

import (
	"testing"
)

func maxOperations(nums []int, k int) int {
	dict := make(map[int]int)
	count := 0
	for _, num := range nums {
		target := k - num
		if v, ok := dict[target]; ok {
			count++
			if v > 1 {
				dict[target]--
			} else {
				delete(dict, target)
			}
		} else {
			dict[num]++
		}
	}
	return count
}

// https://leetcode.com/problems/max-number-of-k-sum-pairs/

func Test(t *testing.T) {
	tests := []struct {
		name string
		nums []int
		k    int
		want int
	}{
		{"example1", []int{1, 2, 3, 4}, 5, 2},
		{"example2", []int{3, 1, 3, 4, 3}, 6, 1},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := maxOperations(tt.nums, tt.k); got != tt.want {
				t.Errorf("%v = %v, want %v", tt.name, got, tt.want)
			}
		})
	}
}
