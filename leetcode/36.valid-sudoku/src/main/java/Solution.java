public class Solution {

    public boolean isValidSudoku(char[][] board) {
        if (board == null || board.length != 9 || board[0].length != 9) {
            return false;
        }
        return hasValidRows(board) && hasValidColumns(board) && hasValidSubBoxes(board);
    }

    private boolean hasValidSubBoxes(char[][] board) {
        for (int y = 0; y < board.length; y += 3) {
            for (int x = 0; x < board[y].length; x += 3) {
                var visited = new boolean[10];
                for (int i = x; i < x + 3; i++) {
                    for (int j = y; j < y + 3; j++) {
                        var cell = board[j][i];
                        if (!Character.isDigit(cell)) {
                            continue;
                        }
                        var cellValue = Character.getNumericValue(cell) - 1;
                        if (visited[cellValue]) {
                            return false;
                        }
                        visited[cellValue] = true;
                    }
                }
            }
        }
        return true;
    }

    private boolean hasValidRows(char[][] board) {
        for (char[] row : board) {
            var visited = new boolean[10];
            for (char cell : row) {
                if (!Character.isDigit(cell)) {
                    continue;
                }
                var cellValue = Character.getNumericValue(cell) - 1;
                if (visited[cellValue]) {
                    return false;
                }
                visited[cellValue] = true;
            }
        }
        return true;
    }

    private boolean hasValidColumns(char[][] board) {
        for (int i = 0; i < board.length; i++) {
            var visited = new boolean[10];
            for (char[] row : board) {
                var cell = row[i];
                if (!Character.isDigit(cell)) {
                    continue;
                }
                var cellValue = Character.getNumericValue(cell) - 1;
                if (visited[cellValue]) {
                    return false;
                }
                visited[cellValue] = true;
            }
        }
        return true;
    }


}
