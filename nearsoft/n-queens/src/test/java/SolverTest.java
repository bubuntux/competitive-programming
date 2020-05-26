import org.junit.Test;

import java.util.Arrays;
import java.util.Collection;

import static org.junit.Assert.assertEquals;

public class SolverTest {

    private final Solver solver = new Solver();

    @Test
    public void testSolutions() throws Exception { //TODO test time?
        test(1, 1);
        test(2, 0);
        test(3, 0);
        test(4, 2);
        test(5, 10);
        test(6, 4);
        test(7, 40);
        test(8, 92);
        test(9, 352);
        test(10, 724);
        test(11, 2680);
        test(12, 14200);
        test(13, 73712);
        test(14, 365596);
        test(15, 2279184);
    }

    private void test(final int N, final int solutions) throws Exception {
        Collection<Board> boards = solver.solve(N);
        assertEquals(solutions, boards.size());
        boards.forEach(board -> //
                assertEquals(N, Arrays.stream(board.queens).flatMap(Arrays::stream).filter(queen -> queen).count()));
    }
}