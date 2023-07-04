import org.junit.jupiter.api.Test;

import java.util.Arrays;
import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;

class SolutionTest {

    private final Solution.Monkey[] monkeys = {
            new Solution.Monkey(Arrays.asList(79, 98), old -> old * 19, 23, 2, 3),
            new Solution.Monkey(Arrays.asList(54, 65, 75, 74), old -> old + 6, 19, 2, 0),
            new Solution.Monkey(Arrays.asList(79, 60, 97), old -> old * old, 13, 1, 3),
            new Solution.Monkey(List.of(74), old -> old + 3, 17, 0, 1)
    };

   // @Test
    void test20Round3CommonDivider() {
        assertEquals("""
                        [0]=101
                        [1]=95
                        [2]=7
                        [3]=105
                        Result=10605
                        """,
                Solution.calculateMonkeyBusiness(20, monkeys));
    }

    @Test
    void test1Round() {
        assertEquals("""
                        [0]=2
                        [1]=4
                        [2]=3
                        [3]=6
                        Result=24
                        """,
                Solution.calculateMonkeyBusiness(1, monkeys));
    }

    @Test
    void test20Round() {
        assertEquals("""
                        [0]=99
                        [1]=97
                        [2]=8
                        [3]=103
                        Result=10197
                        """,
                Solution.calculateMonkeyBusiness(20, monkeys));
    }

    @Test
    void test1000Round() {
        assertEquals("""
                        [0]=5204
                        [1]=4792
                        [2]=199
                        [3]=5192
                        Result=27019168
                        """,
                Solution.calculateMonkeyBusiness(1000, monkeys));
    }

    @Test
    void test4000Round() {
        assertEquals("""
                        [0]=20858
                        [1]=19138
                        [2]=780
                        [3]=20797
                        Result=433783826
                        """,
                Solution.calculateMonkeyBusiness(4000, monkeys));
    }


    @Test
    void test8000Round() {
        assertEquals("""
                        [0]=41728
                        [1]=38268
                        [2]=1553
                        [3]=41606
                        Result=1736135168
                        """,
                Solution.calculateMonkeyBusiness(8000, monkeys));
    }

    @Test
    void test100000Round() {
        assertEquals("""
                        [0]=52166
                        [1]=47830
                        [2]=1938
                        [3]=52013
                        Result=2713310158
                        """,
                Solution.calculateMonkeyBusiness(10_000, monkeys));
    }
}