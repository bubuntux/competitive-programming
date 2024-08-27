package main

import (
	"math"
	"testing"
)

func maxOperations(nums []int, k int) int {
	m := make(map[int]int)
	b := make(map[int]bool)
	for _, v := range nums {
		m[v] = m[v] + 1
		b[v] = true
	}

	count := 0
	for v := range m {
		if b[v] == false {
			continue
		}

		r := k - v
		if r == v {
			count = count + m[v]/2
		} else {
			count = count + int(math.Min(float64(m[v]), float64(m[r])))
		}

		b[v] = false
		b[r] = false
	}

	return count
}

// https://leetcode.com/problems/merge-strings-alternately/

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
