package main

import (
	"testing"
)

func mergeAlternately(word1 string, word2 string) string {
	merge := ""
	for i := 0; i < len(word1) || i < len(word2); i++ {
		if i < len(word1) {
			merge += string(word1[i])
		}
		if i < len(word2) {
			merge += string(word2[i])
		}
	}

	return merge
}

// https://leetcode.com/problems/merge-strings-alternately/

func TestMergeAlternately(t *testing.T) {
	type args struct {
		word1 string
		word2 string
	}
	tests := []struct {
		name string
		args args
		want string
	}{
		{"example1", args{"abc", "pqr"}, "apbqcr"},
		{"example2", args{"ab", "pqrs"}, "apbqrs"},
		{"example3", args{"abcd", "pq"}, "apbqcd"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := mergeAlternately(tt.args.word1, tt.args.word2); got != tt.want {
				t.Errorf("%v = %v, want %v", tt.name, got, tt.want)
			}
		})
	}
}

func Test_mergeAlternately(t *testing.T) {
	type args struct {
		word1 string
		word2 string
	}
	tests := []struct {
		name string
		args args
		want string
	}{
		// TODO: Add test cases.
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := mergeAlternately(tt.args.word1, tt.args.word2); got != tt.want {
				t.Errorf("mergeAlternately() = %v, want %v", got, tt.want)
			}
		})
	}
}
