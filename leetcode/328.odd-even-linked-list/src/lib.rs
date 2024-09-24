impl Solution {
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref()?.next.is_none() {
            return head;
        }

        let mut odd = None;
        let mut odd_head = &mut odd;

        let mut even = None;
        let mut even_head = &mut even;

        let mut toggl = false;
        while let Some(mut node) = head {
            head = node.next.take();
            toggl = !toggl;
            if toggl {
                *odd_head = Some(node);
                odd_head = &mut odd_head.as_mut()?.next;
            } else {
                *even_head = Some(node);
                even_head = &mut even_head.as_mut()?.next;
            }
        }

        *odd_head = even;

        odd
    }
}

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
        assert_eq!(
            Solution::odd_even_list(into_list_node(vec![1, 2, 3, 4, 5])),
            into_list_node(vec![1, 3, 5, 2, 4])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::odd_even_list(into_list_node(vec![2, 1, 3, 5, 6, 4, 7])),
            into_list_node(vec![2, 3, 6, 7, 1, 5, 4])
        );
    }
}
