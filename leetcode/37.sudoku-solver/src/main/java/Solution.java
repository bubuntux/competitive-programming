import java.util.*;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public class Solution {

    public void solveSudoku(char[][] board) {
        var state = new State(board);
        char[][] result = solveState(state);
        if (result != null) {
            board = result;
        }
    }

    private char[][] solveState(State state) {
        for (Point point : state) {
            Set<Integer> possibleValues = state.getPossibleValues(point);
            for (Integer possibleValue : possibleValues) {
                State newState = state.copy();
                newState.modify(point, possibleValue);
                if (!newState.isValid()) {
                    continue;
                }
                if (newState.isSolved()) {
                    return newState.board;
                }
                return solveState(newState);
            }
        }
        return null;
    }

    static class State implements Iterable<Point> {

        private final char[][] board;
        private final Set<Integer>[][] possibleValues = new Set[9][9];
        private final TreeMap<Integer, Set<Point>> possibleValuesSizes = new TreeMap<>();
        private boolean valid = true;

        private State(State state) {
            board = Arrays.copyOf(state.board, state.board.length);

            for (int y = 0; y < state.possibleValues.length; y++) {
                for (int x = 0; x < state.possibleValues[y].length; x++) {
                    if (state.possibleValues[y][x] != null) {
                        possibleValues[y][x] = new HashSet<>(state.possibleValues[y][x]);
                    }
                }
            }
            state.possibleValuesSizes.forEach((key, value) -> possibleValuesSizes.put(key, new HashSet<>(value)));
            valid = state.valid;
        }

        State(char[][] board) {
            this.board = board;
            init();
        }

        void init() {
            Set<Integer> initializer = IntStream.range(0, 10).mapToObj(Integer.class::cast).collect(Collectors.toSet());

            var colRemains = new ArrayList<Set<Integer>>(9);
            var rowRemains = new ArrayList<Set<Integer>>(9);
            var subRemains = new ArrayList<Set<Integer>>(9);

            for (int i = 0; i < 9; i++) {
                colRemains.add(new HashSet<>(initializer));
                rowRemains.add(new HashSet<>(initializer));
                subRemains.add(new HashSet<>(initializer));
            }

            for (int y = 0; y < board.length; y++) {
                for (int x = 0; x < board[y].length; x++) {
                    var cell = board[y][x];
                    if (cell < '0' || cell > '9') {
                        continue;
                    }
                    var cellValue = cell - '0';
                    colRemains.get(x).remove(cellValue);
                    rowRemains.get(y).remove(cellValue);
                    subRemains.get(coordinatesToSubRegion(x, y)).remove(cellValue);
                }
            }

            for (int y = 0; y < 9; y++) {
                for (int x = 0; x < 9; x++) {
                    var cell = board[y][x];
                    if (cell >= '0' && cell <= '9') {
                        continue;
                    }
                    Set<Integer> possibleValueCell = new HashSet<>(subRemains.get(coordinatesToSubRegion(x, y)));
                    possibleValueCell.retainAll(colRemains.get(x));
                    possibleValueCell.retainAll(rowRemains.get(y));
                    possibleValues[y][x] = possibleValueCell;
                    possibleValuesSizes.computeIfAbsent(possibleValueCell.size(), ignored -> new HashSet<>()).add(new Point(x, y));
                }
            }

        }

        int coordinatesToSubRegion(int x, int y) {
            int column = x / 3;
            int row = y / 3;
            return column + (3 * row);
        }

        Iterable<Point> pointsInSubRegion(int subRegion) {
            switch (subRegion) {
                case 0:
                    return Arrays.asList(
                            new Point(0, 0), new Point(1, 0), new Point(2, 0),
                            new Point(0, 1), new Point(1, 1), new Point(2, 1),
                            new Point(0, 2), new Point(1, 2), new Point(2, 2)
                    );
                case 1:
                    return Arrays.asList(
                            new Point(3, 0), new Point(4, 0), new Point(5, 0),
                            new Point(3, 1), new Point(4, 1), new Point(5, 1),
                            new Point(3, 2), new Point(4, 2), new Point(5, 2)
                    );
                case 2:
                    return Arrays.asList(
                            new Point(6, 0), new Point(7, 0), new Point(8, 0),
                            new Point(6, 1), new Point(7, 1), new Point(8, 1),
                            new Point(6, 2), new Point(7, 2), new Point(8, 2)
                    );
                case 3:
                    return Arrays.asList(
                            new Point(0, 3), new Point(1, 3), new Point(2, 3),
                            new Point(0, 4), new Point(1, 4), new Point(2, 4),
                            new Point(0, 5), new Point(1, 5), new Point(2, 5)
                    );
                case 4:
                    return Arrays.asList(
                            new Point(3, 3), new Point(4, 3), new Point(5, 3),
                            new Point(3, 4), new Point(4, 4), new Point(5, 4),
                            new Point(3, 5), new Point(4, 5), new Point(5, 5)
                    );
                case 5:
                    return Arrays.asList(
                            new Point(6, 3), new Point(7, 3), new Point(8, 3),
                            new Point(6, 4), new Point(7, 4), new Point(8, 4),
                            new Point(6, 5), new Point(7, 5), new Point(8, 5)
                    );
                case 6:
                    return Arrays.asList(
                            new Point(0, 6), new Point(1, 6), new Point(2, 6),
                            new Point(0, 7), new Point(1, 7), new Point(2, 7),
                            new Point(0, 8), new Point(1, 8), new Point(2, 8)
                    );
                case 7:
                    return Arrays.asList(
                            new Point(3, 6), new Point(4, 6), new Point(5, 6),
                            new Point(3, 7), new Point(4, 7), new Point(5, 7),
                            new Point(3, 8), new Point(4, 8), new Point(5, 8)
                    );
                case 8:
                    return Arrays.asList(
                            new Point(6, 6), new Point(7, 6), new Point(8, 6),
                            new Point(6, 7), new Point(7, 7), new Point(8, 7),
                            new Point(6, 8), new Point(7, 8), new Point(8, 8)
                    );
            }
            return Collections.emptyList();
        }

        void modify(Point point, int newValue) {
            board[point.y][point.x] = (char) (newValue + '0');

            int size = possibleValues[point.y][point.x].size();
            Set<Point> points = possibleValuesSizes.get(size);
            points.remove(point);
            if (points.isEmpty()) {
                possibleValuesSizes.remove(size);

            }

            possibleValues[point.y][point.x] = null;
            for (Set<Integer>[] possibleValue : possibleValues) {
                if (possibleValue[point.x] != null) {
                    possibleValue[point.x].remove(newValue);
                    if (possibleValue[point.x].isEmpty()) {
                        valid = false;
                        return;
                    }
                }
            }

            for (int x = 0; x < possibleValues[point.y].length; x++) {
                if (possibleValues[point.y][x] != null) {
                    possibleValues[point.y][x].remove(newValue);
                    if (possibleValues[point.y][x].isEmpty()) {
                        valid = false;
                        return;
                    }
                }
            }

            int subRegion = coordinatesToSubRegion(point.x, point.y);
            for (Point subRegionPoint : pointsInSubRegion(subRegion)) {
                if (possibleValues[subRegionPoint.y][subRegionPoint.x] != null) {
                    possibleValues[subRegionPoint.y][subRegionPoint.x].remove(newValue);
                    if (possibleValues[subRegionPoint.y][subRegionPoint.x].isEmpty()) {
                        valid = false;
                        return;
                    }
                }
            }
        }

        boolean isValid() {
            return valid;
        }

        boolean isSolved() {
            return valid && possibleValuesSizes.isEmpty();
        }

        Set<Integer> getPossibleValues(Point point) {
            return possibleValues[point.y][point.x];
        }

        State copy() {
            return new State(this);
        }

        @Override
        public Iterator<Point> iterator() {
            return possibleValuesSizes.values().stream().flatMap(Collection::stream).iterator();
        }

        @Override
        public String toString() {
            StringBuilder sb = new StringBuilder();
            for (int y = 0; y < board.length; y++) {
                if (y % 3 == 0) {
                    sb.append("------------\n");
                }
                for (int x = 0; x < board[y].length; x++) {
                    if (x % 3 == 0) {
                        sb.append('|');
                    }
                    sb.append(board[y][x]);
                }
                sb.append('\n');
            }

            return "State{ Board :\n" + sb + "\n  }";
        }
    }

    static class Point {
        private final int y;
        private final int x;

        Point(int x, int y) {
            this.x = x;
            this.y = y;
        }

        public int getX() {
            return x;
        }

        public int getY() {
            return y;
        }

        @Override
        public boolean equals(Object o) {
            if (this == o) return true;
            if (o == null || getClass() != o.getClass()) return false;
            Point point = (Point) o;
            return y == point.y &&
                    x == point.x;
        }

        @Override
        public int hashCode() {
            return Objects.hash(y, x);
        }
    }
}
