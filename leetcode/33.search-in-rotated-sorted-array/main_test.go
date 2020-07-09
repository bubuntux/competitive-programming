package main

import (
	"testing"
)

func search(nums []int, target int) int {
	left := 0
	right := len(nums) - 1

	for left <= right {
		index := ((right - left) / 2) + left
		value := nums[index]
		if value == target {
			return index
		}
		if value >= nums[left] {
			if target < value && target >= nums[left] {
				right = index - 1
			} else {
				left = index + 1
			}
		} else {
			if target > value && target <= nums[right] {
				left = index + 1
			} else {
				right = index - 1
			}
		}
	}

	return -1
}

func Test_search(t *testing.T) {
	type args struct {
		nums   []int
		target int
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{"example1",
			args{[]int{4, 5, 6, 7, 0, 1, 2}, 0},
			4,
		},
		{"example2",
			args{[]int{4, 5, 6, 7, 0, 1, 2}, 3},
			-1,
		},
		{"testCase2",
			args{[]int{}, 5},
			-1,
		},
		{"testCase8",
			args{[]int{1}, 1},
			0,
		},
		{"testCase9",
			args{[]int{1, 3}, 3},
			1,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := search(tt.args.nums, tt.args.target); got != tt.want {
				t.Errorf("search() = %v, want %v", got, tt.want)
			}
		})
	}
}
