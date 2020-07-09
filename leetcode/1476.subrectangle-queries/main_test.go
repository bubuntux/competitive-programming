package main

import (
	"testing"
)

type SubrectangleQueries struct {
	values [][]int
}

func Constructor(rectangle [][]int) SubrectangleQueries {
	return SubrectangleQueries{rectangle}
}

func (q *SubrectangleQueries) UpdateSubrectangle(row1 int, col1 int, row2 int, col2 int, newValue int) {
	for i := row1; i <= row2; i++ {
		for j := col1; j <= col2; j++ {
			q.values[i][j] = newValue
		}
	}
}

func (q *SubrectangleQueries) GetValue(row int, col int) int {
	return q.values[row][col]
}

// https://leetcode.com/problems/subrectangle-queries/

func TestSubrectangleQueries1(t *testing.T) {
	queries := Constructor([][]int{
		{1, 2, 1},
		{4, 3, 4},
		{3, 2, 1},
		{1, 1, 1},
	})

	if got := queries.GetValue(0, 2); got != 1 {
		t.Errorf("GetValue() = %v, want %v", got, 1)
	}
	queries.UpdateSubrectangle(0, 0, 3, 2, 5)
	if got := queries.GetValue(0, 2); got != 5 {
		t.Errorf("GetValue() = %v, want %v", got, 5)
	}
	if got := queries.GetValue(3, 1); got != 5 {
		t.Errorf("GetValue() = %v, want %v", got, 5)
	}
	queries.UpdateSubrectangle(3, 0, 3, 2, 10)
	if got := queries.GetValue(3, 1); got != 10 {
		t.Errorf("GetValue() = %v, want %v", got, 10)
	}
	if got := queries.GetValue(0, 2); got != 5 {
		t.Errorf("GetValue() = %v, want %v", got, 5)
	}
}

func TestSubrectangleQueries2(t *testing.T) {
	queries := Constructor([][]int{
		{1, 1, 1},
		{2, 2, 2},
		{3, 3, 3},
	})

	if got := queries.GetValue(0, 0); got != 1 {
		t.Errorf("GetValue() = %v, want %v", got, 1)
	}
	queries.UpdateSubrectangle(0, 0, 2, 2, 100)
	if got := queries.GetValue(0, 0); got != 100 {
		t.Errorf("GetValue() = %v, want %v", got, 100)
	}
	if got := queries.GetValue(2, 2); got != 100 {
		t.Errorf("GetValue() = %v, want %v", got, 100)
	}
	queries.UpdateSubrectangle(1, 1, 2, 2, 20)
	if got := queries.GetValue(2, 2); got != 20 {
		t.Errorf("GetValue() = %v, want %v", got, 20)
	}
}
