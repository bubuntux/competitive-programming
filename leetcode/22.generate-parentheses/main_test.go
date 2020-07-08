package main

import (
	"reflect"
	"strings"
	"testing"
)

func generateParenthesis(n int) []string {
	return solve("(", n-1, n)
}

func solve(parent string, open int, close int) []string {
	if open == 0 {
		return []string{parent + strings.Repeat(")", close)}
	}
	res := solve(parent+"(", open-1, close)
	if open < close {
		res = append(res, solve(parent+")", open, close-1)...)
	}
	return res
}

func Test_generateParenthesis(t *testing.T) {
	tests := []struct {
		name string
		n    int
		want []string
	}{
		{"example",
			3,
			[]string{
				"((()))",
				"(()())",
				"(())()",
				"()(())",
				"()()()",
			}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := generateParenthesis(tt.n); !reflect.DeepEqual(got, tt.want) {
				t.Errorf("generateParenthesis() = %v, want %v", got, tt.want)
			}
		})
	}
}
