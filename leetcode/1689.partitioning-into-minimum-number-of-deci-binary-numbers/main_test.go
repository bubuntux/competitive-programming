package main

import (
	"testing"
)

func minPartitions(n string) int {
	max := 0
	for _, c := range n {
		d := int(c - '0')
		if d > max {
			max = d
		}
		if max == 9 {
			break
		}
	}
	return max
}

// https://leetcode.com/problems/partitioning-into-minimum-number-of-deci-binary-numbers/

func Test_minPartitions(t *testing.T) {
	type args struct {
		Input string
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{"example1", args{"32"}, 3},
		{"example2", args{"82734"}, 8},
		{"example3", args{"27346209830709182346"}, 9},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := minPartitions(tt.args.Input); got != tt.want {
				t.Errorf("%v = %v, want %v", tt.name, got, tt.want)
			}
		})
	}
}
