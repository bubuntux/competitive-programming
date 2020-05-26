import org.junit.Test;

import static org.junit.Assert.assertEquals;

public class ProblemTest {

    @Test
    public void isLands() throws Exception {
        Problem p = new Problem();
        assertEquals(1, p.islands(new int[][]{
                {1, 0, 0},
                {1, 1, 1},
                {1, 0, 1}
        }));
        assertEquals(5, p.islands(new int[][]{
                {1, 0, 1, 1, 0},
                {0, 1, 0, 1, 0},
                {1, 0, 0, 0, 0},
                {0, 0, 1, 1, 1}
        }));
        assertEquals(5, p.islands(new int[][]{
                {1, 0, 1},
                {0, 1, 0},
                {1, 0, 1}
        }));
        assertEquals(1, p.islands(new int[][]{
                {0, 1, 1},
                {0, 1, 0},
                {1, 1, 0}
        }));
        assertEquals(2, p.islands(new int[][]{
                {0, 0, 1, 1, 0},
                {1, 1, 0, 1, 0},
                {1, 1, 0, 1, 0},
                {0, 0, 1, 1, 1},
                {0, 1, 1, 0, 0}
        }));

    }
}