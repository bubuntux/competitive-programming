package main

import (
	"testing"
)

type NumArray struct {
	nums []int
}

func Constructor(nums []int) NumArray {
	return NumArray{nums}
}

func (numArray *NumArray) SumRange(left int, right int) int {
	sum := 0
	for i := left; i <= right; i++ {
		sum += numArray.nums[i]
	}
	return sum
}

// https://leetcode.com/problems/range-sum-query-immutable/

func Test_sumRange(t *testing.T) {
	type args struct {
		left  int
		right int
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{"example1", args{0, 2}, 1},
		{"example2", args{2, 5}, -1},
		{"example3", args{0, 5}, -3},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			numArray := Constructor([]int{-2, 0, 3, -5, 2, -1})
			if got := numArray.SumRange(tt.args.left, tt.args.right); got != tt.want {
				t.Errorf("%v = %v, want %v", tt.name, got, tt.want)
			}
		})
	}
}
