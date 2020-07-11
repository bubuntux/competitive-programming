import org.junit.jupiter.api.extension.ExtensionContext;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.ArgumentsProvider;
import org.junit.jupiter.params.provider.ArgumentsSource;

import java.util.stream.Stream;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.params.provider.Arguments.arguments;

class SolutionTest { // https://leetcode.com/problems/find-the-town-judge/

    private static final class Scenarios implements ArgumentsProvider {
        public Stream<? extends Arguments> provideArguments(ExtensionContext context) {
            return Stream.of(
                    arguments(2, new int[][]{{1, 2}}, 2),
                    arguments(3, new int[][]{{1, 3}, {2, 3}}, 3),
                    arguments(3, new int[][]{{1, 3}, {2, 3}, {3, 1}}, -1),
                    arguments(3, new int[][]{{1, 2}, {2, 3}}, -1),
                    arguments(4, new int[][]{{1, 3}, {1, 4}, {2, 3}, {2, 4}, {4, 3}}, 3)
            );
        }
    }


    @ParameterizedTest
    @ArgumentsSource(Scenarios.class)
    void findJudge(int n, int[][] trust, int expect) {
        assertEquals(new Solution().findJudge(n, trust), expect);
    }

}