import org.junit.jupiter.api.Test;

import java.util.List;
import java.util.stream.Collector;
import java.util.stream.Collectors;

import static org.junit.jupiter.api.Assertions.assertEquals;

class SolutionTest {

    @Test
    void test() {
        Solution solution = new Solution();

        assertEquals("[[5,4,11,2],[5,8,4,5]]", print(solution.pathSum(tree1(), 22)));
        assertEquals("[]", print(solution.pathSum(null, 1)));
        assertEquals("[]", print(solution.pathSum(new TreeNode(), 1)));
        assertEquals("[[1,-2,1,-1]]", print(solution.pathSum(tree2(), -1)));
    }

    private TreeNode tree1() {
        TreeNode left2 = new TreeNode(11, new TreeNode(7), new TreeNode(2));
        TreeNode left1 = new TreeNode(4, left2, null);

        TreeNode right2 = new TreeNode(4, new TreeNode(5), new TreeNode(1));
        TreeNode right1 = new TreeNode(8, new TreeNode(13), right2);

        return new TreeNode(5, left1, right1);
    }

    private TreeNode tree2() {
        TreeNode left2 = new TreeNode(1, new TreeNode(-1), null);
        TreeNode left1 = new TreeNode(-2, left2, new TreeNode(3));

        TreeNode right1 = new TreeNode(-3, new TreeNode(-2), null);

        return new TreeNode(1, left1, right1);
    }


    private String print(List<List<Integer>> paths) {
        if (paths == null) return null;
        Collector<CharSequence, ?, String> joiner = Collectors.joining(",", "[", "]");
        return paths.stream().
                map(path -> path.stream().map(String::valueOf).collect(joiner)).
                collect(joiner);
    }
}