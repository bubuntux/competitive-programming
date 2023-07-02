import org.junit.jupiter.api.Test;

import java.util.Arrays;
import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;

class SolutionTest {

    private final Solution.Monkey[] monkeys = {
            new Solution.Monkey(Arrays.asList(79, 98), old -> old * 19, value -> value % 23 == 0 ? 2 : 3),
            new Solution.Monkey(Arrays.asList(54, 65, 75, 74), old -> old + 6, value -> value % 19 == 0 ? 2 : 0),
            new Solution.Monkey(Arrays.asList(79, 60, 97), old -> old * old, value -> value % 13 == 0 ? 1 : 3),
            new Solution.Monkey(List.of(74), old -> old + 3, value -> value % 17 == 0 ? 0 : 1)
    };

    @Test
    void test20Round3CommonDivider() {
        assertEquals("""
                        [0]=101
                        [1]=95
                        [2]=7
                        [3]=105
                        Result=10605
                        """
                , Solution.calculateMonkeyBusiness(20, 3, monkeys));
    }

    @Test
    void test1Round() {
        assertEquals("""
                        [0]=2
                        [1]=4
                        [2]=3
                        [3]=6
                        Result=24
                        """
                , Solution.calculateMonkeyBusiness(1, 1, monkeys));
    }

    @Test
    void test20Round() {
        assertEquals("""
                        [0]=99
                        [1]=97
                        [2]=8
                        [3]=103
                        Result=10197
                        """
                , Solution.calculateMonkeyBusiness(20, 1, monkeys));
    }

    @Test
    void test1000Round() {
        assertEquals("""
                        [0]=5204
                        [1]=4792
                        [2]=199
                        [3]=5192
                        Result=27019168
                        """
                , Solution.calculateMonkeyBusiness(1000, 1, monkeys));
    }

    @Test
    void test100000Round() {
        assertEquals("""
                        [0]=52166
                        [1]=47830
                        [2]=1938
                        [3]=52013
                        Result=2713310158
                        """
                , Solution.calculateMonkeyBusiness(1000, 1, monkeys));
    }
}