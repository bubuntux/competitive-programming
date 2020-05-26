import java.util.Scanner;

class Solution {

    public static void main(String args[]) {
        final Scanner in = new Scanner(System.in);
        final int N = in.nextInt();
        final Node main = new Node();
        for (int i = 0; i < N; i++) {
            final String telephone = in.next();
            Node currentNode = main;
            for (int j = 0; j < telephone.length(); j++) {
                int number = Character.getNumericValue(telephone.charAt(j));
                currentNode = currentNode.add(number);
            }

        }
        System.out.println(main.sum());
    }

    private static class Node {
        Node[] nodes = new Node[10];

        Node add(int number) {
            Node node = nodes[number];
            if (node == null) {
                node = new Node();
                nodes[number] = node;
            }
            return node;
        }

        int sum() {
            int count = 0;
            for (Node node : nodes) {
                if (node != null) {
                    count++;
                    count += node.sum();
                }
            }
            return count;
        }
    }
}