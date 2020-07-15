import org.junit.jupiter.api.Test;

import java.util.Arrays;
import java.util.Collections;
import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

class SolutionTest { // https://leetcode.com/problems/subsets/

    @Test
    void subsets() {
        List<List<Integer>> actual = new Solution().subsets(1, 2, 3);
        List<List<?>> expected = Arrays.asList(
                Collections.singletonList(3),
                Collections.singletonList(1),
                Collections.singletonList(2),
                Arrays.asList(1, 2, 3),
                Arrays.asList(1, 3),
                Arrays.asList(2, 3),
                Arrays.asList(1, 2),
                Collections.emptyList()
        );
        assertEquals(expected.size(), actual.size());
        assertTrue(actual.containsAll(expected));
        assertTrue(expected.containsAll(actual));
    }
}