package main

import (
	"reflect"
	"strings"
	"testing"
)

func reverseVowels(s string) string {
	queue := make([]byte, 0)

	sLower := strings.ToLower(s)
	for i, c := range sLower {
		if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
			queue = append(queue, s[i])
		}
	}

	sResult := []byte(s)
	q := len(queue) - 1
	for i, c := range sLower {
		if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
			sResult[i] = queue[q]
			q--
		}
	}

	return string(sResult)
}

// https://leetcode.com/problems/reverse-vowels-of-a-string

func Test(t *testing.T) {
	tests := []struct {
		name  string
		input string
		want  string
	}{
		{
			"example1",
			"hello",
			"holle",
		},
		{
			"example2",
			"leetcode",
			"leotcede",
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := reverseVowels(tt.input); !reflect.DeepEqual(got, tt.want) {
				t.Errorf("reverseVowelsStrings = %v, want %v", got, tt.want)
			}
		})
	}
}
