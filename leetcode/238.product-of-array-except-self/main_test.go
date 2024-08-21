package main

import (
	"reflect"
	"testing"
)

func productExceptSelf(nums []int) []int {
	size := len(nums)
	result := make([]int, size)

	aux := 1
	for i := 0; i < size; i++ {
		result[i] = aux
		aux = aux * nums[i]
	}

	aux = 1
	for i := size - 1; i >= 0; i-- {
		result[i] = result[i] * aux
		aux = aux * nums[i]
	}

	return result
}

// https://leetcode.com/problems/product-of-array-except-self

func Test(t *testing.T) {
	tests := []struct {
		name string
		in   []int
		want []int
	}{
		{
			"example1",
			[]int{1, 2, 3, 4},
			[]int{24, 12, 8, 6},
		},
		{
			"example2",
			[]int{-1, 1, 0, -3, 3},
			[]int{0, 0, 9, 0, 0},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := productExceptSelf(tt.in); !reflect.DeepEqual(got, tt.want) {
				t.Errorf("generateParenthesis() = %v, want %v", got, tt.want)
			}
		})
	}
}
