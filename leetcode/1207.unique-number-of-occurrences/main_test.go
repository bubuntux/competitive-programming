package main

import (
	"testing"
)

func uniqueOccurrences(arr []int) bool {
	occurrences := make(map[int]int)
	for _, v := range arr {
		occurrences[v]++
	}

	exist := make(map[int]bool)
	for _, v := range occurrences {
		if exist[v] {
			return false
		}
		exist[v] = true
	}

	return true
}

// https://leetcode.com/problems/unique-number-of-occurrences/

func Test(t *testing.T) {
	tests := []struct {
		name string
		arr  []int
		want bool
	}{
		{"example1", []int{1, 2, 2, 1, 1, 3}, true},
		{"example2", []int{1, 2}, false},
		{"example3", []int{-3, 0, 1, -3, 1, 1, 1, -3, 10, 0}, true},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := uniqueOccurrences(tt.arr); got != tt.want {
				t.Errorf("%v = %v, want %v", tt.name, got, tt.want)
			}
		})
	}
}
