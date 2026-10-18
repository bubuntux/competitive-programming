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

struct Solution;

impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev: Option<Box<ListNode>> = None;
        let mut curr_opt = head;

        while let Some(mut curr) = curr_opt {
            let mut next = curr.next.take();
            curr.next = prev.take();
            prev = Some(curr);
            curr_opt = next.take();
        }

        prev
    }
}
