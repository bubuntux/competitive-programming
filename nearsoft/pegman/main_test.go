package main

import "testing"

func TestCase1(t *testing.T) {
	lines := []string{"^", "^"}
	result := processTestCase(lines)
	if result != "1" {
		t.Fail()
	}

}

func TestCase2(t *testing.T) {
	lines := []string{">v", "^<"}
	result := processTestCase(lines)
	if result != "0" {
		t.Fail()
	}
}

func TestCase3(t *testing.T) {
	lines := []string{"...", ".^.", "..."}
	result := processTestCase(lines)
	if result != "IMPOSSIBLE" {
		t.Fail()
	}
}

func TestCase4(t *testing.T) {
	lines := []string{"."}
	result := processTestCase(lines)
	if result != "0" {
		t.Fail()
	}
}