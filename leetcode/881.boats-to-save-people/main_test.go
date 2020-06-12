package main

import (
	"sort"
	"testing"
)

func numRescueBoats(people []int, limit int) int {
	sort.Ints(people)

	boats, left, right := 0, 0, len(people)-1
	for left <= right {
		boats++
		if people[left]+people[right] <= limit {
			left++
		}
		right--
	}

	return boats
}

// https://leetcode.com/problems/boats-to-save-people/

func Test_numRescueBoats(t *testing.T) {
	type args struct {
		people []int
		limit  int
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{"example_1", args{[]int{1, 2}, 3}, 1},
		{"example_2", args{[]int{3, 2, 2, 1}, 3}, 3},
		{"example_3", args{[]int{3, 5, 3, 4}, 5}, 4},

		{"test_case_43", args{[]int{1, 8, 4, 9, 7, 1, 5, 9, 3, 4}, 10}, 6},
		{"test_case_47", args{[]int{3, 2, 3, 2, 2}, 6}, 3},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := numRescueBoats(tt.args.people, tt.args.limit); got != tt.want {
				t.Errorf("%v numRescueBoats() = %v, want %v", tt.name, got, tt.want)
			}
		})
	}
}
