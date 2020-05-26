import java.util.HashMap;
import java.util.Map;
import java.util.Objects;
import java.util.Scanner;

class Player {

    public static void main(String args[]) {
        final Scanner in = new Scanner(System.in);

        boolean isBoostAvailable = true;

        Map<Point, Integer> largestDistancePerCheckpoint = new HashMap<>();
        Point firstCheckpoint = null;
        boolean visitAllCheckpoints = false;
        Point boostCandidate = null;

        // game loop
        while (true) {
            final Point currentLocation = new Point(in.nextInt(), in.nextInt());
            final Point nextCheckpoint = new Point(in.nextInt(), in.nextInt());

            final int nextCheckpointDist = in.nextInt(); // distance to the next checkpoint
            final int nextCheckpointAngle = in.nextInt(); // angle between your pod orientation and the direction of the next checkpoint

            final Point opponentLocation = new Point(in.nextInt(), in.nextInt());

            if (!visitAllCheckpoints) {
                Integer largestDistance = largestDistancePerCheckpoint.get(nextCheckpoint);
                if (largestDistance == null) {
                    largestDistancePerCheckpoint.put(nextCheckpoint, nextCheckpointDist);
                } else {
                    if (firstCheckpoint == null) {
                        firstCheckpoint = nextCheckpoint;
                    }
                    if (largestDistancePerCheckpoint.size() > 1 && nextCheckpoint.equals(firstCheckpoint)) {
                        boostCandidate = largestDistancePerCheckpoint.entrySet().stream()
                                .max((o1, o2) -> Integer.compare(o1.getValue(), o2.getValue())).get().getKey();
                        visitAllCheckpoints = true;
                    }
                }
            }

            int absAngle = Math.abs(nextCheckpointAngle);
            if (visitAllCheckpoints && isBoostAvailable && absAngle < 45 && nextCheckpoint.equals(boostCandidate)) {
                System.out.println(nextCheckpoint + " BOOST");
                isBoostAvailable = false;
                continue;
            }

            int thrust = Math.abs(100 - absAngle);
            if (nextCheckpointDist < 2000) {
                thrust -= nextCheckpointDist / 20;
                if (thrust < 0) {
                    thrust = 0;
                }
            }
            System.err.println(nextCheckpointAngle + " " + nextCheckpointDist);
            System.out.println(nextCheckpoint + " " + thrust);
        }
    }


    private static class Point {
        final int x;
        final int y;

        private Point(int x, int y) {
            this.x = x;
            this.y = y;
        }

        @Override
        public boolean equals(Object o) {
            if (this == o) return true;
            if (o == null || getClass() != o.getClass()) return false;
            Point point = (Point) o;
            return x == point.x &&
                    y == point.y;
        }

        @Override
        public int hashCode() {
            return Objects.hash(x, y);
        }

        @Override
        public String toString() {
            return x + " " + y;
        }
    }
}