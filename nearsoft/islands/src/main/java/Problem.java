import java.util.LinkedList;
import java.util.List;
import java.util.Objects;
import java.util.stream.Collectors;

class Problem {

    int islands(int[][] matrix) {
        if (matrix.length == 0) {
            return 0;
        }

        final boolean[][] visited = new boolean[matrix.length][matrix[0].length];
        final List<List<Point>> islands = new LinkedList<>();

        for (int h = 0; h < matrix.length; h++) {
            for (int w = 0; w < matrix[0].length; w++) {
                if (visited[h][w]) {
                    continue;
                }

                if (matrix[h][w] == 1) {
                    List<Point> island = new LinkedList<>();
                    addNeighbours(island, visited, matrix, h, w);
                    islands.add(island);
                } else {
                    visited[h][w] = true;
                }

            }
        }

        System.out.println("******");
        islands.stream().map(island -> island.stream().sorted().map(String::valueOf).collect(Collectors.joining())).forEach(System.out::println);
        return islands.size();
    }

    private void addNeighbours(List<Point> island, boolean[][] visited, int[][] matrix, int h, int w) {
        if (h < 0 || h >= matrix.length || w < 0 || w >= matrix[0].length //out of range
                || visited[h][w]) {
            return;
        }
        visited[h][w] = true;
        if (matrix[h][w] == 1) {
            island.add(new Point(h, w));
            addNeighbours(island, visited, matrix, h + 1, w);
            addNeighbours(island, visited, matrix, h - 1, w);
            addNeighbours(island, visited, matrix, h, w + 1);
            addNeighbours(island, visited, matrix, h, w - 1);
        }
    }

}

class Point implements Comparable<Point> {
    private final int _x;
    private final int _y;

    Point(int x, int y) {
        _x = x;
        _y = y;
    }

    @Override
    public String toString() {
        return "{" + _x + ',' + _y + '}';
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        Point point = (Point) o;
        return _x == point._x &&
                _y == point._y;
    }

    @Override
    public int hashCode() {
        return Objects.hash(_x, _y);
    }

    @Override
    public int compareTo(Point o) {
        final int xCompare = Integer.compare(_x, o._x);
        if (xCompare != 0) {
            return xCompare;
        }
        return Integer.compare(_y, o._y);
    }
}