import java.util.*;

class Solution {

    public static void main(String args[]) {
        final Scanner in = new Scanner(System.in);

        System.err.println("INPUT ******");

        final int n = in.nextInt(); // the number of relationships of influence
        System.err.println(n);

        final Map<Integer, Node> nodes = new HashMap<>();
        for (int i = 0; i < n; i++) {
            final int x = in.nextInt(); // a relationship of influence between two people (x influences y)
            final int y = in.nextInt();
            System.err.println(x + " " + y);

            Node nodeX = nodes.get(x);
            if (nodeX == null) {
                nodeX = new Node();
                nodes.put(x, nodeX);
            }

            Node nodeY = nodes.get(y);
            if (nodeY == null) {
                nodeY = new Node();
                nodes.put(y, nodeY);
            }

            nodeX._links.add(nodeY);
        }
        System.err.println("SEARCH ******");

        final int longestPath = nodes.values().stream().mapToInt(Node::getLongestPath).max().orElse(0);

        System.err.println("OUTPUT ******");

        System.out.println(longestPath);
    }

    private final static class Node {

        private final Collection<Node> _links;
        private int _longestPath;

        private Node() {
            _links = new LinkedList<>();
            _longestPath = 0;
        }

        public int getLongestPath() {
            if (_longestPath == 0) {
                _longestPath = 1 + _links.stream().mapToInt(Node::getLongestPath).max().orElse(0);
            }
            return _longestPath;
        }

    }

}