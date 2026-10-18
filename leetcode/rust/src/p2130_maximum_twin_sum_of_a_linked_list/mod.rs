use std::collections::VecDeque;

impl Solution {
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut q: VecDeque<i32> = VecDeque::new();

        let mut curr = head;
        while let Some(node) = curr {
            q.push_front(node.val);
            curr = node.next;
        }

        let mut max_sum = 0;
        while !q.is_empty() {
            max_sum = max_sum.max(q.pop_back().unwrap() + q.pop_front().unwrap());
        }

        max_sum
    }
}

// https://leetcode.com/problems/maximum-twin-sum-of-a-linked-list/

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

#[cfg(test)]
mod tests {
    use super::*;

    fn into_list_node(nums: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut tail = &mut head;
        for num in nums {
            *tail = Some(Box::new(ListNode::new(num)));
            tail = &mut tail.as_mut()?.next;
        }
        head
    }

    #[test]
    fn example_1() {
        assert_eq!(Solution::pair_sum(into_list_node(vec![5, 4, 2, 1])), 6);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::pair_sum(into_list_node(vec![4, 2, 2, 3])), 7);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::pair_sum(into_list_node(vec![1, 100000])), 100001);
    }
}
