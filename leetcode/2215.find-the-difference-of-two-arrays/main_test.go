package main

import (
	"reflect"
	"testing"
)

func findDifference(nums1 []int, nums2 []int) [][]int {
	cache1 := make(map[int]bool)
	for _, v := range nums1 {
		cache1[v] = true
	}
	cache2 := make(map[int]bool)
	for _, v := range nums2 {
		cache2[v] = true
	}

	result := make([][]int, 2)
	result[0] = make([]int, 0)
	for _, v := range nums1 {
		if !cache2[v] {
			result[0] = append(result[0], v)
			cache2[v] = true
		}
	}
	result[1] = make([]int, 0)
	for _, v := range nums2 {
		if !cache1[v] {
			result[1] = append(result[1], v)
			cache1[v] = true
		}
	}

	return result
}

// https://leetcode.com/problems/find-the-difference-of-two-arrays/

func Test(t *testing.T) {
	tests := []struct {
		name  string
		nums1 []int
		nums2 []int
		want  [][]int
	}{
		{
			"example1",
			[]int{1, 2, 3},
			[]int{2, 4, 6},
			[][]int{{1, 3}, {4, 6}},
		},
		{
			"example2",
			[]int{1, 2, 3, 3},
			[]int{1, 1, 2, 2},
			[][]int{{3}, {}},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := findDifference(tt.nums1, tt.nums2); !reflect.DeepEqual(got, tt.want) {
				t.Errorf("got = %v, want = %v", got, tt.want)
			}
		})
	}
}
