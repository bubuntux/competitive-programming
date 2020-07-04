package main

import (
	"reflect"
	"testing"
)

func merge(ma []int, m int, na []int, n int) {
	m--
	n--
	for i := len(ma) - 1; i >= 0; i-- {
		if n < 0 {
			break
		}

		if m >= 0 && ma[m] > na[n] {
			ma[i] = ma[m]
			m--
		} else {
			ma[i] = na[n]
			n--
		}
	}
}

// https://leetcode.com/problems/merge-sorted-array/

func Test_merge(t *testing.T) {
	type args struct {
		nums1 []int
		m     int
		nums2 []int
		n     int
	}
	tests := []struct {
		name string
		args args
		want []int
	}{
		{"example",
			args{[]int{1, 2, 3, 0, 0, 0}, 3, []int{2, 5, 6}, 3},
			[]int{1, 2, 2, 3, 5, 6},
		},
		{"example2",
			args{[]int{1, 3, 5, 0, 0, 0}, 3, []int{2, 4, 6}, 3},
			[]int{1, 2, 3, 4, 5, 6},
		},
		{"case1",
			args{[]int{1}, 1, []int{}, 0},
			[]int{1},
		},
		{"case29",
			args{[]int{4, 5, 6, 0, 0, 0}, 3, []int{1, 2, 3}, 3},
			[]int{1, 2, 3, 4, 5, 6},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			merge(tt.args.nums1, tt.args.m, tt.args.nums2, tt.args.n)
			if !reflect.DeepEqual(tt.want, tt.args.nums1) {
				t.Errorf("%v got = %v , want = %v", tt.name, tt.args.nums1, tt.want)
			}
		})
	}
}
