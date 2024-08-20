package main

import (
	"reflect"
	"regexp"
	"slices"
	"strings"
	"testing"
)

func reverseWords(s string) string {
	re := regexp.MustCompile(`\S+`)

	split := re.FindAllString(s, -1)

	slices.Reverse(split)

	return strings.Join(split, " ")
}

// https://leetcode.com/problems/reverse-words-in-a-string

func Test(t *testing.T) {
	tests := []struct {
		name  string
		input string
		want  string
	}{
		{
			"example1",
			"the sky is blue",
			"blue is sky the",
		},
		{
			"example2",
			"  hello world  ",
			"world hello",
		},
		{
			"example3",
			"a good   example",
			"example good a",
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := reverseWords(tt.input); !reflect.DeepEqual(got, tt.want) {
				t.Errorf("reverseVowelsStrings = '%v', want '%v'", got, tt.want)
			}
		})
	}
}
