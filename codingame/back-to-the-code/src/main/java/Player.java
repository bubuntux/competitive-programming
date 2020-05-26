import java.util.Scanner;

class Player {

    private static final int ROWS = 20;
    private static final int COLUMNS = 35;

    public static void main(String args[]) {
        final int[][] maze = new int[ROWS][COLUMNS];
        final Scanner in = new Scanner(System.in);
        final int opponentCount = in.nextInt();
        final Point[] opponents = new Point[opponentCount];
        // game loop
        while (true) {
            final int gameRound = in.nextInt();
            final int x = in.nextInt(); // Your x position
            final int y = in.nextInt(); // Your y position
            final Point myPosition = new Point(x, y);
            final int backInTimeLeft = in.nextInt(); // Remaining back in time
            for (int i = 0; i < opponentCount; i++) {
                int opponentX = in.nextInt(); // X position of the opponent
                int opponentY = in.nextInt(); // Y position of the opponent
                opponents[i] = new Point(opponentX, opponentY);
                int opponentBackInTimeLeft = in.nextInt(); // Remaining back in time of the opponent
            }
            for (int i = 0; i < 20; i++) {
                String line = in.next(); // One line of the map ('.' = free, '0' = you, otherwise the id of the opponent)
                for (int j = 0; j < line.length(); j++) {
                    char character = line.charAt(j);
                    if (Character.isDigit(character)) {
                        maze[i][j] = Character.getNumericValue(character);
                    } else {
                        maze[i][j] = -1;
                    }
                }
            }

            // Write an action using System.out.println()
            // To debug: System.err.println("Debug messages...");


            // action: "x y" to move or "BACK rounds" to go back in time
            System.out.println("17 10");
        }
    }

    private static final class Point {
        private int x;
        private int y;

        public Point(int x, int y) {
            this.x = x;
            this.y = y;
        }
    }
}