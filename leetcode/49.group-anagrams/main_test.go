package main

import (
	"reflect"
	"testing"
)

func groupAnagrams(in []string) [][]string {
	res := make([][]string, 0)
	groups := make(map[[26]uint8]uint)

	for _, x := range in {
		letters := [26]uint8{}
		for _, c := range x {
			letters[c-'a']++
		}
		if v, ok := groups[letters]; ok {
			res[v] = append(res[v], x)
		} else {
			res = append(res, []string{x})
			groups[letters] = uint(len(res) - 1)
		}
	}

	return res
}

// https://leetcode.com/problems/group-anagrams/

func Test_groupAnagrams(t *testing.T) {
	tests := []struct {
		name  string
		input []string
		want  [][]string
	}{
		{"example",
			[]string{"eat", "tea", "tan", "ate", "nat", "bat"},
			[][]string{
				{"eat", "tea", "ate"},
				{"tan", "nat"},
				{"bat"},
			}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := groupAnagrams(tt.input); !reflect.DeepEqual(got, tt.want) {
				t.Errorf("groupAnagrams() = %v, want %v", got, tt.want)
			}
		})
	}
}
