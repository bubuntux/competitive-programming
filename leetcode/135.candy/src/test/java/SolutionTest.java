import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

class SolutionTest {

    @Test
    void tests() {
        Solution s = new Solution();

        assertEquals(5, s.candy(new int[]{1, 0, 2}));
        assertEquals(4, s.candy(new int[]{1, 2, 2}));
        assertEquals(7, s.candy(new int[]{1, 3, 2, 2, 1}));
    }
}