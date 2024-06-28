package main

import (
	"reflect"
	"testing"
)

func gcdOfStrings(str1 string, str2 string) string {
	if str1+str2 != str2+str1 {
		return ""
	}

	return str1[:gcd(len(str1), len(str2))]
}

func gcd(a, b int) int {
	for b != 0 {
		a, b = b, a%b
	}

	return a
}

// https://leetcode.com/problems/greatest-common-divisor-of-strings/

func Test_generateParenthesis(t *testing.T) {
	tests := []struct {
		name string
		str1 string
		str2 string
		want string
	}{
		{
			"example122",
			"AA",
			"A",
			"A",
		},
		{
			"example1",
			"ABCABC",
			"ABC",
			"ABC",
		},
		{
			"example2",
			"ABABAB",
			"ABAB",
			"AB",
		},
		{
			"example3",
			"LEET",
			"CODE",
			"",
		},
		{
			"example4",
			"TAUXXTAUXXTAUXXTAUXXTAUXX",
			"TAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXX",
			"TAUXX",
		},
		{
			"example5",
			"UETKZUETKZUETKZUETKZUETKZUETKZUETKZUETKZUETKZUETKZUETKZ",
			"UETKZUETKZUETKZUETKZUETKZUETKZUETKZUETKZUETKZUETKZUETKZUETKZUETKZ",
			"UETKZ",
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := gcdOfStrings(tt.str1, tt.str2); !reflect.DeepEqual(got, tt.want) {
				t.Errorf("gcdOfStrings = %v, want %v", got, tt.want)
			}
		})
	}
}

func Test_gcdOfStrings(t *testing.T) {
	type args struct {
		str1 string
		str2 string
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
			if got := gcdOfStrings(tt.args.str1, tt.args.str2); got != tt.want {
				t.Errorf("gcdOfStrings() = %v, want %v", got, tt.want)
			}
		})
	}
}
