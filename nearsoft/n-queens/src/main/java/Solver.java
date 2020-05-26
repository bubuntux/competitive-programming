import java.util.Collection;
import java.util.Collections;

class Solver {

    public Collection<Board> solve(int N) {
        
        return Collections.emptyList();
    }

}

class Board {
    final Boolean[][] queens;

    Board(int N) {
        queens = new Boolean[N][N];
    }
}