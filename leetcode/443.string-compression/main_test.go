package main

import (
	"reflect"
	"strconv"
	"testing"
)

func compress(chars []byte) int {
	n := len(chars)
	if n <= 1 {
		return n
	}

	out := make([]byte, 0)

	counter := 1
	current := chars[0]

	for i := 1; i <= n; i++ {
		if chars[i] == current {
			counter++
		} else {
			out = append(out, current)
			if counter > 1 {
				count := strconv.Itoa(counter)
				for _, c := range count {
					out = append(out, byte(c))
				}
			}
			counter = 1
			current = chars[i]
		}
	}

	out = append(out, current)
	if counter > 1 {
		count := strconv.Itoa(counter)
		for _, c := range count {
			out = append(out, byte(c))
		}
	}

	copy(chars, out)
	return len(out)
}

// https://leetcode.com/problems/string-compression/

func Test(t *testing.T) {
	tests := []struct {
		name  string
		input []byte
		want  int
	}{
		{
			"example1",
			[]byte{'a', 'a', 'b', 'b', 'c', 'c', 'c'},
			6,
		},
		{
			"example2",
			[]byte{'a'},
			1,
		},
		{
			"example3",
			[]byte{'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b'},
			4,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := compress(tt.input); !reflect.DeepEqual(got, tt.want) {
				t.Errorf("reverseVowelsStrings = %v, want %v", got, tt.want)
			}
		})
	}
}
