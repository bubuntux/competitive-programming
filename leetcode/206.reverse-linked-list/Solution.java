class Solution {
    public ListNode reverseList(ListNode head) {
        if (head == null || head.next == null) return head;

        ListNode prev = null;
        ListNode current = head;
        while (current.next != null) {
            ListNode aux = current.next;
            current.next = prev;
            prev = current;
            current = aux;
        }
        current.next = prev;
        return current;
    }
}

///

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