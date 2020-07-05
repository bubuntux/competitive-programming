package main

import (
	"testing"
)

func addBinary(a string, b string) string {
	res := make([]rune, max(len(a), len(b)))
	carry := false

	for ri, ai, bi, av, bv := len(res)-1, len(a)-1, len(b)-1, '0', '0'; ri >= 0; ri-- {
		av, ai = valueAt(a, ai)
		bv, bi = valueAt(b, bi)
		res[ri] = '0'
		if av == bv {
			if carry {
				res[ri] = '1'
			}
			carry = av == '1'
		} else if !carry {
			res[ri] = '1'
		}
	}
	if carry {
		return "1" + string(res)
	}
	return string(res)
}

func max(a int, b int) int {
	if a > b {
		return a
	}
	return b
}

func valueAt(input string, index int) (rune, int) {
	value := '0'
	if index >= 0 {
		value = rune(input[index])
		index--
	}
	return value, index
}

// https://leetcode.com/problems/add-binary/

func Test_addBinary(t *testing.T) {
	tests := []struct {
		name string
		a    string
		b    string
		want string
	}{
		{"example1", "11", "1", "100"},
		{"example2", "1010", "1011", "10101"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := addBinary(tt.a, tt.b); got != tt.want {
				t.Errorf("%v got = %v, want %v", tt.name, got, tt.want)
			}
		})
	}
}
