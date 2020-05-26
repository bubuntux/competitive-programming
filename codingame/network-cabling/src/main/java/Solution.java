import java.util.*;
import java.util.stream.IntStream;

class Solution {

    public static void main(String args[]) {
        final Scanner in = new Scanner(System.in);
        System.err.println("Inputs *************");

        final int N = in.nextInt();
        System.err.println(N);

        final Data data = IntStream.range(0, N).mapToObj(ignore -> new Point(in.nextInt(), in.nextInt()))
                .peek(System.err::println).collect(Data::new, Data::accumulate, Data::combine);

        System.out.print(data.minLength());
    }

    private static final class Point {
        private final long _x;
        private final long _y;

        private Point(long x, long y) {
            _x = x;
            _y = y;
        }

        @Override
        public String toString() {
            return _x + " " + _y;
        }
    }

    private static final class Data {

        private long _maxX = Long.MIN_VALUE;
        private long _minX = Long.MAX_VALUE;
        private final List<Long> _yList = new ArrayList<>();
        private final SortedSet<Long> _ySet = new TreeSet<>();

        void accumulate(Point point) {
            _maxX = Math.max(_maxX, point._x);
            _minX = Math.min(_minX, point._x);

            _yList.add(point._y);
            _ySet.add(point._y);
        }

        void combine(Data other) {
            _maxX = Math.max(_maxX, other._maxX);
            _minX = Math.min(_minX, other._minX);

            _yList.addAll(other._yList);
            _ySet.addAll(other._ySet);
        }

        long minLength() {
            System.err.println("Otuputs *************");
            final int medianIndex = _ySet.size() / 2;
            System.err.print("ySet: ");
            final long median = _ySet.stream().peek(y -> System.err.print(y + " ")).skip(medianIndex).findFirst().orElseThrow(IllegalStateException::new);
            System.err.println("");
            final long ySum = _yList.stream().mapToLong(y -> Math.abs(y - median)).sum();
            final long xLenght = _maxX - _minX;
            System.err.println("median: " + median + " ysum: " + ySum + " maxX: " + _maxX + " minX: " + _minX + " xLenght: " + xLenght);
            return xLenght + ySum;
        }


    }
}