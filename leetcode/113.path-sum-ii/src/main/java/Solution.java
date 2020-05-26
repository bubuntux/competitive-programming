import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;

public class Solution {
    
    public List<List<Integer>> pathSum(TreeNode root, int sum) {
        List<List<Integer>> paths = new LinkedList<>();
        populatePath(paths, new ArrayList<>(), root, sum);
        return paths;
    }

    private void populatePath(List<List<Integer>> paths, List<Integer> path, TreeNode node, int remain) {
        if (node == null) return;
        remain -= node.val;
        if (node.left == null && node.right == null) {
            if (remain != 0) return;
            ArrayList<Integer> validPath = new ArrayList<>(path);
            validPath.add(node.val);
            paths.add(validPath);
            return;
        }
        path.add(node.val);
        populatePath(paths, path, node.left, remain);
        populatePath(paths, path, node.right, remain);
        path.remove(path.size() - 1);
    }

}