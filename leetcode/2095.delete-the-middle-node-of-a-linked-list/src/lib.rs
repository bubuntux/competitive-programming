#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = &(head.clone());
        let mut slow = &mut head;

        while fast.is_some() && fast.as_ref()?.next.is_some() {
            fast = &(fast.as_ref()?.next.as_ref()?.next);
            slow = &mut (slow.as_mut()?.next);
        }

        *slow = (*slow).as_mut()?.next.take();

        head
    }
}

// https://leetcode.com/problems/delete-the-middle-node-of-a-linked-list/
struct Solution;
