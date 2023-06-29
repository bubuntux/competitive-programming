import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

class SolutionTest {

    @Test
    void solution() {
        assertEquals(1, Solution.solution("1211", 10));
        assertEquals(3, Solution.solution("210022", 3));
    }

    @Test
    void reverse() {
        char[] chars = {'1', '2', '3'};
        Solution.reverse(chars);
        assertEquals("321", new String(chars));
    }
}