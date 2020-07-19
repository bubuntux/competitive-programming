import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;

class SolutionTest { // https://leetcode.com/problems/sudoku-solver/

    @Test
    void solveSudoku() {
        Solution solution = new Solution();
        char[][] board = {
                {'5', '3', '.', '.', '7', '.', '.', '.', '.'},
                {'6', '.', '.', '1', '9', '5', '.', '.', '.'},
                {'.', '9', '8', '.', '.', '.', '.', '6', '.'},
                {'8', '.', '.', '.', '6', '.', '.', '.', '3'},
                {'4', '.', '.', '8', '.', '3', '.', '.', '1'},
                {'7', '.', '.', '.', '2', '.', '.', '.', '6'},
                {'.', '6', '.', '.', '.', '.', '2', '8', '.'},
                {'.', '.', '.', '4', '1', '9', '.', '.', '5'},
                {'.', '.', '.', '.', '8', '.', '.', '7', '9'}
        };
        solution.solveSudoku(board);
        char[][] expected = {
                {'5', '3', '4', '6', '7', '8', '9', '1', '2'},
                {'6', '7', '2', '1', '9', '5', '3', '4', '8'},
                {'1', '9', '8', '3', '4', '2', '5', '6', '7'},
                {'8', '5', '9', '7', '6', '1', '4', '2', '3'},
                {'4', '2', '6', '8', '5', '3', '7', '9', '1'},
                {'7', '1', '3', '9', '2', '4', '8', '5', '6'},
                {'9', '6', '1', '5', '3', '7', '2', '8', '4'},
                {'2', '8', '7', '4', '1', '9', '6', '3', '5'},
                {'3', '4', '5', '2', '8', '6', '1', '7', '9'}
        };
        assertArrayEquals(expected, board);
    }

    @Test
    void testCoordinatesToSubRegion() {
        Solution.State solution = new Solution.State(null) {
            @Override
            void init() {
            }
        };
        assertEquals(0, solution.coordinatesToSubRegion(0, 0));
        assertEquals(0, solution.coordinatesToSubRegion(1, 1));
        assertEquals(0, solution.coordinatesToSubRegion(2, 2));
        assertEquals(1, solution.coordinatesToSubRegion(3, 0));
        assertEquals(1, solution.coordinatesToSubRegion(4, 1));
        assertEquals(1, solution.coordinatesToSubRegion(5, 2));
        assertEquals(2, solution.coordinatesToSubRegion(6, 0));
        assertEquals(2, solution.coordinatesToSubRegion(7, 1));
        assertEquals(2, solution.coordinatesToSubRegion(8, 2));
        assertEquals(3, solution.coordinatesToSubRegion(0, 3));
        assertEquals(3, solution.coordinatesToSubRegion(1, 4));
        assertEquals(3, solution.coordinatesToSubRegion(2, 5));
        assertEquals(7, solution.coordinatesToSubRegion(3, 6));
        assertEquals(7, solution.coordinatesToSubRegion(4, 7));
        assertEquals(7, solution.coordinatesToSubRegion(5, 8));
    }
}