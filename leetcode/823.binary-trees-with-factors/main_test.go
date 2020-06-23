package main

import (
	"sort"
	"testing"
)

func numFactoredBinaryTrees(in []int) int {
	if in == nil || len(in) == 0 {
		return 0
	}

	sort.Ints(in)
	values := make(map[int]int)

	res := 0
	for _, current := range in {
		res += calculate(values, current)
	}

	return res % 1_000_000_007
}

func calculate(values map[int]int, target int) int {
	value := 1
	for leftNode, leftValue := range values {
		if target%leftNode != 0 {
			continue
		}
		rightNode := target / leftNode
		if rightValue, exist := values[rightNode]; exist {
			value += leftValue * rightValue
		}
	}
	values[target] = value
	return value
}

// https://leetcode.com/problems/binary-trees-with-factors/

func Test_numFactoredBinaryTrees(t *testing.T) {
	type args struct {
		A []int
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{"example1", args{[]int{2, 4}}, 3},        //[2], [4], [4, 2, 2]
		{"example2", args{[]int{2, 4, 5, 10}}, 7}, // [2], [4], [5], [10], [4, 2, 2], [10, 2, 5], [10, 5, 2]
		{"case28", args{[]int{18, 3, 6, 2}}, 12},
		// [2], [3], [6], [18],
		// [6, 2, 3], [6, 3, 2],
		// [18, 6, 3], [18, 3, 6]
		// [18, null , 6, 3, 2], [18, null , 6, 2, 3],  [18, 6, null, 3, 2], [18, 6, null, 2, 3]
		{"case41", args{[]int{46, 144, 5040, 4488, 544, 380, 4410, 34, 11, 5, 3063808, 5550, 34496, 12, 540, 28, 18, 13, 2, 1056, 32710656, 31, 91872, 23, 26, 240, 18720, 33, 49, 4, 38, 37, 1457, 3, 799, 557568, 32, 1400, 47, 10, 20774, 1296, 9, 21, 92928, 8704, 29, 2162, 22, 1883700, 49588, 1078, 36, 44, 352, 546, 19, 523370496, 476, 24, 6000, 42, 30, 8, 16262400, 61600, 41, 24150, 1968, 7056, 7, 35, 16, 87, 20, 2730, 11616, 10912, 690, 150, 25, 6, 14, 1689120, 43, 3128, 27, 197472, 45, 15, 585, 21645, 39, 40, 2205, 17, 48, 136}}, 509730797},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := numFactoredBinaryTrees(tt.args.A); got != tt.want {
				t.Errorf("%v = %v, want %v", tt.name, got, tt.want)
			}
		})
	}
}
