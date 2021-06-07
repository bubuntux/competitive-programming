class Solution {
    public ListNode addTwoNumbers(ListNode l, ListNode r) {
        return add(l, r, false);
    }

    private ListNode add(ListNode l, ListNode r, boolean flag) {
        if (l == null && r == null) {
            return flag ? new ListNode(1) : null;
        }
        ListNode node = new ListNode();
        int val = (l == null ? 0 : l.val) + (r == null ? 0 : r.val);
        if (flag) {
            val++;
            flag = false;
        }
        if (val >= 10) {
            val -= 10;
            flag = true;
        }
        node.val = val;
        node.next = add(l == null ? null : l.next, r == null ? null : r.next, flag);
        return node;
    }

}

///
class Tester {
    private final static Solution solution = new Solution();

    public static void main(String... args) {
        test("7,0,8", new int[]{2, 4, 3}, new int[]{5, 6, 4});
        test("0", new int[]{0}, new int[]{0});
        test("8,9,9,9,0,0,0,1", new int[]{9, 9, 9, 9, 9, 9, 9}, new int[]{9, 9, 9, 9});
    }

    private static void test(String expect, int[] a, int[] b) {
        ListNode result = solution.addTwoNumbers(node(a), node(b));
        System.out.println('[' + expect + "] = " + toString(result));
    }

    private static ListNode node(int... values) {
        ListNode root = new ListNode();

        ListNode node = root;
        ListNode prev = null;
        for (int i : values) {
            if (node == null) {
                node = new ListNode();
                prev.next = node;
            }
            node.val = i;
            prev = node;
            node = null;
        }

        return root;
    }

    private static String toString(ListNode root) {
        StringBuilder sb = new StringBuilder("[");
        ListNode node = root;
        while (node != null) {
            if (sb.length() > 1) {
                sb.append(',');
            }
            sb.append(node.val);
            node = node.next;
        }
        sb.append(']');
        return sb.toString();
    }
}

class ListNode {
    int val;
    ListNode next;

    ListNode() {}

    ListNode(int val) { this.val = val; }

    ListNode(int val, ListNode next) {
        this.val = val;
        this.next = next;
    }
}