package main

import (
	"testing"
)

func isSubsequence(s string, t string) bool {
	if len(s) == 0 {
		return true
	}
	if len(s) == len(t) {
		return s == t
	}

	rs := []rune(s)
	rt := []rune(t)

	for is, it := 0, 0; it < len(rt); it++ {
		if rt[it] == rs[is] {
			is++
			if is == len(rs) {
				return true
			}
		}
	}

	return false
}

// https://leetcode.com/problems/is-subsequence

func Test(t *testing.T) {
	tests := []struct {
		name string
		s    string
		t    string
		want bool
	}{
		{
			"example1",
			"abc",
			"ahbgdc",
			true,
		},
		{
			"example2",
			"axc",
			"ahbgdc",
			false,
		},
		{
			"example18",
			"",
			"ahbgdc",
			true,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := isSubsequence(tt.s, tt.t); got != tt.want {
				t.Errorf("%v = %v, want %v", tt.name, got, tt.want)
			}
		})
	}
}
